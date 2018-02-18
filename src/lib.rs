#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::path::Path;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct MCU {
    Flash : u32,
    Eeprom : u32,
    Ram: u32,
    Frequency : u32,
    IOs : u16,
    Core : String,
    Name : String,
    Line : Option<String>,
    Family : Option<String>,
    Package : String,
    #[serde(rename="IP")]
    IPs: Vec<IP>,
    #[serde(rename="Pin")]
    Pins: Vec<Pin>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct MCUBuilder {
    Mcu: MCU
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct IP {
	ConfigFile : String,
	Name : String
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Pin {
	Name : String,
	Position : u16,
	Type : String
}

#[derive(Serialize, Deserialize, Debug)]
struct MCUConf {
	flash : u32,
	eeprom : u32,
	ram: u32,
	frequency : u32,
	ios : u16,
	core : String,
	name : String,
	package : String,
	periherals: Option<Vec<String>>,
	middlewares: Option<Vec<String>>,
	components: Option<Vec<String>>,
}

impl <'a>MCUBuilder {

    pub fn new(path: &Path) -> Result<MCUBuilder, Box<Error>> {
        let file = File::open(path)?;
        let mcu_builder = serde_json::from_reader(file)?;

        Ok(mcu_builder)
    }

    fn process_peripherals(&self) {

         for ip in &self.Mcu.IPs {
            println!("Peripheral: {}", ip.Name);
        }

    }
    
    fn process_pins (&self) {

         for pin in &self.Mcu.Pins {
            println!("Pin: {}", pin.Name);
        }
    }

    fn process_package (&self) {

        match self.Mcu.Package.as_ref() {
            "UFBGA176" => println!("count: {}", 201),
            _ => println!("count: DIY")
        }
    }
    
    pub fn finish (self) -> MCUConf {

        self.process_peripherals();
        self.process_pins();
        
         MCUConf {
            flash : self.Mcu.Flash,
	    eeprom : self.Mcu.Eeprom,
	    ram: self.Mcu.Ram,
	    frequency : self.Mcu.Frequency,
	    ios : self.Mcu.IOs,
	    core : self.Mcu.Core,
	    name : self.Mcu.Name,
	    package : self.Mcu.Package,
	    periherals: None,
	    middlewares: None,
	    components: None
        }
    }

    fn create_peripheral(&mut self, name: &'a str) {

    }

    fn create_middleware(&mut self, name: &'a str) {

    }

    fn create_component(&mut self, name: &'a str) {

    }
}


#[cfg(test)]
mod tests {

    use super::*;

    // TODO: Test no file
    // TODO: Test json error
    #[test]
    fn mcubuilder_load() {

        let sample = Path::new("./samples/STM32F030C6Tx.json");
        let mcubuilder = MCUBuilder::new(sample).unwrap();

    	assert_eq!(mcubuilder.Mcu.Core, "ARM Cortex-M0");
    	assert_eq!(mcubuilder.Mcu.Eeprom, 0);
    	assert_eq!(mcubuilder.Mcu.Flash, 32);
    	assert_eq!(mcubuilder.Mcu.Frequency, 48);
    	assert_eq!(mcubuilder.Mcu.Name, "STM32F030C6Tx");
    	assert_eq!(mcubuilder.Mcu.Package, "LQFP48");
    	assert_eq!(mcubuilder.Mcu.Ram, 4);
        assert_eq!(mcubuilder.Mcu.IPs.len(), 20);
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
    	assert_eq!(mcu_conf.package, "LQFP48");
    	assert_eq!(mcu_conf.ram, 4);
        assert_eq!(mcu_conf.periherals.is_some(), false);
    }

    #[test]
    fn pin_ok() {

    	let json = r#"{ "ConfigFile" : "adc.conf",
    					"Name" : "ADC"}"#;
    	
    	let ip : IP = serde_json::from_str(json).unwrap();

    	assert_eq!(ip.ConfigFile, "adc.conf");
    	assert_eq!(ip.Name, "ADC");
    }

    #[test]
    fn ip_ok() {

    	let json = r#"{ "Name" : "VCC",
    					"Position" : 4,
    					"Type" : "Power"}"#;
    	
    	let pin : Pin = serde_json::from_str(json).unwrap();

    	assert_eq!(pin.Name, "VCC");
    	assert_eq!(pin.Position, 4);
    	assert_eq!(pin.Type, "Power");
    }
}
