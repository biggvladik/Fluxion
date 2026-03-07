use crate::gas::GasApi;
use std::ffi::{CStr, CString, OsStr};
use std::os::windows::ffi::OsStrExt;

pub struct GasDriver {
    api: GasApi,
}

impl GasDriver {
    pub fn new(dll_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            api: GasApi::new(dll_path)?,
        })
    }

    fn utf16(s: &str) -> Vec<u16> {
        OsStr::new(s)
            .encode_wide()
            .chain(Some(0))
            .collect()
    }

    fn c_string(s: &str) -> CString {
        CString::new(s).unwrap()
    }

    fn read_c_string(ptr: *const i8) -> String {
        if ptr.is_null() {
            return String::new();
        }

        unsafe {
            CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn start(&self) -> Result<(), i32> {
        let mut warning = 0i32;

        let res = unsafe { (self.api.start)(&mut warning) };

        if res != 0 {
            return Err(res);
        }

        Ok(())
    }

    pub fn init(&self) -> Result<(), i32> {
        let mut warning = 0i32;

        let res = unsafe { (self.api.init)(&mut warning) };

        if res != 0 {
            return Err(res);
        }

        Ok(())
    }

    pub fn get_spectrum(&self) -> Result<Vec<f32>, i32> {
        let size = unsafe { (self.api.get_size)() as usize };

        if size == 0 {
            return Ok(Vec::new());
        }

        let mut xo = 0f32;
        let mut xs = 0f32;

        let mut buffer = vec![0f32; size];

        unsafe {
            (self.api.get_spectr)(
                &mut xo,
                &mut xs,
                buffer.as_mut_ptr(),
            );
        }

        Ok(buffer)
    }

    pub fn get_error(&self) -> String {
        let mut buf = [0i8; 128];

        unsafe {
            (self.api.get_error)(buf.as_mut_ptr());
        }

        Self::read_c_string(buf.as_ptr())
    }
}