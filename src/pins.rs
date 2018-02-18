use pin::Position;
use pin::IOPin;
use pin::Pin;

// Pins class
//
//    Holds configuration for all the pins. All pin modifications are connected to slots of this class.
//
//    \sa Peripheral, Pin
//
pub struct Pins {
    pins: Vec<Pin>,
}

impl Pins {
    pub fn new() -> Self {
        /*
        let pin0 = Pin::IO {
            name: "PA0".to_string(),
            position: Position::Linear(1),
            params: Box::new(IOPin {
                reset: true,
                label: "Hello".to_string(),
                signals: vec![
                    "Input".to_string(),
                    "Output".to_string(),
                    "EXTI".to_string(),
                ],
                current: Some(0),
            }),
        };

        let pin1 = Pin::IO {
            name: "PA1".to_string(),
            position: Position::Linear(2),
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

        let pin2 = Pin::IO {
            name: "PA2".to_string(),
            position: Position::Linear(3),
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

        let pin3 = Pin::IO {
            name: "PA3".to_string(),
            position: Position::Linear(4),
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

        let pin4 = Pin::IO {
            name: "PA4".to_string(),
            position: Position::Linear(5),
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

        Pins {
            pins: vec![pin0, pin1, pin2, pin3, pin4],
    }*/
        Pins { pins: vec![] }
    }

    pub fn pins(&self) -> &Vec<Pin> {
        &self.pins
    }

    fn find_pin(&self, name: &str) {
        //let mut pins: Vec<usize> = vec![];

        //let mut iter = IntoIterator::into_iter(&self.pins);

        // loop {
        //     match iter.next() {
        //         Some(pin) => {

        //         	pin.position(|ref r| r == &signal)
        //             if name == pin.name() {

        //             	match
        //             	pins.push(iter.position());
        // 				}
        // 			else {
        // 				match pin.signals() {
        // 					Some(ref idx) => break,
        // 					None => ()
        // 					}
        //         		}
        //     		},
        //         None => break,
        //     }
        //}

        //                    for (auto j = ioPin->getIOPinFunctions().begin(); j != ioPin->getIOPinFunctions().end(); j++) {

        //                        if ((*j).contains(str.toUpper())) {

        //                            pinsToHighlight.append(pins.indexOf(*i));
        //                            break;
        //                            }
        //                        }
        //
        //    emit highlightPins (pinsToHighlight, PinItem::HIGHLIGHT_SEARCH);
    }

    fn find_alternate_pins(&mut self, idx: usize, name: &str) {
        for mut pin in &mut self.pins {
            let params = pin.params();

            match params {
                Some(params) => {
                    let position = params.signals().iter().position(|r| r == name);
                    match position {
                        Some(_i) => (),
                        None => (),
                    }
                }
                None => (),
            }
        }

        //    emit highlightPins(list, PinItem::HIGHLIGHT_ALTERNATIVE);
    }

    // Configures or resets pins belonging to peripheral

    //    Handles the configuration of pin with signals \p peripheralPins. Iterates through pins until finds pin with corresponding peripheral signal. Parameter \p state either sets the alternate signal on the pin if it's reset or resets the pin if it's configured with the same alternate signal.

    //    \param peripheralPins List of alternate functions to configure
    //    \param state Configure or reset the pin
    //
    fn peripheral_pins() {}
    //void Pins::onPeripheralPins(QList<QString> peripheralPins, Pin::PinState state)
    //{

    //    Pin::AlternateFunction alternateFunction;

    //    bool configured;

    //    // TODO: Handle when the are no pins available for configuration
    //    foreach (QString value, peripheralPins) {

    //        configured = false;

    //        foreach (pin, pins) {

    //            foreach (alternateFunction, pin->getAlternateFunctions()) {

    //                if (alternateFunction.name == value) {

    //                    if (state == Pin::PIN_ASSIGNED) {

    //                        if (pin->isReset()) {

    //                            //pin->setGpioMode(GPIOStr::gpioModeStr[3]);
    //                            pin->selectAlternateFunction(value);

    //                            //emit pinFunctionChanged (pin->getName(), state, value);

    //                            configured = true;

    //                            break;
    //                            }
    //                        }
    //                    else {

    //                        if (pin->getAlternateFunction().name == value) {

    //                            pin->resetPin();

    //                            //emit pinFunctionChanged (pin->getName(), Pin::Pins_RESET, "");

    //                            configured = true;

    //                            break;
    //                            }
    //                        }
    //                    }
    //                }

    //            if (configured){

    //                break;
    //                }
    //            }
    //        }
    //}
}

#[cfg(test)]
mod tests {}
