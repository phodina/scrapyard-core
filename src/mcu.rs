use std::error::Error;
use std::fs::File;
use std::path::Path;

use module::peripheral::Peripheral;
use pins::PinsBuilder;

use serde_json;
use regex::Regex;

#[derive(Serialize, Deserialize, Debug)]
enum Package {
    LQFP(u16),
    TSSOP(u16),
    WLCSP(u16),
    UFQFPN(u16),
    TFBGA(u16),
    VFQFPN(u16),
    EWLCSP(u16),
    UFBGA(u16),
    Unknown(u16),
}

impl Package {
    // TODO: Return error
    fn new(package: &str) -> Package {
        lazy_static! {
            static ref RE :Regex = Regex::new(r"([[:alpha:]]*)(\d*)").unwrap();
        }

        let caps = RE.captures(package).unwrap();

        let count = caps.get(2).unwrap().as_str().parse::<u16>().unwrap();

        match caps.get(1).unwrap().as_str() {
            "LQFP" => Package::LQFP(count),
            "TSSOP" => Package::TSSOP(count),
            "WLCSP" => Package::WLCSP(count),
            "UFQFPN" => Package::UFQFPN(count),
            "TFBGA" => Package::TFBGA(count),
            "VFQFPN" => Package::VFQFPN(count),
            "EWLCSP" => Package::EWLCSP(count),
            "UFBGA" => Package::UFBGA(count),
            &_ => Package::Unknown(count),
        }
    }

    pub fn is_grid(&self) -> bool {
        match *self {
            Package::UFBGA(_) => true,
            Package::TFBGA(_) => true,
            Package::EWLCSP(_) => true,
            _ => false,
        }
    }

    pub fn pins(&self) -> u16 {
        match *self {
            Package::LQFP(count) => count,
            Package::TSSOP(count) => count,
            Package::WLCSP(count) => count,
            Package::UFQFPN(count) => count,
            Package::TFBGA(count) => count,
            Package::VFQFPN(count) => count,
            Package::EWLCSP(count) => count,
            Package::UFBGA(count) => count,
            Package::Unknown(count) => count,
        }
    }
}
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct MCU {
    Flash: u32,
    Eeprom: u32,
    Ram: u32,
    Frequency: u32,
    IOs: u16,
    Core: String,
    Name: String,
    Line: Option<String>,
    Family: Option<String>,
    Package: String,
    #[serde(rename = "IP")] IPs: Vec<IP>,
    #[serde(rename = "Pin")] Pins: Vec<Pin>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct MCURoot {
    Mcu: MCU,
}

pub struct MCUBuilder {
    mcu: MCURoot,
    //pins: Option<PinsBuilder>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct IP {
    ConfigFile: String,
    Name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Pin {
    pub Name: String,
    pub Position: u16,
    pub Type: String,
}

impl<'a> MCUBuilder {
    pub fn new(path: &Path) -> Result<MCUBuilder, Box<Error>> {
        let file = File::open(path)?;
        let mcu_root: MCURoot = serde_json::from_reader(file)?;

        Ok(MCUBuilder {
            mcu: mcu_root,
            //          pins: None,
        })
    }

    fn process_peripherals(&mut self) {
        let mut peripherals: Vec<Peripheral> = Vec::with_capacity(self.mcu.Mcu.IPs.len());

        for ip in &self.mcu.Mcu.IPs {
            match ip.Name.as_ref() {
                name @ "GPIO" => {
                    println!("IO peripheral {}, {}", name, ip.ConfigFile);
                    let pins = PinsBuilder::new(&ip.Name, &ip.ConfigFile, &mut self.mcu.Mcu.Pins);
                    //peripherals.push(pins.finish());
                }
                name => {
                    println!("Peripheral: {}, {}", name, ip.ConfigFile);
                    let peripheral = Peripheral::new(&name, &ip.ConfigFile);
                    peripheral.setup();
                    peripherals.push(peripheral);
                }
            }
        }
    }

    fn process_package(&self) -> Package {
        Package::new(&self.mcu.Mcu.Package)
    }

