pub struct PinBuilder {}

impl PinBuilder {
    pub fn new(pin_type: &str) -> PinBuilder {
        PinBuilder {}
    }

    pub fn position(&mut self, position: Position) -> PinBuilder {
        PinBuilder {}
    }

    pub fn name(&mut self, name: &str) -> PinBuilder {
        PinBuilder {}
    }

    pub fn signals(&mut self, signals: Vec<String>) -> PinBuilder {
        PinBuilder {}
    }

    pub fn current(&mut self, current: &str) -> PinBuilder {
        PinBuilder {}
    }

    pub fn finish(mut self) -> Pin {
        let name = String::from("NC");
        Pin::NC {
            name: name,
            position: Position::Grid(0, 0),
        }
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub enum Position {
    Linear(u16),
    Grid(u8, u8),
}

#[derive(Serialize, Debug)]
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
        let position = self.signals.iter().position(|r| r == signal);

        match position {
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

#[derive(Serialize, Debug)]
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

    pub fn params(&mut self) -> Option<&mut IOPin> {
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

        let params = pin.params().unwrap();
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

        let params = pin.params().unwrap();
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

        let params = pin.params().unwrap();
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

        let params = pin.params().unwrap();
        let ret = params.select_signal("Missing");

        assert_eq!(ret, false);
        assert_eq!(params.current_signal().is_none(), true);
    }
}
