mod gas;
mod driver;

use driver::GasDriver;

fn main() {
    let dll_path = "GAS.dll";

    let driver = match GasDriver::new(dll_path) {
        Ok(d) => d,
        Err(e) => {
            println!("DLL load error: {}", e);
            return;
        }
    };

    println!("DLL loaded!");

    match driver.start() {
        Ok(_) => println!("start OK"),
        Err(e) => println!("start error {}", e),
    }

    match driver.init() {
        Ok(_) => println!("init OK"),
        Err(e) => println!("init error {}", e),
    }

    match driver.get_spectrum() {
        Ok(spec) => println!("Spectrum size = {}", spec.len()),
        Err(e) => println!("Spectrum error {}", e),
    }

    println!("Error text = {}", driver.get_error());
}