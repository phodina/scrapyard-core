#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct MCUConf {
	Flash : u32,
	Eeprom : u32,
	Ram: u32,
	Frequency : u32,
	IOs : u16,
	Core : String,
	Name : String,
	Line : String,
	Family : String,
	Package : String,
	IPs: Option<Vec<IP>>,
	Pins: Option<Vec<Pin>>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct IP {
	ConfigFile : String,
	Name : String
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Pin {
	Name : String,
	Position : String,
	Type : String
}


#[cfg(test)]
mod tests {

	use super::*;

    #[test]
    fn mcuconf_ok() {
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
    	
    	let mcu_conf : MCUConf = serde_json::from_str(json).unwrap();

    	assert_eq!(mcu_conf.Core, "ARM Cortex-M0");
    	assert_eq!(mcu_conf.Eeprom, 0);
    	assert_eq!(mcu_conf.Family, "STM32F0");
    	assert_eq!(mcu_conf.Flash, 32);
    	assert_eq!(mcu_conf.Frequency, 48);
    	assert_eq!(mcu_conf.IOs, 39);
    	assert_eq!(mcu_conf.Line, "STM32F0x0 Value Line");
    	assert_eq!(mcu_conf.Name, "STM32F030C6Tx");
    	assert_eq!(mcu_conf.Package, "LQFP48");
    	assert_eq!(mcu_conf.Ram, 4);
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
