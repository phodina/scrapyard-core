use std::ffi::{CStr, CString};
use std::path::Path;
use std::os::raw::c_char;
use std;
use libc;

use pins::Pins;
use pin::Pin;

pub mod cc {

    use super::*;

    #[no_mangle]
    pub extern "C" fn pins_get_size(ptr: *mut Pins) -> libc::uint32_t {
        let pins = unsafe {
            assert!(!ptr.is_null());
            &mut *ptr
        };

        pins.pins.len() as libc::uint32_t
    }
    /*
    #[no_mangle]
    pub extern "C" fn pins_get_pin(ptr: *mut Pins, idx: usize) -> *mut Pin {
        let pins = unsafe {
            assert!(!ptr.is_null());
            &mut *ptr
        };
        
        match pins.pins().get(idx) {
            Some(pin) => pin,
            None => std::ptr::null(),
        }
    }
*/

    #[no_mangle]
    pub extern "C" fn pins_find_pin(ptr: *mut Pins, pin_name: *const c_char) {
        let pins = unsafe {
            assert!(!ptr.is_null());
            &mut *ptr
        };

        let pin_str = unsafe {
            assert!(!pin_name.is_null());
            CStr::from_ptr(pin_name)
        };

        pins.find_pin(pin_str.to_str().unwrap());
    }
}
