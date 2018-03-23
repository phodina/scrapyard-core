use std::error::Error;
use std::fs::File;
use std::path::Path;

use module::peripheral::Peripheral;
use pin::Pin;
use pins::Pins;
use package::Package;

use serde_json;

use memory::Memory;

#[derive(Serialize, Deserialize, Debug)]
pub enum Platform {
    STM32 { family: String, line: String },
    STM8,
    AVR,
    MSP430,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ARMCore {
    CortexM0,
    CortexM3,
    CortexM4,
    CortexM7,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Core {
    ARM(ARMCore),
    AVR,
    STM8,
    MSP430,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Frequency {
    MHz(u16),
}

// TODO: Do not store Middleware as IP
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct IP {
    pub config_file: String,
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MCU {
    pub memory: Vec<Memory>,
    pub frequency: Frequency,
    pub core: Core,
    pub name: String,
    pub package: Package,
    pub ips: Vec<IP>,
    pub pins: Vec<Pin>,
    pub platform: Platform,
}

impl MCU {
    pub fn new(path: &Path) -> Result<MCU, Box<Error>> {
        let file = File::open(path)?;
        let mcu: MCU = serde_json::from_reader(file)?;

        Ok(mcu)
    }

    fn process_peripherals(&self) -> Vec<Peripheral> {
        let mut peripherals: Vec<Peripheral> = Vec::with_capacity(self.ips.len());

        for ip in &self.ips {
            match ip.name.as_ref() {
                name @ "GPIO" => {
                    println!("IO peripheral {}, {}", name, ip.config_file);
                    //let pins = PinsBuilder::new(&ip.name, &ip.config_file, &mut self.mcu.pins);
                    //peripherals.push(pins.finish());
                }
                name => {
                    println!("Peripheral: {}, {}", name, ip.config_file);
                    let peripheral = Peripheral::new(&name, &ip.config_file);
                    peripheral.setup();
                    peripherals.push(peripheral);
                }
            }
        }

        peripherals
    }

    pub fn finish(self) -> MCUConf {
        let peripherals = self.process_peripherals();
        let middlewares: Vec<String> = Vec::new();
        let components: Vec<String> = Vec::new();

        MCUConf {
            memory: self.memory,
            frequency: self.frequency,
            platform: self.platform,
            core: self.core,
            name: self.name,
            package: self.package,
            periherals: peripherals,
            middlewares: middlewares,
            components: components,
            pins: Pins { pins: self.pins },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MCUConf {
    memory: Vec<Memory>,
    frequency: Frequency,
    platform: Platform,
    core: Core,
    name: String,
    package: Package,
    periherals: Vec<Peripheral>,
    middlewares: Vec<String>,
    components: Vec<String>,
    pins: Pins,
}

impl MCUConf {
    pub fn get_pins(&self) -> &Pins {
        &self.pins
    }

    pub fn get_peripherals(&self) -> &Vec<Peripheral> {
        &self.periherals
    }
}

#[cfg(test)]
mod tests {
    // TODO: Check for memory and IPs
    use super::*;
    use pin::Position;
    // TODO: Test for no file
    #[test]
    fn no_file() {
        let sample = Path::new(".samples/none.json");
        let mcu = MCU::new(sample);
        //assert!(mcu.err(), Err(std::io::ErrorKind::NotFound));
    }

    // TODO: Test no file
    // TODO: Test json error
    #[test]
    fn mcubuilder_load() {
        let sample = Path::new("./samples/STM32F030C6Tx.json");
        let mcu = MCU::new(sample).unwrap();

        assert_eq!(mcu.core, Core::ARM(ARMCore::CortexM0));
        assert_eq!(2, mcu.memory.len());
        assert_eq!(
            Memory::Flash {
                start: 0x08000000,
                size: 32,
            },
            mcu.memory[0]
        );
        assert_eq!(
            Memory::Ram {
                start: 0x20000000,
                size: 4,
            },
            mcu.memory[1]
        );
        assert_eq!(mcu.frequency, Frequency::MHz(48));
        assert_eq!(mcu.name, "STM32F030C6Tx");
        assert_eq!(mcu.package, Package::LQFP(48));
        //assert_eq!(mcubuilder.mcu.ips.len(), 20);
        //assert_eq!(mcubuilder.is_ok(), true);
    }

    #[test]
    fn mcubuilder_builder() {
        let sample = Path::new("./samples/STM32F030C6Tx.json");
        let mcu = MCU::new(sample).unwrap();

        let mcu_conf = mcu.finish();

        assert_eq!(mcu_conf.core, Core::ARM(ARMCore::CortexM0));
        assert_eq!(2, mcu_conf.memory.len());
        assert_eq!(
            Memory::Flash {
                start: 0x08000000,
                size: 32,
            },
            mcu_conf.memory[0]
        );
        assert_eq!(
            Memory::Ram {
                start: 0x20000000,
                size: 4,
            },
            mcu_conf.memory[1]
        );
        assert_eq!(mcu_conf.frequency, Frequency::MHz(48));
        assert_eq!(mcu_conf.name, "STM32F030C6Tx");
        match mcu_conf.package {
            Package::LQFP(_) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn ip_ok() {
        let json = r#"{ "config_file" : "adc.conf",
    					"name" : "ADC"}"#;

        let ip: IP = serde_json::from_str(json).unwrap();

        assert_eq!(ip.config_file, "adc.conf");
        assert_eq!(ip.name, "ADC");
    }

    #[test]
    fn pin_ok() {
        let json = r#"{"POWER":{"name":"VSS","position":{"Linear":47}}}"#;

        let pin: Pin = serde_json::from_str(json).unwrap();

        assert_eq!(pin.name(), "VSS");
        assert_eq!(*pin.position(), Position::Linear(47));
        match pin {
            Pin::POWER { .. } => assert!(true),
            _ => assert!(false),
        };
    }
}
