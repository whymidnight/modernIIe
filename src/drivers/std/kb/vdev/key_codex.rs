use std::collections::HashMap;

use enigo::Key;

use crate::drivers::kb::input::Modifiers;

#[derive(Clone)]
pub struct VdevKeyMacroSequenceEntrant {
    pub to: String,
    pub until: Option<String>,
    pub until_after: Option<String>,
}

impl VdevKeyMacroSequenceEntrant {
    // pub fn from_spec() {}
    pub fn into_vdev_key(self) -> Key {
        match self.to.as_str() {
            "CTRL" => Key::Control,
            "OPTION" => Key::Option,
            "ALT" => Key::Alt,
            "CMD" => Key::Meta,
            "META" => Key::Meta,
            "WIN" => Key::Meta,
            "TAB" => Key::Tab,
            "SHIFT" => Key::Shift,
            /* directional keys start */
            "LEFT" => Key::LeftArrow,
            "RIGHT" => Key::RightArrow,
            "e" => Key::Layout('e'),
            "h" => Key::Layout('h'),
            "l" => Key::Layout('l'),
            "UP" => Key::UpArrow,
            "DOWN" => Key::DownArrow,
            /* directional keys end */
            _ => {
                let to_char = self.to.chars();
                let c = to_char.take(1).last().unwrap();
                Key::Layout(c)
            }
        }
    }
}

/// see `${PROJECT_DIR}/rust/spec.configure_keyboard_layout.md` for shape.
pub type VdevKeyMacro = HashMap<String, VdevKeyMacroSequenceEntrant>;

#[derive(Clone)]
pub enum VdevKey {
    None(Key),
    #[allow(dead_code)]
    Remap(Key),
    Macro(VdevKeyMacro),
}

pub type VdevKeyCodex = HashMap<String, VdevKey>;
pub type VdevKeyLayerCodex = HashMap<u8, VdevKeyCodex>;

#[derive(Clone)]
pub struct VdevKeys {
    pub codex: VdevKeyCodex,
    pub layers: VdevKeyLayerCodex,
}

