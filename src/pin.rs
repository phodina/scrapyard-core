#[derive(Debug)]
pub struct PinBuilder<'a> {
    pin_type: &'a str,
    position: Position,
    name: &'a str,
    signals: Option<Vec<String>>,
    current: Option<usize>,
}

impl<'a> PinBuilder<'a> {
    pub fn new(pin_type: &'a str, position: Position, name: &'a str) -> PinBuilder<'a> {
        PinBuilder {
            pin_type: pin_type,
            name: name,
            position: position,
            signals: None,
            current: None,
        }
    }

    pub fn signals(&mut self, signals: Vec<String>, current: usize) -> PinBuilder<'a> {
        PinBuilder {
            pin_type: self.pin_type,
            name: self.name,
            position: self.position,
            signals: Some(signals),
            current: Some(current),
        }
    }

    pub fn finish(self) -> Pin {
        let current = match self.current {
            Some(idx) => match &self.signals {
                &Some(ref signals) => {
                    if idx < signals.len() {
                        Some(idx)
                    } else {
                        None
                    }
                }
                &None => None,
            },
            None => None,
        };

        // TODO: Return also error
        match self.pin_type {
            "NC" => Pin::NC {
                name: String::from(self.name),
                position: self.position,
            },
            "BOOT" => Pin::BOOT {
                name: String::from(self.name),
                position: self.position,
            },
            "Reset" => Pin::NRST {
                name: String::from(self.name),
                position: self.position,
            },
            "Power" => Pin::POWER {
                name: String::from(self.name),
                position: self.position,
            },
            "I/O" => {
                return Pin::IO {
                    name: String::from(self.name),
                    position: self.position,
                    params: Box::new(IOPin {
                        reset: true,
                        label: String::new(),
                        signals: match self.signals {
                            Some(s) => s,
                            None => Vec::new(),
                        },
                        current: current,
                    }),
                }
            }
            &_ => Pin::NC {
                name: String::from(self.name),
                position: self.position,
            },
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum Position {
    Linear(u16),
    Grid(u8, u8),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IOPin {
    reset: bool,
    label: String,
    signals: Vec<String>,
    current: Option<usize>,
}

impl IOPin {
    pub fn reset(&mut self) {
        self.reset = true;
    }

    pub fn is_reset(&self) -> bool {
        self.reset
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn signals(&self) -> &Vec<String> {
        &self.signals
    }

    pub fn select_signal(&mut self, signal: &str) -> bool {
        let item = self.signals.iter().position(|r| r.as_str() == signal);

        match item {
            Some(idx) => {
                self.current = Some(idx);
                true
            }
            None => {
                self.current = None;
                false
            }
        }
    }

    pub fn current_signal(&self) -> Option<&str> {
        match self.current {
            Some(idx) => Some(&self.signals[idx]),
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Pin {
    NC {
        name: String,
        position: Position,
    },
    IO {
        name: String,
        position: Position,
        params: Box<IOPin>,
    },
    BOOT {
        name: String,
        position: Position,
    },
    NRST {
        name: String,
        position: Position,
    },
    POWER {
        name: String,
        position: Position,
    },
}

impl Pin {
    pub fn name(&self) -> &str {
        match *self {
            Pin::NC { ref name, .. } => &name,
            Pin::IO { ref name, .. } => &name,
            Pin::BOOT { ref name, .. } => &name,
            Pin::NRST { ref name, .. } => &name,
            Pin::POWER { ref name, .. } => &name,
        }
    }

    pub fn position(&self) -> &Position {
        match *self {
            Pin::NC { ref position, .. } => &position,
            Pin::IO { ref position, .. } => &position,
            Pin::BOOT { ref position, .. } => &position,
            Pin::NRST { ref position, .. } => &position,
            Pin::POWER { ref position, .. } => &position,
        }
    }

    pub fn params(&self) -> Option<&IOPin> {
        match *self {
            Pin::IO { ref params, .. } => Some(params),
            _ => None,
        }
    }

    pub fn params_mut(&mut self) -> Option<&mut IOPin> {
        match *self {
            Pin::IO { ref mut params, .. } => Some(params),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn pin_ok() {
        let pin = Pin::POWER {
            name: "VDD".to_string(),
            position: Position::Grid(4, 3),
        };

        assert_eq!(pin.name(), "VDD");
        assert_eq!(*pin.position(), Position::Grid(4, 3));
    }

    #[test]
    fn pin_reset() {
        let pin = Pin::IO {
            name: "PA3".to_string(),
            position: Position::Grid(4, 3),
            params: Box::new(IOPin {
                reset: true,
                label: "".to_string(),
                signals: vec![],
                current: None,
            }),
        };

        let params = pin.params().unwrap();
        assert_eq!(params.is_reset(), true);
    }

    #[test]
    fn pin_label() {
        let mut pin = Pin::IO {
            name: "PA3".to_string(),
            position: Position::Grid(4, 3),
            params: Box::new(IOPin {
                reset: true,
                label: "".to_string(),
                signals: vec![],
                current: None,
            }),
        };

        let params = pin.params_mut().unwrap();
        params.set_label("PWM");

        assert_eq!(params.label(), "PWM");
    }

    #[test]
    fn pin_signals() {
        let mut pin = Pin::IO {
            name: "PA3".to_string(),
            position: Position::Grid(4, 3),
            params: Box::new(IOPin {
                reset: true,
                label: "".to_string(),
                signals: vec![
                    "Input".to_string(),
                    "Output".to_string(),
                    "EXTI".to_string(),
                ],
                current: Some(0),
            }),
        };

        let params = pin.params_mut().unwrap();
        let signals = params.signals();

        assert_eq!(
            *signals,
            vec![
                "Input".to_string(),
                "Output".to_string(),
                "EXTI".to_string(),
            ]
        );
    }

    #[test]
    fn pin_select_signal_present() {
        let mut pin = Pin::IO {
            name: "PA3".to_string(),
            position: Position::Grid(4, 3),
            params: Box::new(IOPin {
                reset: true,
                label: "".to_string(),
                signals: vec![
                    "Input".to_string(),
                    "Output".to_string(),
                    "EXTI".to_string(),
                ],
                current: Some(1),
            }),
        };

        let params = pin.params_mut().unwrap();
        let ret = params.select_signal("Output");

        assert_eq!(ret, true);
        assert_eq!(params.current_signal().unwrap(), "Output");
    }

    #[test]
    fn pin_select_signal_missing() {
        let mut pin = Pin::IO {
            name: "PA3".to_string(),
            position: Position::Grid(4, 3),
            params: Box::new(IOPin {
                reset: true,
                label: "".to_string(),
                signals: vec![
                    "Input".to_string(),
                    "Output".to_string(),
                    "EXTI".to_string(),
                ],
                current: Some(0),
            }),
        };

        let params = pin.params_mut().unwrap();
        let ret = params.select_signal("Missing");

        assert_eq!(ret, false);
        assert_eq!(params.current_signal().is_none(), true);
    }

    // TODO: Create unknown Pin type and fail
    #[test]
    fn build_nc_pin() {
        let pinbuilder = PinBuilder::new("NC", Position::Linear(10), "NotConnected");
        let pin = pinbuilder.finish();

        match pin {
            Pin::NC { .. } => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn build_boot_pin() {
        let pinbuilder = PinBuilder::new("BOOT", Position::Linear(10), "WakeUp");
        let pin = pinbuilder.finish();

        match pin {
            Pin::BOOT { .. } => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn build_power_pin() {
        let pinbuilder = PinBuilder::new("Power", Position::Linear(10), "VCC");
        let pin = pinbuilder.finish();

        match pin {
            Pin::POWER { .. } => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn build_nrst_pin() {
        let pinbuilder = PinBuilder::new("Reset", Position::Linear(10), "NRST");
        let pin = pinbuilder.finish();

        match pin {
            Pin::NRST { .. } => assert!(true),
            _ => assert!(false),
        }
    }

    // TODO: Check signals
    #[test]
    fn build_io_pin() {
        let pinbuilder = PinBuilder::new("I/O", Position::Linear(10), "PA1")
            .signals(vec![String::from("Input"), String::from("Output")], 0);
        let pin = pinbuilder.finish();

        match pin {
            Pin::IO { .. } => assert!(true),
            _ => assert!(false),
        }
    }
}
