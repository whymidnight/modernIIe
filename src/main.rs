#![no_std]
#![no_main]

extern crate alloc;

mod drivers;
mod utils;

use crate::drivers::no_std::kb::decoder::{Debounce, KeyScan, NUM_COLS, NUM_MODS, NUM_ROWS};
use crate::drivers::no_std::kb::input::A2PI_DESCRIPTOR;
use alloc::vec::Vec;
use core::borrow::{Borrow, BorrowMut};
use core::{cell::RefCell, convert::Infallible};
use critical_section::Mutex;
use drivers::no_std::kb::oracle::KbOracleReports;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use hal::gpio::bank0::{Gpio16, Gpio17, Gpio18};
use hal::gpio::{Input, Output, Pin, PullDown, PullUp, PushPull};
use hal::multicore::{Multicore, Stack};
use usbd_human_interface_device::device::consumer::{ConsumerControl, ConsumerControlConfig};
use usbd_human_interface_device::usb_class::{UsbHidClass, UsbHidClassBuilder};

use usb_device::class_prelude::*;
use usb_device::prelude::*;

use crate::drivers::no_std::kb::driver::KbDriver;
use crate::drivers::shared::kb::KeyboardDriver;
use cortex_m::prelude::_embedded_hal_timer_CountDown;
use cortex_m::singleton;
use embedded_alloc::Heap;
use frunk::{HCons, HNil, ToMut};
use fugit::{ExtU32, HertzU32, RateExtU32};
use hal::clocks::Clock;
use hal::dma::DMAExt;
use hal::uart::{DataBits, StopBits, UartConfig};
use hal::usb::UsbBus;
use panic_probe as _;
use usb_device::class::UsbClass;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use usbd_hid::hid_class::{
    HIDClass, HidClassSettings, HidCountryCode, HidProtocol, HidSubClass, ProtocolModeConfig,
};

use rp2040_hal as hal;
use rp2040_hal::pac;
use rp2040_hal::pac::interrupt;

#[cfg(feature = "probe")]
use defmt_rtt as _;
#[cfg(feature = "serial")]
use defmt_serial as _;

const SCAN_LOOP_RATE_MS: u32 = 5;
const DEBOUNCE_TICKS: u8 = 1;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[global_allocator]
static HEAP: Heap = Heap::empty();

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

static mut USB_BUS: Option<usb_device::bus::UsbBusAllocator<UsbBus>> = None;
static mut USB_DEVICE: Option<UsbDevice<'static, UsbBus>> = None;
static mut USB_HID: Option<HIDClass<'static, UsbBus>> = None;
static KEYBOARD_REPORT: Mutex<RefCell<KbOracleReports>> =
    Mutex::new(RefCell::new(KbOracleReports::Keyboard(KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: [0u8; 6],
    })));

type Pins = (
    Pin<Gpio16, Output<PushPull>>,
    Pin<Gpio17, Input<PullUp>>,
    Pin<Gpio18, Output<PushPull>>,
);
static PINS: Mutex<RefCell<Option<Pins>>> = Mutex::new(RefCell::new(None));

static mut KEY_PRESS_EVENT: [u8; 3] = [0x0; 3];

static mut A2PI_DRIVER_KB: Mutex<RefCell<Option<KbDriver>>> = Mutex::new(RefCell::new(None));

