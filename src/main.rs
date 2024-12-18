use gpio::GpioIn;
use local_ip_address::linux::local_ip;


const K1: u16 = 0;
const K2: u16 = 2;
const K3: u16 = 3;
fn main() {
    
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
    screen_refresh_required: bool,
}

impl NanoPi
{
    fn new() -> Self
    {
        Self {
            state: AppState::Main,
            k1: init_pin(K1),
            k2: init_pin(K2),
            k3: init_pin(K3),
            screen: init_screen(),
            screen_refresh_required: true
        }
    }

    fn start(&mut self)
    {
        
        while let Err(_) = self.screen.init()
        {
            let _ = self.screen.init();
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
        let _ = self.screen.clear_display();

        let _ = self.screen.put_string("Starting");

        

        if let (Err(_), Ok(ip)) = (std::process::Command::new("adb").spawn(), local_ip())
        {
            let _ = self.screen.put_string(&format!("IP: {}. Installing ADB", ip.to_string()));
            let _ = std::process::Command::new("apt").arg("update").output();
            let _ = std::process::Command::new("apt").args(["install", "-y", "adb"]).spawn();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }


        let _ = self.screen.clear_display();
        loop {
            match self.state {
                AppState::Main => {

                    if self.screen_refresh_required
                    {
                        
                        if let Ok(_) = self.screen.draw_image(include_bytes!("../assets/screen_main.raw"), 1)
                        {
                            self.screen_refresh_required = false;
                        } else {
                            let _ = self.screen.clear_display();
                        }
                    }
                    
                    
                    if let Ok(mut pushed) = self.k1.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                while pushed == gpio::GpioValue::High {
                                    pushed = self.k1.read_value().unwrap();
                                }
                                self.state = AppState::ADB;
                                self.screen_refresh_required = true;
                                let _ = self.screen.clear_display();
                                debounce();

                            },
                        }
                    }

                    if let Ok(mut pushed) = self.k2.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                while pushed == gpio::GpioValue::High {
                                    pushed = self.k2.read_value().unwrap();
                                }
                                self.state = AppState::Info;
                                self.screen_refresh_required = true;
                                let _ = self.screen.clear_display();
                                debounce();

                            },
                        }
                    }

                    if let Ok(mut pushed) = self.k3.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                while pushed == gpio::GpioValue::High {
                                    pushed = self.k3.read_value().unwrap();
                                }
                                self.state = AppState::Shutdown;
                                self.screen_refresh_required = true;
                                let _ = self.screen.clear_display();
                                debounce();

                            },
                        }
                    }
                },
                AppState::ADB => {
                    if self.screen_refresh_required
                    {
                        if let Ok(_) = self.screen.draw_image(include_bytes!("../assets/screen_adb.raw"), 1)
                        {
                            self.screen_refresh_required = false;
                        }
                    }

                    if let Ok(mut pushed) = self.k1.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                while pushed == gpio::GpioValue::High {
                                    pushed = self.k1.read_value().unwrap();
                                }

                                if let Err(_) = std::process::Command::new("adb").args(["shell", "sh", "/sdcard/Android/data/moe.shizuku.privileged.api/start.sh"]).spawn()
                                {
                                    let _ = self.screen.put_string("ADB Err. Click F3 to exit.");
                                }
                            
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

                    if let Ok(mut pushed) = self.k3.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                while pushed == gpio::GpioValue::High {
                                    pushed = self.k3.read_value().unwrap();
                                }
                                self.state = AppState::Main;
                                self.screen_refresh_required = true;
                                let _ = self.screen.clear_display();
                                debounce();

                            },
                        }
                    }
                },
                AppState::Shutdown => {
                    if self.screen_refresh_required
                    {
                        if let Ok(_) = self.screen.draw_image(include_bytes!("../assets/screen_shutdown.raw"), 1)
                        {
                            self.screen_refresh_required = false;
                        }
                    }

                    if let Ok(mut pushed) = self.k1.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                let _ = self.screen.clear_display();
                                let _ = self.screen.put_string("Shutting Down...");
                                while pushed == gpio::GpioValue::High {
                                    pushed = self.k1.read_value().unwrap();
                                }
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

                    if let Ok(mut pushed) = self.k3.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                while pushed == gpio::GpioValue::High {
                                    pushed = self.k3.read_value().unwrap();
                                }
                                self.state = AppState::Main;
                                self.screen_refresh_required = true;
                                let _ = self.screen.clear_display();
                                debounce();

                            },
                        }
                    }
                },
                AppState::Info => {
                    if self.screen_refresh_required
                    {

                        if let Ok(ipv4) = local_ip()
                        {
                            if let Ok(_) = self.screen.put_string(&ipv4.to_string())
                            {
                                self.screen_refresh_required = false;
                            } else {
                                let _ = self.screen.clear_display();
                            }
                        }
                        //if let Ok(_) = self.screen.put_string("Start: k1: adb, k3: shutdown")
                        
                    }
                    
                    
                    if let Ok(mut pushed) = self.k1.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                while pushed == gpio::GpioValue::High {
                                    pushed = self.k1.read_value().unwrap();
                                }
                                self.state = AppState::Main;
                                self.screen_refresh_required = true;
                                let _ = self.screen.clear_display();
                                debounce();

                            },
                        }
                    }

                    if let Ok(mut pushed) = self.k2.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                while pushed == gpio::GpioValue::High {
                                    pushed = self.k2.read_value().unwrap();
                                }
                                self.state = AppState::Main;
                                self.screen_refresh_required = true;
                                let _ = self.screen.clear_display();
                                debounce();

                            },
                        }
                    }

                    if let Ok(mut pushed) = self.k3.read_value()
                    {
                        match pushed {
                            gpio::GpioValue::Low => {
                                
                            },
                            gpio::GpioValue::High => {
                                while pushed == gpio::GpioValue::High {
                                    pushed = self.k3.read_value().unwrap();
                                }
                                self.state = AppState::Main;
                                self.screen_refresh_required = true;
                                let _ = self.screen.clear_display();
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
    Main,
    ADB,
    Info,
    Shutdown,
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
    //std::thread::sleep(std::time::Duration::from_millis(100));
}


#[cfg(test)]
mod tests {


    

    #[test]
    fn image_len() {

        println!("Image Length is: {}", include_bytes!("../assets/screen_main.raw").len());
        
        println!("Needs to be: {}", (128*64))

    }

    #[test]
    fn print_ip() {

        if let Err(_) = std::process::Command::new("adb").spawn()
        {
            let _ = std::process::Command::new("apt").arg("update").output();
            let _ = std::process::Command::new("apt").args(["install","-y","adb"]).output();


        }
        
        

    }

}

