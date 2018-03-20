use std::error::Error;
use std::fs::File;
use std::path::Path;

use module::peripheral::Peripheral;
use pin::{Pin, Position};
use package::Package;

use serde_json;

use memory::Memory;

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
    pub frequency: u32,
    pub core: String,
    pub name: String,
    pub package: Package,
    pub ips: Vec<IP>,
    pub pins: Vec<Pin>,
    /*
    IOs: u16,
    Line: Option<String>,
    Family: Option<String>,
    */
}

impl MCU {
    pub fn new(path: &Path) -> Result<MCU, Box<Error>> {
        let file = File::open(path)?;
        let mcu: MCU = serde_json::from_reader(file)?;

        Ok(mcu)
    }

    fn process_peripherals(&mut self) {
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
    }

    pub fn finish(mut self) -> MCUConf {

        let peripherals: Vec<String> = Vec::new();
        let middlewares: Vec<String> = Vec::new();
        let components: Vec<String> = Vec::new();

        MCUConf {
            memory: self.memory,
            frequency: self.frequency,
            //ios: self.mcu.Mcu.IOs,
            core: self.core,
            name: self.name,
            package: self.package,
            periherals: peripherals,
            middlewares: middlewares,
            components: components,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MCUConf {
    
    memory: Vec<Memory>,
    frequency: u32,
    //ios: u16,
    core: String,
    name: String,
    package: Package,
    periherals: Vec<String>,
    middlewares: Vec<String>,
    components: Vec<String>,
}

#[cfg(test)]
mod tests {
    // TODO: Check for memory and IPs
    use super::*;
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

        assert_eq!(mcu.core, "ARM Cortex-M0");
        //assert_eq!(mcubuilder.mcu.Reprom, 0);
        //assert_eq!(mcubuilder.mcu.Flash, 32);
        //assert_eq!(mcubuilder.mcu.ram, 4);
        assert_eq!(mcu.frequency, 48);
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

        assert_eq!(mcu_conf.core, "ARM Cortex-M0");
        //assert_eq!(mcu_conf.eeprom, 0);
        //assert_eq!(mcu_conf.flash, 32);
        //assert_eq!(mcu_conf.ram, 4);
        assert_eq!(mcu_conf.frequency, 48);
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
            Pin::POWER{ .. } => assert!(true),
            _ => assert!(false),
        };
    }
}