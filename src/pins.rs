use pin::Pin;

// Pins class
//
//    Holds configuration for all the pins. All pin modifications are connected to slots of this class.
//
//    \sa Peripheral, Pin
#[derive(Serialize, Deserialize, Debug)]
pub struct Pins {
    pub pins: Vec<Pin>,
}

impl Pins {
    fn new(pins: Vec<Pin>) -> Self {
        Pins { pins: pins }
    }

    pub fn pins(&mut self) -> &Vec<Pin> {
        &mut self.pins
    }

    pub fn find_pin(&self, name: &str) -> Vec<usize> {
        let mut pins: Vec<usize> = vec![];

        for (idx, pin) in self.pins.iter().enumerate() {
            if pin.name() == name {
                pins.push(idx);
            } else {
                match pin.params() {
                    Some(params) => {
                        match params.signals().iter().position(|ref s| s.contains(name)) {
                            Some(_) => pins.push(idx),
                            None => (),
                        }
                    }
                    None => (),
                }
            }
        }

        pins
    }

    pub fn find_alternate_pins(&self, idx: usize, name: &str) {
        let mut pins: Vec<usize> = vec![];

        for (idx, pin) in self.pins.iter().enumerate() {
            match pin.params() {
                Some(params) => match params.signals().iter().position(|ref s| s.contains(name)) {
                    Some(_) => pins.push(idx),
                    None => (),
                },
                None => (),
            }
        }
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
mod tests {

    use super::*;
    use std::path::Path;
    use mcu::{MCUConf, MCU};

    #[test]
    fn load_pins_ok() {
        let sample = Path::new("./samples/STM32F030C6Tx.json");
        let mcu = MCU::new(sample).unwrap();

        let mcu_conf = mcu.finish();
        let pins = mcu_conf.get_pins();
    }

    #[test]
    fn find_pin_ok() {
        let sample = Path::new("./samples/STM32F030C6Tx.json");
        let mcu = MCU::new(sample).unwrap();

        let mcu_conf = mcu.finish();
        let pins = mcu_conf.get_pins();

        let found = pins.find_pin("PA2");

        assert_eq!(vec![11], found);
    }

    #[test]
    fn find_signal_ok() {
        let sample = Path::new("./samples/STM32F030C6Tx.json");
        let mcu = MCU::new(sample).unwrap();

        let mcu_conf = mcu.finish();
        let pins = mcu_conf.get_pins();

        let found = pins.find_pin("USART1_DE");

        assert_eq!(vec![10, 32], found);
    }

    #[test]
    fn find_signal_empty() {
        let sample = Path::new("./samples/STM32F030C6Tx.json");
        let mcu = MCU::new(sample).unwrap();

        let mcu_conf = mcu.finish();
        let pins = mcu_conf.get_pins();

        let found = pins.find_pin("XXXX");

        let empty: Vec<usize> = Vec::new();
        assert_eq!(empty, found);
    }

    #[test]
    fn find_alternative_pins() {
        let sample = Path::new("./samples/STM32F030C6Tx.json");
        let mcu = MCU::new(sample).unwrap();

        let mcu_conf = mcu.finish();
        let pins = mcu_conf.get_pins();

        let found = pins.find_pin("I2C1_SCL");

        assert_eq!(vec![20, 29, 34, 41, 44], found);
    }
}
