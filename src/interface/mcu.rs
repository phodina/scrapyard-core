use std::ffi::{CStr, CString};
use std::path::Path;
use std::os::raw::c_char;
use std;

use package::Package;
use pins::Pins;
use mcu::{MCUConf, MCU};

#[repr(C)]
pub enum PackageType {
    LQFP,
    TSSOP,
    WLCSP,
    UFQFPN,
    TFBGA,
    VFQFPN,
    EWLCSP,
    UFBGA,
    LFBGA,
    NONE,
}

pub mod cc {

    use super::*;

    #[no_mangle]
    pub extern "C" fn mcu_conf_new(path: *const c_char) -> *mut MCUConf {
        let path = unsafe {
            assert!(!path.is_null());
            CStr::from_ptr(path)
        };

        match path.to_str() {
            Ok(path) => match MCU::new(Path::new(path)) {
                Ok(mcu) => Box::into_raw(Box::new(mcu.finish())),
                Err(_) => std::ptr::null_mut(),
            },
            Err(_) => std::ptr::null_mut(),
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

    #[no_mangle]
    pub extern "C" fn mcu_conf_get_name(ptr: *mut MCUConf) -> *mut c_char {
        let mcu_conf = unsafe {
            assert!(!ptr.is_null());
            &mut *ptr
        };

        let c_str_name = CString::new(mcu_conf.get_name()).unwrap();
        c_str_name.into_raw()
    }

    #[no_mangle]
    pub extern "C" fn mcu_conf_free_name(name: *mut c_char) {
        unsafe {
            if name.is_null() {
                return;
            }
            CString::from_raw(name)
        };
    }

    #[no_mangle]
    pub extern "C" fn mcu_conf_get_pins(ptr: *mut MCUConf) -> *mut Pins {
        let mcu_conf = unsafe {
            assert!(!ptr.is_null());
            &mut *ptr
        };

        mcu_conf.get_pins()
    }

    #[no_mangle]
    pub extern "C" fn mcu_conf_get_package(ptr: *mut MCUConf) -> *mut Package {
        let mcu_conf = unsafe {
            assert!(!ptr.is_null());
            &mut *ptr
        };

        mcu_conf.get_package()
    }

    #[no_mangle]
    pub extern "C" fn package_is_grid(ptr: *mut Package) -> bool {
        let package = unsafe {
            assert!(!ptr.is_null());
            &mut *ptr
        };

        package.is_grid()
    }

    #[no_mangle]
    pub extern "C" fn package_type(ptr: *mut Package) -> PackageType {
        let package = unsafe {
            assert!(!ptr.is_null());
            &mut *ptr
        };

        match *package {
            Package::LQFP(..) => PackageType::LQFP,
            Package::TSSOP(..) => PackageType::TSSOP,
            Package::WLCSP(..) => PackageType::WLCSP,
            Package::UFQFPN(..) => PackageType::UFQFPN,
            Package::TFBGA(..) => PackageType::TFBGA,
            Package::VFQFPN(..) => PackageType::VFQFPN,
            Package::EWLCSP(..) => PackageType::EWLCSP,
            Package::UFBGA(..) => PackageType::UFBGA,
            Package::LFBGA(..) => PackageType::LFBGA,
            Package::Unknown(..) => PackageType::NONE,
        }
    }
}
