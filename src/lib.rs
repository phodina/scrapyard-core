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
	Package : String 
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
}
