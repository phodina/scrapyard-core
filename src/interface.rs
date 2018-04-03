use std::ffi::CStr;
use std::path::Path;
use std::os::raw::c_char;
use std::ptr::null;

use mcu::MCUConf;

pub mod c_api {

    use super::*;

    #[no_mangle]
    pub extern "C" fn mcu_conf_new(path: *const c_char) -> *mut MCUConf {
        let path = unsafe {
            assert!(!path.is_null());
            CStr::from_ptr(path)
        };

        match path.to_str() {
            Ok(path) => {
                /*
                let mcu = Box::new(MCU::new(path));
                match mcu {
                    Ok(mcu) => Box::into_raw(mcu),
                    Err(_) => std::ptr::null
            }*/
                null
            }
            Err(_) => null,
        }
    }

    #[no_mangle]
    pub extern "C" fn mcu_conf_free(ptr: *mut MCUConf) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }
}