#[rp2040_hal::entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 200 * 1000;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    //
    // -- BEGIN PRELUDE --
    //

    let mut a2pi = KbDriver::init();

    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let mut sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let sys_freq = clocks.system_clock.freq().to_Hz();
    let mut mc = Multicore::new(&mut pac.PSM, &mut pac.PPB, &mut sio.fifo);
    let cores = mc.cores();
    let core1 = &mut cores[1];
    let _test = core1.spawn(unsafe { &mut CORE1_STACK.mem }, move || {
        core1_task(sys_freq)
    });

    let probe_uart = hal::uart::UartPeripheral::new(
        pac.UART0,
        (
            // UART TX (characters sent from RP2040) on pin 1 (GPIO0)
            pins.gpio0.into_mode::<hal::gpio::FunctionUart>(),
            // UART RX (characters received by RP2040) on pin 2 (GPIO1)
            pins.gpio1.into_mode::<hal::gpio::FunctionUart>(),
        ),
        &mut pac.RESETS,
    )
    .enable(
        UartConfig::new(115200.Hz(), DataBits::Eight, None, StopBits::One),
        clocks.peripheral_clock.freq(),
    )
    .unwrap();
    #[cfg(feature = "serial")]
    defmt_serial::defmt_serial(probe_uart);
    defmt::info!("booting!!");

    // -- BEGIN device INIT --

    let hal_usb_bus = hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    );

    hal_usb_bus.remote_wakeup();
    let usb_bus = UsbBusAllocator::new(hal_usb_bus);
    unsafe {
        USB_BUS = Some(usb_bus);
    }
    let hid_endpoint = HIDClass::new_with_settings(
        unsafe { USB_BUS.as_ref().unwrap() },
        A2PI_DESCRIPTOR,
        1,
        HidClassSettings {
            subclass: HidSubClass::NoSubClass,
            protocol: HidProtocol::Keyboard,
            config: ProtocolModeConfig::ForceReport,
            locale: HidCountryCode::US,
        },
    );

    unsafe {
        USB_HID = Some(hid_endpoint);
    }

    let usb_device = UsbDeviceBuilder::new(
        unsafe { USB_BUS.as_ref().unwrap() },
        UsbVidPid(0x05ac, 0x0220),
    )
    .manufacturer("Apple Inc.")
    .product("Apple Keyboard")
    // .serial_number("00123")
    .supports_remote_wakeup(true)
    .build();

    unsafe {
        USB_DEVICE = Some(usb_device);
    }

    // -- BEGIN MAIN --

    let mut tick_count_down = timer.count_down();
    tick_count_down.start(1.millis());

    let cols: &mut [&mut dyn OutputPin<Error = Infallible>; NUM_COLS] = &mut [
        &mut pins.gpio13.into_push_pull_output(), // X0
        &mut pins.gpio17.into_push_pull_output(), // X1
        &mut pins.gpio15.into_push_pull_output(), // X2
        &mut pins.gpio19.into_push_pull_output(), // X3
        &mut pins.gpio20.into_push_pull_output(), // X5
        &mut pins.gpio18.into_push_pull_output(), // X4
        &mut pins.gpio28.into_push_pull_output(), // X6
        &mut pins.gpio16.into_push_pull_output(), // X7
    ];

    let rows: &[&dyn InputPin<Error = Infallible>; NUM_ROWS] = &[
        &pins.gpio2.into_pull_down_input(),  // Y0
        &pins.gpio3.into_pull_down_input(),  // Y1
        &pins.gpio4.into_pull_down_input(),  // Y2
        &pins.gpio14.into_pull_down_input(), // Y3
        &pins.gpio8.into_pull_down_input(),  // Y3
        &pins.gpio10.into_pull_down_input(), // Y4
        &pins.gpio22.into_pull_down_input(), // Y5
        &pins.gpio27.into_pull_down_input(), // Y6
        &pins.gpio12.into_pull_down_input(), // Y8
        &pins.gpio21.into_pull_down_input(), // Y9
    ];

    let modifiers: (
        &[&dyn InputPin<Error = Infallible>],
        &[&dyn InputPin<Error = Infallible>],
    ) = (
        &[
            &pins.gpio5.into_pull_down_input(), // SW1 :: Closed Apple
            &pins.gpio7.into_pull_down_input(), // SW0 :: Open Apple
        ],
        &[
            &pins.gpio11.into_pull_up_input(), // Control
            &pins.gpio9.into_pull_up_input(),  // RESET
            &pins.gpio26.into_pull_up_input(), // Shift
        ],
    );

    let mut debounce: Debounce<NUM_MODS, NUM_ROWS, NUM_COLS> = Debounce::new(DEBOUNCE_TICKS);

    critical_section::with(|cs| {
        KEYBOARD_REPORT.replace(cs, KbOracleReports::init());
    });

    unsafe {
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    };

    loop {
        let processed_reports =
            a2pi.process_key_event(modifiers, rows, cols, &mut delay, &mut debounce);
        if let Some(reports) = processed_reports {
            // defmt::info!("!-----! {}", reports.len());
            critical_section::with(|cs| {
                for report in reports {
                    KEYBOARD_REPORT.replace(cs, report);
                    // delay.delay_ms(10);
                }
            });
        }
        delay.delay_ms(SCAN_LOOP_RATE_MS);
    }
    //
    // -- END MAIN --
    //
}

