use nanohat_oled::{Oled, OledResult};
use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

const K1: u8 = 0;
const K2: u8 = 2;
const K3: u8 = 3;
fn main() {
    std::thread::sleep(std::time::Duration::from_secs(5));
    hello_wrld();
    get_pins();
}



fn hello_wrld() -> OledResult {
    let mut oled = Oled::from_path("/dev/i2c-0")?;
    oled.init()?;
    oled.put_string("Hello, world!")?;
    Ok(())
}



fn get_pins() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut k1 = Gpio::new()?.get(K1)?.into_input();
    let mut k2 = Gpio::new()?.get(K2)?.into_input();
    let mut k3 = Gpio::new()?.get(K3)?.into_input();
    
    println!("K1: {}", k1.read().to_string());
    println!("K2: {}", k2.read().to_string());
    println!("K3: {}", k3.read().to_string());

    // Blink the LED by setting the pin's logic level high for 500 ms.
    

    Ok(())
}