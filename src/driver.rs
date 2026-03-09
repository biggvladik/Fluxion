use crate::gas::GasApi;
use std::ffi::{CStr, CString, OsStr};
use std::os::windows::ffi::OsStrExt;
use libloading::Library;

pub struct GasDriver {
    api: GasApi,
}

impl GasDriver {
    /// Создание драйвера из DLL
    pub fn new(dll_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let lib = unsafe { Library::new(dll_path)? };
        let api = GasApi::new(lib)?;
        Ok(Self { api })
    }

    /// Конвертация Rust-строки в UTF-16 для Windows API
    fn utf16(s: &str) -> Vec<u16> {
        OsStr::new(s).encode_wide().chain(Some(0)).collect()
    }

    /// Конвертация Rust-строки в C-string
    fn c_string(s: &str) -> CString {
        CString::new(s).unwrap()
    }

    /// Чтение C-string в Rust-строку
    fn read_c_string(ptr: *const i8) -> String {
        if ptr.is_null() {
            return String::new();
        }
        unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
    }

    /// Запуск GAS
    pub fn start(&self) -> Result<(), i32> {
        let mut warning = 0i32;
        let res = unsafe { (self.api.start)(&mut warning) };
        if res != 0 { return Err(res); }
        Ok(())
    }

    /// Инициализация GAS
    pub fn init(&self) -> Result<(), i32> {
        let mut warning = 0i32;
        let res = unsafe { (self.api.init)(&mut warning) };
        if res != 0 { return Err(res); }
        Ok(())
    }

    /// Получение спектра
    pub fn get_spectrum(&self) -> Result<Vec<f32>, i32> {
        let size = unsafe { (self.api.get_size)() as usize };
        if size == 0 { return Ok(Vec::new()); }

        let mut xo = 0f32;
        let mut xs = 0f32;
        let mut buffer = vec![0f32; size];

        unsafe {
            (self.api.get_spectr)(&mut xo, &mut xs, buffer.as_mut_ptr());
        }

        Ok(buffer)
    }

    /// Получение последней ошибки
    pub fn get_error(&self) -> String {
        self.api.last_error()
    }

    /// Получение последнего предупреждения
    pub fn get_warning(&self) -> String {
        self.api.last_warning()
    }

    /// Обёртка для get_value
    pub fn get_value(&self, param: &str, extra: &str) -> Result<f64, i32> {
        let mut value = 0f64;
        let mut flag = 0i32;
        let param_utf16 = Self::utf16(param);
        let extra_c = Self::c_string(extra);

        let res = unsafe {
            (self.api.get_value)(
                param_utf16.as_ptr(),
                &mut value,
                &mut flag,
                extra_c.as_ptr()
            )
        };

        if res != 0 { return Err(res); }
        Ok(value)
    }

    /// Обёртка для get_value_file
    pub fn get_value_file(&self, file: &str, param: &str) -> Result<f64, i32> {
        let mut value = 0f64;
        let file_utf16 = Self::utf16(file);
        let param_utf16 = Self::utf16(param);
        let res = unsafe {
            (self.api.get_value_file)(
                file_utf16.as_ptr(),
                param_utf16.as_ptr(),
                &mut value,
                std::ptr::null()
            )
        };
        if res != 0 { return Err(res); }
        Ok(value)
    }

    /// Загрузка параметров
    pub fn load_param(&self) {
        unsafe { (self.api.load_param)() }
    }

    /// Обёртка для get_value_spec
    pub fn get_value_spec(&self) -> Result<(Vec<f32>, Vec<f32>), i32> {
        let mut xo = 0f32;
        let mut xs = 0f32;
        let mut ptr: *mut f32 = std::ptr::null_mut();
        let mut size = 0i32;
        let mut flag = 0i32;

        let res = unsafe {
            (self.api.get_value_spec)(&mut xo, &mut xs, &mut ptr, &mut size, &mut flag)
        };

        if res != 0 || ptr.is_null() || size <= 0 { return Err(res); }

        let slice = unsafe { std::slice::from_raw_parts(ptr, size as usize) };
        Ok((slice.to_vec(), vec![xo, xs]))
    }

    /// Обёртка для get_energy
    pub fn get_energy(&self, index: i32) -> Result<(Vec<f32>, Vec<f32>), i32> {
        let mut xo = 0f32;
        let mut xs = 0f32;
        let mut ptr: *mut f32 = std::ptr::null_mut();
        let mut size = 0i32;
        let mut flag = 0i32;

        let res = unsafe {
            (self.api.get_energy)(index, &mut xo, &mut xs, &mut ptr, &mut size, &mut flag)
        };

        if res != 0 || ptr.is_null() || size <= 0 { return Err(res); }

        let slice = unsafe { std::slice::from_raw_parts(ptr, size as usize) };
        Ok((slice.to_vec(), vec![xo, xs]))
    }

    /// Обёртка для calc_value
    pub fn calc_value(&self, x: f32, y: f32, param: &str) -> Result<f64, i32> {
        let mut value = 0f64;
        let param_utf16 = Self::utf16(param);

        let res = unsafe {
            (self.api.calc_value)(x, y, std::ptr::null_mut(), 0, param_utf16.as_ptr(), &mut value, std::ptr::null())
        };

        if res != 0 { return Err(res); }
        Ok(value)
    }

    /// Обёртка для test
    pub fn test(&self, index: i32) -> Result<(), i32> {
        let mut a = 0f32;
        let mut b = 0f32;
        let mut c = 0f32;
        let mut d = 0f32;
        let mut e = 0f32;
        let mut f = 0f32;
        let mut g = 0f32;
        let mut h = 0f32;
        let mut res_flag = 0i32;

        let res = unsafe {
            (self.api.test)(
                index,
                &mut a, &mut b, &mut c, &mut d,
                &mut e, &mut f, &mut g, &mut h,
                &mut res_flag
            )
        };

        if res != 0 { return Err(res); }
        Ok(())
    }
}