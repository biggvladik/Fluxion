use libloading::{Library, Symbol};
use libc::{c_char, c_float, c_int, c_double};
use std::mem::transmute;

pub struct GasApi {
    _lib: Box<Library>,

    pub start: Symbol<'static, unsafe extern "stdcall" fn(*mut c_int) -> c_int>,
    pub init: Symbol<'static, unsafe extern "stdcall" fn(*mut c_int) -> c_int>,
    pub get_error: Symbol<'static, unsafe extern "stdcall" fn(*mut c_char)>,
    pub get_warning: Symbol<'static, unsafe extern "stdcall" fn(*mut c_char)>,
    pub get_size: Symbol<'static, unsafe extern "stdcall" fn() -> c_int>,
    pub get_spectr: Symbol<'static, unsafe extern "stdcall" fn(
        *mut c_float,
        *mut c_float,
        *mut c_float,
    )>,
}

impl GasApi {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {

        let lib = Box::new(unsafe { Library::new(path)? });

        unsafe {

            let start: Symbol<unsafe extern "stdcall" fn(*mut c_int) -> c_int> =
                lib.get(b"Start")?;

            let start = transmute(start);

            let init: Symbol<unsafe extern "stdcall" fn(*mut c_int) -> c_int> =
                lib.get(b"Init")?;
            let init = transmute(init);

            let get_error: Symbol<unsafe extern "stdcall" fn(*mut c_char)> =
                lib.get(b"GetError")?;
            let get_error = transmute(get_error);

            let get_warning: Symbol<unsafe extern "stdcall" fn(*mut c_char)> =
                lib.get(b"GetWarning")?;
            let get_warning = transmute(get_warning);

            let get_size: Symbol<unsafe extern "stdcall" fn() -> c_int> =
                lib.get(b"GetSize")?;
            let get_size = transmute(get_size);

            let get_spectr: Symbol<unsafe extern "stdcall" fn(
                *mut c_float,
                *mut c_float,
                *mut c_float,
            )> = lib.get(b"GetSpectr")?;
            let get_spectr = transmute(get_spectr);

            Ok(Self {
                _lib: lib,
                start,
                init,
                get_error,
                get_warning,
                get_size,
                get_spectr,
            })
        }
    }
}