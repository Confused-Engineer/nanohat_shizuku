use nanohat_oled::{Oled, OledResult};
use std::{error::Error, ops::ControlFlow};
use std::thread;
use std::time::Duration;

use gpio::{GpioIn, GpioOut};

const K1: u16 = 0;
const K2: u16 = 2;
const K3: u16 = 3;
fn main() {
    //std::thread::sleep(std::time::Duration::from_secs(5));
    let mut nanopi = NanoPi::new();
    nanopi.start();
}



struct NanoPi
{
    state: AppState,
    k1: std::ops::ControlFlow<(gpio::sysfs::SysFsGpioInput)>,
    k2: std::ops::ControlFlow<(gpio::sysfs::SysFsGpioInput)>,
    k3: std::ops::ControlFlow<(gpio::sysfs::SysFsGpioInput)>,
    screen: nanohat_oled::OledResult<(nanohat_oled::Oled)>,
}

impl NanoPi
{
    fn new() -> Self
    {
        Self {
            state: AppState::Main(Menu::Null),
            k1: init_pin(K1),
            k2: init_pin(K2),
            k3: init_pin(K3),
            screen: init_screen()
        }
    }

    fn start(&mut self)
    {
        loop {
            match self.state {
                AppState::Main(menu) => {
                    if let std::ops::ControlFlow::Continue(screen) = self.screen
                    {
                        
                    }
                },
                AppState::ADB(menu) => todo!(),
                AppState::Shutdown(menu) => todo!(),
            }
        }
    }
}


#[derive(PartialEq, Clone, Copy)]
enum AppState
{
    Main(Menu),
    ADB(Menu),
    Shutdown(Menu),
}

#[derive(PartialEq, Clone, Copy)]
enum Menu
{
    Yes,
    No,
    Null,
}

fn init_pin(pin: u16) -> std::ops::ControlFlow<(gpio::sysfs::SysFsGpioInput)>
{
    let init_pin: gpio::sysfs::SysFsGpioInput = gpio::sysfs::SysFsGpioInput::open(pin)?;
    ControlFlow::Continue(init_pin)
}

fn init_screen() -> nanohat_oled::OledResult<(nanohat_oled::Oled)>
{
    let mut oled = Oled::from_path("/dev/i2c-0")?;
    oled.init()?;
    Ok(oled)
}

