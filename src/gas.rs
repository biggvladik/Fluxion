use libloading::{Library, Symbol};
use libc::{c_char, c_float, c_int};
use std::ffi::CStr;
use std::sync::Arc;

/// Обертка для GAS.dll
pub struct GasApi {
    _lib: Arc<Library>, // библиотека живет столько же, сколько структура

    pub start: Symbol<'static, unsafe extern "system" fn(*mut c_int) -> c_int>,
    pub init: Symbol<'static, unsafe extern "system" fn(*mut c_int) -> c_int>,
    pub get_error: Symbol<'static, unsafe extern "system" fn(*mut c_char)>,
    pub get_warning: Symbol<'static, unsafe extern "system" fn(*mut c_char)>,
    pub get_size: Symbol<'static, unsafe extern "system" fn() -> c_int>,
    pub get_spectr: Symbol<'static, unsafe extern "system" fn(*mut c_float, *mut c_float, *mut c_float)>,

    pub get_value: Symbol<'static, unsafe extern "system" fn(
        *const u16, *mut f64, *mut c_int, *const c_char
    ) -> c_int>,

    pub test: Symbol<'static, unsafe extern "system" fn(
        c_int,
        *mut c_float, *mut c_float, *mut c_float, *mut c_float,
        *mut c_float, *mut c_float, *mut c_float, *mut c_float,
        *mut c_int
    ) -> c_int>,

    pub get_value_file: Symbol<'static, unsafe extern "system" fn(
        *const u16, *const u16, *mut f64, *const c_char
    ) -> c_int>,

    pub load_param: Symbol<'static, unsafe extern "system" fn()>,

    pub get_value_spec: Symbol<'static, unsafe extern "system" fn(
        *mut c_float, *mut c_float, *mut *mut c_float, *mut c_int, *mut c_int
    ) -> c_int>,

    pub get_energy: Symbol<'static, unsafe extern "system" fn(
        c_int, *mut c_float, *mut c_float, *mut *mut c_float, *mut c_int, *mut c_int
    ) -> c_int>,

    pub calc_value: Symbol<'static, unsafe extern "system" fn(
        c_float, c_float, *mut c_float, c_int, *const u16, *mut f64, *const c_char
    ) -> c_int>,
}

impl GasApi {
    /// Создание обертки из уже загруженной библиотеки
    pub fn new(lib: Library) -> Result<Self, Box<dyn std::error::Error>> {
        // Используем Arc, чтобы продлить жизнь библиотеки для всех символов
        let lib = Arc::new(lib);
        let lib_ref: &'static Library = unsafe { std::mem::transmute(lib.as_ref()) };

        unsafe {
            Ok(Self {
                start: lib_ref.get(b"Start")?,
                init: lib_ref.get(b"Init")?,
                get_error: lib_ref.get(b"GetError")?,
                get_warning: lib_ref.get(b"GetWarning")?,
                get_size: lib_ref.get(b"GetSize")?,
                get_spectr: lib_ref.get(b"GetSpectr")?,
                get_value: lib_ref.get(b"GetValue")?,
                test: lib_ref.get(b"Test")?,
                get_value_file: lib_ref.get(b"GetValueFile")?,
                load_param: lib_ref.get(b"LoadParam")?,
                get_value_spec: lib_ref.get(b"getValueSpec")?,
                get_energy: lib_ref.get(b"getEnergy")?,
                calc_value: lib_ref.get(b"CalcValue")?,
                _lib: lib,
            })
        }
    }

    pub fn last_error(&self) -> String {
        let mut buf = [0 as c_char; 128];
        unsafe {
            (self.get_error)(buf.as_mut_ptr());
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    pub fn last_warning(&self) -> String {
        let mut buf = [0 as c_char; 128];
        unsafe {
            (self.get_warning)(buf.as_mut_ptr());
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }
}