// @sauce: https://github.com/bschwind/key-ripper/blob/576e5d1b99436b6539302fa41861851fd24ff004/firmware/src/main.rs#L217
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let usb_hid = USB_HID.as_mut().unwrap();

    if usb_dev.poll(&mut [usb_hid]) {
        usb_hid.poll();
    }

    let report = critical_section::with(|cs| *KEYBOARD_REPORT.borrow_ref(cs));
    match report {
        KbOracleReports::Keyboard(k) => {
            if let Err(_err) = usb_hid.push_input(&k) {
                let _no_op = 0;
            }
        }
        KbOracleReports::Consumer(c) => loop {
            let fn_key = usb_hid.push_input(&c);
            match fn_key {
                Ok(_) => break,
                Err(UsbError::WouldBlock) => {
                    continue;
                }
                Err(e) => {
                    defmt::error!("{}", e);
                    break;
                }
            }
        },
    };

    // macOS doesn't like it when you don't pull this, apparently.
    // TODO: maybe even parse something here
    usb_hid.pull_raw_output(&mut [0; 64]).ok();

    // Wake the host if a key is pressed and the device supports
    // remote wakeup.
    if usb_dev.state() == UsbDeviceState::Suspend && usb_dev.remote_wakeup_enabled() {
        usb_dev.bus().remote_wakeup();
    }
}

fn report_is_empty(report: &KeyboardReport) -> bool {
    report.modifier != 0 || report.keycodes.iter().any(|key| *key != 0x0u8)
}

static mut CORE1_STACK: Stack<4096> = Stack::new();

fn core1_task(sys_freq: u32) -> ! {
    loop {}
    /*
    let mut pac = unsafe { pac::Peripherals::steal() };
    let core = unsafe { pac::CorePeripherals::steal() };

    let mut sio = hal::Sio::new(pac.SIO);
    /*
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    */

    let mut delay = cortex_m::delay::Delay::new(core.SYST, sys_freq);

    // let buffer: [u8; 3] = [0x0u8; 3];
    let mut buffer = Vec::new();
    loop {
        let input = sio.fifo.read();
        if let Some(word) = input {
            let byte = word;
            if byte == 512 {
                buffer.clear();
            } else {
                buffer.push(byte as u8);
            }
        };
        if buffer.len() == 3 {
            defmt::info!(
                "[KEY_PRESS]: {=[u8]:#x} ::::: {:?}",
                buffer.as_slice(),
                buffer.as_slice()
            );
            // process buffer
            let reports: Option<Vec<KeyboardReport>> = critical_section::with(|cs| unsafe {
                let mut a2pi = unsafe { A2PI_DRIVER_KB.borrow_ref_mut(cs) };
                a2pi.as_mut().unwrap().process_key_event({
                    let mut buf: [u8; 3] = [0x0u8; 3];
                    buffer.iter().enumerate().for_each(|(idx, &b)| buf[idx] = b);
                    buf
                })
            });
            critical_section::with(|cs| unsafe {
                if let Some(keyboard_reports) = reports {
                    let num_reports = keyboard_reports.len();
                    for (idx, &report) in keyboard_reports.iter().enumerate() {
                        defmt::info!("writing report... {}", report.keycodes);
                        KEYBOARD_REPORT.replace(cs, report);
                        // delay.delay_ms(10);
                        // unless a case exists where there are multiple reports to be
                        // emitted, do not incur a delay.
                        if idx != num_reports {
                            defmt::info!("delaying next report...");
                        }
                    }
                }
            });
            buffer.clear();
        }
    }
    // sio.fifo.write_blocking(CORE1_TASK_COMPLETE);
    */
}