impl VdevKeys {
    pub fn init_codex() -> VdevKeyCodex {
        HashMap::from([
            /* ALPA-NUMERIC */
            ("a".to_string(), VdevKey::None(Key::Layout('a'))),
            ("A".to_string(), VdevKey::None(Key::Layout('A'))),
            ("b".to_string(), VdevKey::None(Key::Layout('b'))),
            ("B".to_string(), VdevKey::None(Key::Layout('B'))),
            ("c".to_string(), VdevKey::None(Key::Layout('c'))),
            ("C".to_string(), VdevKey::None(Key::Layout('C'))),
            ("d".to_string(), VdevKey::None(Key::Layout('d'))),
            ("D".to_string(), VdevKey::None(Key::Layout('D'))),
            ("e".to_string(), VdevKey::None(Key::Layout('e'))),
            ("E".to_string(), VdevKey::None(Key::Layout('E'))),
            ("f".to_string(), VdevKey::None(Key::Layout('f'))),
            ("F".to_string(), VdevKey::None(Key::Layout('F'))),
            ("g".to_string(), VdevKey::None(Key::Layout('g'))),
            ("G".to_string(), VdevKey::None(Key::Layout('G'))),
            ("h".to_string(), VdevKey::None(Key::Layout('h'))),
            ("H".to_string(), VdevKey::None(Key::Layout('H'))),
            ("i".to_string(), VdevKey::None(Key::Layout('i'))),
            ("I".to_string(), VdevKey::None(Key::Layout('I'))),
            ("j".to_string(), VdevKey::None(Key::Layout('j'))),
            ("J".to_string(), VdevKey::None(Key::Layout('J'))),
            ("k".to_string(), VdevKey::None(Key::Layout('k'))),
            ("K".to_string(), VdevKey::None(Key::Layout('K'))),
            ("l".to_string(), VdevKey::None(Key::Layout('l'))),
            ("L".to_string(), VdevKey::None(Key::Layout('L'))),
            ("m".to_string(), VdevKey::None(Key::Layout('m'))),
            ("M".to_string(), VdevKey::None(Key::Layout('M'))),
            ("n".to_string(), VdevKey::None(Key::Layout('n'))),
            ("N".to_string(), VdevKey::None(Key::Layout('N'))),
            ("o".to_string(), VdevKey::None(Key::Layout('o'))),
            ("O".to_string(), VdevKey::None(Key::Layout('O'))),
            ("p".to_string(), VdevKey::None(Key::Layout('p'))),
            ("P".to_string(), VdevKey::None(Key::Layout('P'))),
            ("q".to_string(), VdevKey::None(Key::Layout('q'))),
            ("Q".to_string(), VdevKey::None(Key::Layout('Q'))),
            ("r".to_string(), VdevKey::None(Key::Layout('r'))),
            ("R".to_string(), VdevKey::None(Key::Layout('R'))),
            ("s".to_string(), VdevKey::None(Key::Layout('s'))),
            ("S".to_string(), VdevKey::None(Key::Layout('S'))),
            ("t".to_string(), VdevKey::None(Key::Layout('t'))),
            ("T".to_string(), VdevKey::None(Key::Layout('T'))),
            ("u".to_string(), VdevKey::None(Key::Layout('u'))),
            ("U".to_string(), VdevKey::None(Key::Layout('U'))),
            ("v".to_string(), VdevKey::None(Key::Layout('v'))),
            ("V".to_string(), VdevKey::None(Key::Layout('V'))),
            ("w".to_string(), VdevKey::None(Key::Layout('w'))),
            ("W".to_string(), VdevKey::None(Key::Layout('W'))),
            ("x".to_string(), VdevKey::None(Key::Layout('x'))),
            ("X".to_string(), VdevKey::None(Key::Layout('X'))),
            ("y".to_string(), VdevKey::None(Key::Layout('y'))),
            ("Y".to_string(), VdevKey::None(Key::Layout('Y'))),
            ("z".to_string(), VdevKey::None(Key::Layout('z'))),
            ("Z".to_string(), VdevKey::None(Key::Layout('Z'))),
            /* NUMERIC */
            ("1".to_string(), VdevKey::None(Key::Layout('1'))),
            ("2".to_string(), VdevKey::None(Key::Layout('2'))),
            ("3".to_string(), VdevKey::None(Key::Layout('3'))),
            ("4".to_string(), VdevKey::None(Key::Layout('4'))),
            ("5".to_string(), VdevKey::None(Key::Layout('5'))),
            ("6".to_string(), VdevKey::None(Key::Layout('6'))),
            ("7".to_string(), VdevKey::None(Key::Layout('7'))),
            ("8".to_string(), VdevKey::None(Key::Layout('8'))),
            ("9".to_string(), VdevKey::None(Key::Layout('9'))),
            ("0".to_string(), VdevKey::None(Key::Layout('0'))),
            /* SYMBOLS */
            ("<".to_string(), VdevKey::None(Key::Layout('<'))),
            (">".to_string(), VdevKey::None(Key::Layout('>'))),
            (",".to_string(), VdevKey::None(Key::Layout(','))),
            (";".to_string(), VdevKey::None(Key::Layout(';'))),
            (".".to_string(), VdevKey::None(Key::Layout('.'))),
            (":".to_string(), VdevKey::None(Key::Layout(':'))),
            ("-".to_string(), VdevKey::None(Key::Layout('-'))),
            ("_".to_string(), VdevKey::None(Key::Layout('_'))),
            ("#".to_string(), VdevKey::None(Key::Layout('#'))),
            ("^".to_string(), VdevKey::None(Key::Layout('\''))),
            ("+".to_string(), VdevKey::None(Key::Layout('+'))),
            ("*".to_string(), VdevKey::None(Key::Layout('*'))),
            /* NUMERICAL SYMBOLS */
            ("!".to_string(), VdevKey::None(Key::Layout('!'))),
            ("\"".to_string(), VdevKey::None(Key::Layout('"'))),
            ("@".to_string(), VdevKey::None(Key::Layout('"'))),
            ("$".to_string(), VdevKey::None(Key::Layout('$'))),
            ("%".to_string(), VdevKey::None(Key::Layout('%'))),
            ("&".to_string(), VdevKey::None(Key::Layout('&'))),
            ("/".to_string(), VdevKey::None(Key::Layout('/'))),
            ("(".to_string(), VdevKey::None(Key::Layout('('))),
            (")".to_string(), VdevKey::None(Key::Layout(')'))),
            (")".to_string(), VdevKey::None(Key::Layout(')'))),
            ("=".to_string(), VdevKey::None(Key::Layout('='))),
            /* NUMERICAL SYMBOLS (cont'd) */
            ("~".to_string(), VdevKey::None(Key::Layout('ß'))),
            ("?".to_string(), VdevKey::None(Key::Layout('?'))),
            ("'".to_string(), VdevKey::None(Key::Layout('´'))),
            ("`".to_string(), VdevKey::None(Key::Layout('`'))),
            /* CONTROLS */
            ("Enter".to_string(), VdevKey::None(Key::Return)),
            ("BS".to_string(), VdevKey::None(Key::Backspace)),
            ("ESCAPE".to_string(), VdevKey::None(Key::Layout('^'))),
            ("CTRL-I".to_string(), VdevKey::None(Key::Escape)),
            ("' '".to_string(), VdevKey::None(Key::Space)),
            /* CONTROL KEY COMBOS */
            (
                "CTRL-A".to_string(),
                VdevKey::Macro(VdevKeyMacro::from([
                    (
                        "0".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "CTRL".to_string(),
                            until: None,
                            until_after: Some("1".to_string()),
                        },
                    ),
                    (
                        "1".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "a".to_string(),
                            until: None,
                            until_after: None,
                        },
                    ),
                ])),
            ),
            (
                "CTRL-H".to_string(),
                VdevKey::Macro(VdevKeyMacro::from([
                    (
                        "0".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "CTRL".to_string(),
                            until: None,
                            until_after: Some("1".to_string()),
                        },
                    ),
                    (
                        "1".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "h".to_string(),
                            until: None,
                            until_after: None,
                        },
                    ),
                ])),
            ),
            (
                "CTRL-J".to_string(),
                VdevKey::Macro(VdevKeyMacro::from([
                    (
                        "0".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "CTRL".to_string(),
                            until: None,
                            until_after: Some("1".to_string()),
                        },
                    ),
                    (
                        "1".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "j".to_string(),
                            until: None,
                            until_after: None,
                        },
                    ),
                ])),
            ),
            (
                "CTRL-K".to_string(),
                VdevKey::Macro(VdevKeyMacro::from([
                    (
                        "0".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "CTRL".to_string(),
                            until: None,
                            until_after: Some("1".to_string()),
                        },
                    ),
                    (
                        "1".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "k".to_string(),
                            until: None,
                            until_after: None,
                        },
                    ),
                ])),
            ),
            (
                "CTRL-L".to_string(),
                VdevKey::Macro(VdevKeyMacro::from([
                    (
                        "0".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "CTRL".to_string(),
                            until: None,
                            until_after: Some("1".to_string()),
                        },
                    ),
                    (
                        "1".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "l".to_string(),
                            until: None,
                            until_after: None,
                        },
                    ),
                ])),
            ),
            (
                "CTRL-D".to_string(),
                VdevKey::Macro(VdevKeyMacro::from([
                    (
                        "0".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "CTRL".to_string(),
                            until: None,
                            until_after: Some("1".to_string()),
                        },
                    ),
                    (
                        "1".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "d".to_string(),
                            until: None,
                            until_after: None,
                        },
                    ),
                ])),
            ),
            (
                "CTRL-U".to_string(),
                VdevKey::Macro(VdevKeyMacro::from([
                    (
                        "0".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "CTRL".to_string(),
                            until: None,
                            until_after: Some("1".to_string()),
                        },
                    ),
                    (
                        "1".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "u".to_string(),
                            until: None,
                            until_after: None,
                        },
                    ),
                ])),
            ),
            (
                "CTRL-R".to_string(),
                VdevKey::Macro(VdevKeyMacro::from([
                    (
                        "0".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "CTRL".to_string(),
                            until: None,
                            until_after: Some("1".to_string()),
                        },
                    ),
                    (
                        "1".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "r".to_string(),
                            until: None,
                            until_after: None,
                        },
                    ),
                ])),
            ),
            (
                "CTRL-W".to_string(),
                VdevKey::Macro(VdevKeyMacro::from([
                    (
                        "0".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "CTRL".to_string(),
                            until: None,
                            until_after: Some("1".to_string()),
                        },
                    ),
                    (
                        "1".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "w".to_string(),
                            until: None,
                            until_after: None,
                        },
                    ),
                ])),
            ),
            (
                "CTRL-E".to_string(),
                VdevKey::Macro(VdevKeyMacro::from([
                    (
                        "0".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "CTRL".to_string(),
                            until: None,
                            until_after: Some("1".to_string()),
                        },
                    ),
                    (
                        "1".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "e".to_string(),
                            until: None,
                            until_after: None,
                        },
                    ),
                ])),
            ),
            (
                "CTRL-Y".to_string(),
                VdevKey::Macro(VdevKeyMacro::from([
                    (
                        "0".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "CTRL".to_string(),
                            until: None,
                            until_after: Some("1".to_string()),
                        },
                    ),
                    (
                        "1".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "y".to_string(),
                            until: None,
                            until_after: None,
                        },
                    ),
                ])),
            ),
            (
                "CTRL-O".to_string(),
                VdevKey::Macro(VdevKeyMacro::from([
                    (
                        "0".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "CTRL".to_string(),
                            until: None,
                            until_after: Some("1".to_string()),
                        },
                    ),
                    (
                        "1".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "o".to_string(),
                            until: None,
                            until_after: None,
                        },
                    ),
                ])),
            ),
            (
                "CTRL-T".to_string(),
                VdevKey::Macro(VdevKeyMacro::from([
                    (
                        "0".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "CTRL".to_string(),
                            until: None,
                            until_after: Some("1".to_string()),
                        },
                    ),
                    (
                        "1".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "t".to_string(),
                            until: None,
                            until_after: None,
                        },
                    ),
                ])),
            ),
            (
                "CTRL-C".to_string(),
                VdevKey::Macro(VdevKeyMacro::from([
                    (
                        "0".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "CTRL".to_string(),
                            until: None,
                            until_after: Some("1".to_string()),
                        },
                    ),
                    (
                        "1".to_string(),
                        VdevKeyMacroSequenceEntrant {
                            to: "c".to_string(),
                            until: None,
                            until_after: None,
                        },
                    ),
                ])),
            ),
        ])
    }
    pub fn init_layers() -> VdevKeyLayerCodex {
        HashMap::from([
            (
                0x40,
                HashMap::from([
                    (
                        "Enter".to_string(),
                        VdevKey::Macro(VdevKeyMacro::from([
                            (
                                "0".to_string(),
                                VdevKeyMacroSequenceEntrant {
                                    to: "SHIFT".to_string(),
                                    until: None,
                                    until_after: Some("1".to_string()),
                                },
                            ),
                            (
                                "1".to_string(),
                                VdevKeyMacroSequenceEntrant {
                                    to: "TAB".to_string(),
                                    until: None,
                                    until_after: None,
                                },
                            ),
                        ])),
                    ),
                    (
                        "n".to_string(),
                        VdevKey::Macro(VdevKeyMacro::from([(
                            "0".to_string(),
                            VdevKeyMacroSequenceEntrant {
                                to: "~".to_string(),
                                until: None,
                                until_after: None,
                            },
                        )])),
                    ),
                    (
                        "l".to_string(),
                        VdevKey::Macro(VdevKeyMacro::from([(
                            "0".to_string(),
                            VdevKeyMacroSequenceEntrant {
                                to: "@".to_string(),
                                until: None,
                                until_after: None,
                            },
                        )])),
                    ),
                    (
                        "5".to_string(),
                        VdevKey::Macro(VdevKeyMacro::from([(
                            "0".to_string(),
                            VdevKeyMacroSequenceEntrant {
                                to: "[".to_string(),
                                until: None,
                                until_after: None,
                            },
                        )])),
                    ),
                    (
                        "6".to_string(),
                        VdevKey::Macro(VdevKeyMacro::from([(
                            "0".to_string(),
                            VdevKeyMacroSequenceEntrant {
                                to: "]".to_string(),
                                until: None,
                                until_after: None,
                            },
                        )])),
                    ),
                    (
                        "7".to_string(),
                        VdevKey::Macro(VdevKeyMacro::from([(
                            "0".to_string(),
                            VdevKeyMacroSequenceEntrant {
                                to: "|".to_string(),
                                until: None,
                                until_after: None,
                            },
                        )])),
                    ),
                    (
                        "/".to_string(),
                        VdevKey::Macro(VdevKeyMacro::from([(
                            "0".to_string(),
                            VdevKeyMacroSequenceEntrant {
                                to: "\\".to_string(),
                                until: None,
                                until_after: None,
                            },
                        )])),
                    ),
                    (
                        "8".to_string(),
                        VdevKey::Macro(VdevKeyMacro::from([(
                            "0".to_string(),
                            VdevKeyMacroSequenceEntrant {
                                to: "{".to_string(),
                                until: None,
                                until_after: None,
                            },
                        )])),
                    ),
                    (
                        "9".to_string(),
                        VdevKey::Macro(VdevKeyMacro::from([(
                            "0".to_string(),
                            VdevKeyMacroSequenceEntrant {
                                to: "}".to_string(),
                                until: None,
                                until_after: None,
                            },
                        )])),
                    ),
                ]),
            ),
            (
                0x80,
                HashMap::from([
                    (
                        "Enter".to_string(),
                        VdevKey::Macro(VdevKeyMacro::from([(
                            "0".to_string(),
                            VdevKeyMacroSequenceEntrant {
                                to: "TAB".to_string(),
                                until: None,
                                until_after: None,
                            },
                        )])),
                    ),
                    (
                        "h".to_string(),
                        VdevKey::Macro(VdevKeyMacro::from([(
                            "0".to_string(),
                            VdevKeyMacroSequenceEntrant {
                                to: "LEFT".to_string(),
                                until: None,
                                until_after: None,
                            },
                        )])),
                    ),
                    (
                        "j".to_string(),
                        VdevKey::Macro(VdevKeyMacro::from([(
                            "0".to_string(),
                            VdevKeyMacroSequenceEntrant {
                                to: "DOWN".to_string(),
                                until: None,
                                until_after: None,
                            },
                        )])),
                    ),
                    (
                        "k".to_string(),
                        VdevKey::Macro(VdevKeyMacro::from([(
                            "0".to_string(),
                            VdevKeyMacroSequenceEntrant {
                                to: "UP".to_string(),
                                until: None,
                                until_after: None,
                            },
                        )])),
                    ),
                    (
                        "l".to_string(),
                        VdevKey::Macro(VdevKeyMacro::from([(
                            "0".to_string(),
                            VdevKeyMacroSequenceEntrant {
                                to: "RIGHT".to_string(),
                                until: None,
                                until_after: None,
                            },
                        )])),
                    ),
                ]),
            ),
            (
                0xC0,
                HashMap::from([(
                    "Enter".to_string(),
                    VdevKey::Macro(VdevKeyMacro::from([
                        (
                            "0".to_string(),
                            VdevKeyMacroSequenceEntrant {
                                to: "CTRL".to_string(),
                                until: None,
                                until_after: Some("1".to_string()),
                            },
                        ),
                        (
                            "1".to_string(),
                            VdevKeyMacroSequenceEntrant {
                                to: "TAB".to_string(),
                                until: None,
                                until_after: None,
                            },
                        ),
                    ])),
                )]),
            ),
        ])
    }
    pub fn init() -> VdevKeys {
        // TODO(@dom): apply layers/remaps here
        Self {
            codex: VdevKeys::init_codex(),
            layers: VdevKeys::init_layers(),
        }
    }
    pub fn get_vdev_key(self, modifier: Modifiers, key_character: String) -> Option<VdevKey> {
        match modifier {
            Modifiers::Bare(_) => self.codex.get(&key_character).cloned(),
            Modifiers::OpenApple(_) => {
                let layer = self.layers.get(&modifier.inner()).cloned().unwrap();
                layer.get(&key_character).cloned()
            }
            Modifiers::ClosedApple(_) => {
                let layer = self.layers.get(&modifier.inner()).cloned().unwrap();
                layer.get(&key_character).cloned()
            }
            Modifiers::OpenClosedApple(_) => {
                let layer = self.layers.get(&modifier.inner()).cloned().unwrap();
                layer.get(&key_character).cloned()
            }
        }
    }
}
