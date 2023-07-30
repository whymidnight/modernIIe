#! /usr/bin/env bash
echo "writing elf..."
cp $1 a2pi.elf
cp a2pi.elf ../../embedded/serial-defmt/a2pi.elf
chmod +rx ../../embedded/serial-defmt/a2pi.elf

