use gpio::GpioIn;


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
    k1: gpio::sysfs::SysFsGpioInput,
    k2: gpio::sysfs::SysFsGpioInput,
    k3: gpio::sysfs::SysFsGpioInput,
    screen: nanohat_oled::Oled,
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
                    let _ = self.screen.put_string("Start: \n k1: adb, k3: shutdown");
                    
                    if let Ok(pushed) = self.k1.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                self.state = AppState::ADB(Menu::Null);
                                debounce();

                            },
                        }
                    }

                    if let Ok(pushed) = self.k2.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                
                            },
                        }
                    }

                    if let Ok(pushed) = self.k3.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                self.state = AppState::Shutdown(Menu::Null);
                                debounce();

                            },
                        }
                    }
                },
                AppState::ADB(menu) => {
                    let _ = self.screen.put_string("Launch adb? \n k1: yes, k3: no");

                    if let Ok(pushed) = self.k1.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                let _ = std::process::Command::new("adb")
                                .args(["shell", "sh", "/sdcard/Android/data/moe.shizuku.privileged.api/start.sh"])
                                .spawn();
                            
                                debounce();

                            },
                        }
                    }

                    if let Ok(pushed) = self.k2.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                
                            },
                        }
                    }

                    if let Ok(pushed) = self.k3.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                self.state = AppState::Main(Menu::Null);
                                debounce();

                            },
                        }
                    }
                },
                AppState::Shutdown(menu) => {
                    let _ = self.screen.put_string("Shutdown? \n k1: yes, k3: no");

                    if let Ok(pushed) = self.k1.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                let _ = std::process::Command::new("shutdown")
                                .arg("now")
                                .spawn();

                                debounce();

                            },
                        }
                    }

                    if let Ok(pushed) = self.k2.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                
                            },
                        }
                    }

                    if let Ok(pushed) = self.k3.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                self.state = AppState::Main(Menu::Null);
                                debounce();

                            },
                        }
                    }
                },
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

fn init_pin(pin: u16) -> gpio::sysfs::SysFsGpioInput
{
    gpio::sysfs::SysFsGpioInput::open(pin).unwrap()
    
}

fn init_screen() -> nanohat_oled::Oled
{
    
    nanohat_oled::Oled::from_path("/dev/i2c-0").unwrap()

}

fn debounce()
{
    std::thread::sleep(std::time::Duration::from_millis(100));
}