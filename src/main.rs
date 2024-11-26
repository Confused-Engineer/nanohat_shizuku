use nanohat_oled::{Oled, OledResult};
use std::error::Error;
use std::thread;
use std::time::Duration;

use gpio::{GpioIn, GpioOut};

const K1: u16 = 0;
const K2: u16 = 2;
const K3: u16 = 3;
fn main() {
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("write hello wrld");
    hello_wrld();
    println!("read pins");
    get_pins();
}



fn hello_wrld() -> OledResult {
    let mut oled = Oled::from_path("/dev/i2c-0")?;
    oled.init()?;
    oled.put_string("Hello, World!")?;
    Ok(())
}



fn get_pins()  {

    let mut k1 = gpio::sysfs::SysFsGpioInput::open(K1).unwrap();
    let mut k2 = gpio::sysfs::SysFsGpioInput::open(K2).unwrap();
    let mut k3 = gpio::sysfs::SysFsGpioInput::open(K3).unwrap();
    
    println!("K1: {:?}", k1.read_value().unwrap());
    println!("K2: {:?}", k2.read_value().unwrap());
    println!("K3: {:?}", k3.read_value().unwrap());

    // Blink the LED by setting the pin's logic level high for 500 ms.
    
}