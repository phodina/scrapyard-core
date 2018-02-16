#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct MCUBuilder {
	Flash : u32,
	Eeprom : u32,
	Ram: u32,
	Frequency : u32,
	IOs : u16,
	Core : String,
	Name : String,
	Line : Option<String>,
	Family : Option<String>,
	Package : Option<String>,
	IPs: Option<Vec<IP>>,
	Pins: Option<Vec<Pin>>
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
	Position : String,
	Type : String
}

#[derive(Serialize, Deserialize, Debug)]
struct MCUConf {
	flash : u32,
	eeprom : u32,
	ram: u32,
	frequency : u32,
	iOs : u16,
	core : String,
	name : String,
	package : Option<String>,
	periherals: Option<Vec<String>>,
	middlewares: Option<Vec<String>>,
	components: Option<Vec<String>>,
}

enum MCU {
    STM32, //(MCUConf),
    STM8, //(MCUConf),
    ATMEGA,// (MCUConf),
    MSP430,// (MCUConf)
}

impl MCU {

	fn create_peripherals (&self) {
/*
		for peripheral in self.IPs {
		match self {
		    MCU::STM32{} => println!("STM32"),
		    _ => println!("OTHER"),
		}	
		}
*/		
	}
}

impl <'a>MCUBuilder {

	fn new(cfg_file: &str) -> Option<MCU> {
		// TODO: Return Enum MCU or error
    	let mcu_builder : MCUBuilder = serde_json::from_str(cfg_file).unwrap();
    	
    	let mut mcu = if mcu_builder.Name.starts_with("STM32") {

    		MCU::STM32
	    	}
	    else if mcu_builder.Name.starts_with("STM8") {

	    	MCU::STM8
	    	}
	    else if mcu_builder.Name.starts_with("ATMEGA") {

	    	MCU::ATMEGA
	    	}
	    else if mcu_builder.Name.starts_with("MSP430") {

    		MCU::MSP430
	    	}
	    else {
	    	    
    		return None;
	    	};

	    mcu.create_peripherals();
	    Some(mcu)
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

    #[test]
    fn mcubuilder_ok() {
    	let json = r#"{ "Core" : "ARM Cortex-M0",
    					"Eeprom" : 0,
    					"Family" : "STM32F0",
    					"Flash" : 32,
    					"Frequency" : 48,
    					"IOs" : 39,
    					"Line" : "STM32F0x0 Value Line",
    					"Name" : "STM32F030C6Tx",
    					"Package" : "LQFP48",
    					"Ram" : 4 }"#;
    	
    	let mcu_builder : MCUBuilder = serde_json::from_str(json).unwrap();

    	assert_eq!(mcu_builder.Core, "ARM Cortex-M0");
    	assert_eq!(mcu_builder.Eeprom, 0);
    	assert_eq!(mcu_builder.Family, "STM32F0");
    	assert_eq!(mcu_builder.Flash, 32);
    	assert_eq!(mcu_builder.Frequency, 48);
    	assert_eq!(mcu_builder.IOs, 39);
    	assert_eq!(mcu_builder.Line, "STM32F0x0 Value Line");
    	assert_eq!(mcu_builder.Name, "STM32F030C6Tx");
    	assert_eq!(mcu_builder.Package, "LQFP48");
    	assert_eq!(mcu_builder.Ram, 4);
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

    	let json = r#"{ "Name" : "adc.conf",
    					"Position" : "4",
    					"Type" : "Power"}"#;
    	
    	let pin : Pin = serde_json::from_str(json).unwrap();

    	assert_eq!(pin.Name, "adc.conf");
    	assert_eq!(pin.Position, "4");
    	assert_eq!(pin.Type, "Power");
    }
}
