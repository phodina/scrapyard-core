extern crate mcu;

#[cfg(feature = "cc")]
use std::ffi::CString;
#[cfg(feature = "cc")]
use mcu::interface::mcu::cc::*;

#[cfg(feature = "cc")]
#[test]
fn get_mcu_name() {
    let mut mcu_conf = mcu_conf_new(
        CString::new("samples/STM32F030C6Tx.json")
            .unwrap()
            .into_raw(),
    );

    let mut name = mcu_conf_get_name(mcu_conf);
    unsafe {
        assert_eq!(
            CString::new("STM32F030C6Tx").unwrap(),
            CString::from_raw(name)
        );
    }

    mcu_conf_free(mcu_conf);
}

#[cfg(feature = "cc")]
#[test]
fn get_pins() {
    let mut mcu_conf = mcu_conf_new(
        CString::new("samples/STM32F030C6Tx.json")
            .unwrap()
            .into_raw(),
    );

    let mut name = mcu_conf_get_pins(mcu_conf);

    mcu_conf_free(mcu_conf);
}