    pub fn finish(mut self) -> MCUConf {
        self.process_peripherals();

        let package = self.process_package();
        MCUConf {
            flash: self.mcu.Mcu.Flash,
            eeprom: self.mcu.Mcu.Eeprom,
            ram: self.mcu.Mcu.Ram,
            frequency: self.mcu.Mcu.Frequency,
            ios: self.mcu.Mcu.IOs,
            core: self.mcu.Mcu.Core,
            name: self.mcu.Mcu.Name,
            package: package,
            periherals: None,
            middlewares: None,
            components: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MCUConf {
    flash: u32,
    eeprom: u32,
    ram: u32,
    frequency: u32,
    ios: u16,
    core: String,
    name: String,
    package: Package,
    periherals: Option<Vec<String>>,
    middlewares: Option<Vec<String>>,
    components: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {

    use super::*;
    // TODO: Test for no file
    #[test]
    fn no_file() {
        let sample = Path::new(".samples/none.json");
        let mcubuilder = MCUBuilder::new(sample);
        //assert!(mcubuilder.err(), Err(std::io::ErrorKind::NotFound));
    }
    // TODO: Test no file
    // TODO: Test json error
    #[test]
    fn mcubuilder_load() {
        let sample = Path::new("./samples/STM32F030C6Tx.json");
        let mcubuilder = MCUBuilder::new(sample).unwrap();

        assert_eq!(mcubuilder.mcu.Mcu.Core, "ARM Cortex-M0");
        assert_eq!(mcubuilder.mcu.Mcu.Eeprom, 0);
        assert_eq!(mcubuilder.mcu.Mcu.Flash, 32);
        assert_eq!(mcubuilder.mcu.Mcu.Frequency, 48);
        assert_eq!(mcubuilder.mcu.Mcu.Name, "STM32F030C6Tx");
        assert_eq!(mcubuilder.mcu.Mcu.Package, "LQFP48");
        assert_eq!(mcubuilder.mcu.Mcu.Ram, 4);
        assert_eq!(mcubuilder.mcu.Mcu.IPs.len(), 20);
        //assert_eq!(mcubuilder.is_ok(), true);
    }

    #[test]
    fn mcubuilder_builder() {
        let sample = Path::new("./samples/STM32F030C6Tx.json");
        let mcubuilder = MCUBuilder::new(sample).unwrap();

        let mcu_conf = mcubuilder.finish();

        assert_eq!(mcu_conf.core, "ARM Cortex-M0");
        assert_eq!(mcu_conf.eeprom, 0);
        assert_eq!(mcu_conf.flash, 32);
        assert_eq!(mcu_conf.frequency, 48);
        assert_eq!(mcu_conf.name, "STM32F030C6Tx");
        match mcu_conf.package {
            Package::LQFP(_) => assert!(true),
            _ => assert!(false),
        };
        assert_eq!(mcu_conf.ram, 4);
        assert_eq!(mcu_conf.periherals.is_some(), false);
    }

    #[test]
    fn pin_ok() {
        let json = r#"{ "ConfigFile" : "adc.conf",
    					"Name" : "ADC"}"#;

        let ip: IP = serde_json::from_str(json).unwrap();

        assert_eq!(ip.ConfigFile, "adc.conf");
        assert_eq!(ip.Name, "ADC");
    }

    #[test]
    fn ip_ok() {
        let json = r#"{ "Name" : "VCC",
    					"Position" : 4,
    					"Type" : "Power"}"#;

        let pin: Pin = serde_json::from_str(json).unwrap();

        assert_eq!(pin.Name, "VCC");
        assert_eq!(pin.Position, 4);
        assert_eq!(pin.Type, "Power");
    }

    #[test]
    fn package_lqfp48() {
        let package_name = "LQFP48";

        let package = Package::new(package_name);

        assert_eq!(false, package.is_grid());
        assert_eq!(48, package.pins());
    }

    #[test]
    fn package_tfbga() {
        let package_name = "TFBGA144";

        let package = Package::new(package_name);

        assert_eq!(true, package.is_grid());
        assert_eq!(144, package.pins());
    }
}
