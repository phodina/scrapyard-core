extern crate mcu;

use mcu::MCU;
use std::path::Path;

#[test]
fn load_mcu() {
    let mcu = MCU::new(Path::new("samples/STM32F030C6Tx.json")).unwrap();
    let mcu_conf = mcu.finish();

    mcu_conf.get_pins().pins().len();
    mcu_conf.get_peripherals().len();
    assert!(true);
}
