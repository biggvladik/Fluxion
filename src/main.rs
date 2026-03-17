mod gas;
mod driver;

use driver::GasDriver;

fn main() {
    let dll_path = "GAS.dll";

    // Загружаем DLL и создаём драйвер
    let driver = match GasDriver::new(dll_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to load DLL: {}", e);
            return;
        }
    };

    println!("DLL loaded successfully!");

    // Загрузка параметров
    driver.load_param();


    // Запуск GAS
    match driver.start() {
        Ok(_) => println!("GAS start OK"),
        Err(code) => eprintln!("GAS start error, code = {}", code),
    }

    // Инициализация GAS
    match driver.init() {
        Ok(_) => println!("GAS init OK"),
        Err(code) => eprintln!("GAS init error, code = {}", code),
    }

    // Получение спектра
    match driver.get_spectrum() {
        Ok(spec) => println!("Spectrum retrieved, size = {}", spec.len()),
        Err(code) => eprintln!("Failed to get spectrum, code = {}", code),
    }

    // Получение последней ошибки и предупреждения
    let last_error = driver.get_error();
    let last_warning = driver.get_warning();

    if !last_error.is_empty() {
        println!("Last error: {}", last_error);
    } else {
        println!("No errors reported.");
    }

    if !last_warning.is_empty() {
        println!("Last warning: {}", last_warning);
    } else {
        println!("No warnings reported.");
    }
}