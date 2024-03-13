use std::sync::{Arc, Mutex};

pub const FLAG_RUNAWAY: u8 = 0b00000001;
pub const FLAG_MOUSE_GRAVITY: u8 = 0b00000010;

use winapi::um::winuser::VK_F9;

use crate::links::*;

pub unsafe fn do_loop(flags: Arc<Mutex<u8>>) {
    let (w, h) = screen_dimentions();

    loop {
        let flags = *match flags.as_ref().try_lock() {
            Ok(f) => f,
            Err(_err) => {
                continue;
            }
        };

        if pressing(VK_F9) {
            break;
        }

        if flags & FLAG_RUNAWAY != 0 {
            match Window::current() {
                Some(window) => window_runaway(&window, w, h),
                None => {}
            }
        }

        if flags & FLAG_MOUSE_GRAVITY != 0 {
            match get_cursor_position() {
                Some((x, y)) => {
                    if y < h {
                        set_cursor_position(x, y + 1);
                    }
                }
                None => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

pub fn window_runaway(window: &Window, width: i32, height: i32) {
    if window.title() != "" {
        let (x, y) = match get_cursor_position() {
            Some(xy) => xy,
            None => {
                return;
            }
        };
        let (nx, ny, nw, nh) = calculate_new_position((x, y), (width, height));
        window.resize(nx, ny, nw, nh);
    }
}
