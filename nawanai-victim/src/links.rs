use winapi::{
    shared::{
        ntdef::HANDLE,
        windef::{HWND, POINT},
    },
    um::{
        handleapi::CloseHandle,
        winuser::{
            GetAsyncKeyState, GetCursorPos, GetForegroundWindow, GetSystemMetrics,
            GetWindowTextLengthW, GetWindowTextW, MoveWindow, SetCursorPos,
        },
    },
};
pub struct Window {
    pub handle: HWND,
}

impl Window {
    pub fn current() -> Option<Self> {
        unsafe {
            let handle = GetForegroundWindow();
            if handle.is_null() {
                None
            } else {
                Some(Self { handle: handle })
            }
        }
    }
    pub fn title(&self) -> String {
        unsafe {
            let handle = self.handle as isize as HWND;
            let len = GetWindowTextLengthW(handle);
            if len == 0 {
                return String::new();
            }

            let mut buffer: Vec<u16> = vec![0; (len + 1) as usize];
            GetWindowTextW(handle, buffer.as_mut_ptr(), len + 1);
            let title = String::from_utf16_lossy(&buffer);
            title.trim_end_matches('\0').to_string()
        }
    }
    pub fn resize(&self, x: i32, y: i32, width: i32, height: i32) -> bool {
        unsafe { MoveWindow(self.handle, x, y, width, height, 1) == 0 }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle as isize as HANDLE);
        }
    }
}

pub fn get_cursor_position() -> Option<(i32, i32)> {
    unsafe {
        let mut point: POINT = std::mem::zeroed();
        if GetCursorPos(&mut point as *mut POINT) != 0 {
            Some((point.x, point.y))
        } else {
            None
        }
    }
}

pub fn set_cursor_position(x: i32, y: i32) -> Option<()> {
    unsafe {
        if SetCursorPos(x, y) != 0 {
            Some(())
        } else {
            None
        }
    }
}

pub fn screen_dimentions() -> (i32, i32) {
    let w = unsafe { GetSystemMetrics(winapi::um::winuser::SM_CXFULLSCREEN) };
    let h = unsafe { GetSystemMetrics(winapi::um::winuser::SM_CYFULLSCREEN) };
    (w, h)
}

pub fn calculate_new_position(
    cursor_pos: (i32, i32),
    screen_dimentions: (i32, i32),
) -> (i32, i32, i32, i32) {
    let (cursor_x, cursor_y) = cursor_pos;
    let (screen_w, screen_h) = screen_dimentions;

    let factor = get_factor(cursor_x, cursor_y, screen_w, screen_h);
    let new_win_h = ((screen_h / 2) as f32 * (factor / 2.0)) as i32 + 20;
    let new_win_w = ((screen_w / 2) as f32 * (factor / 2.0)) as i32;
    let (new_win_x, new_win_y) = calculate_position(
        cursor_x, cursor_y, screen_w, screen_h, new_win_w, new_win_h, factor,
    );

    (new_win_x, new_win_y, new_win_w, new_win_h)
}

fn get_factor(x: i32, y: i32, screen_width: i32, screen_height: i32) -> f32 {
    let cx = screen_width / 2;
    let cy = screen_height / 2;
    let distance_from_center = ((x - cx).pow(2) + (y - cy).pow(2)) as f32;
    let max_distance_from_center_to_corner = (cx.pow(2) + cy.pow(2)) as f32;
    let factor_multiplier =
        1.0 - (distance_from_center.sqrt() / max_distance_from_center_to_corner.sqrt());
    factor_multiplier
}

fn calculate_position(
    cursor_x: i32,
    cursor_y: i32,
    monitor_width: i32,
    monitor_height: i32,
    window_width: i32,
    window_height: i32,
    factor: f32,
) -> (i32, i32) {
    let half_window_width = window_width / 2;
    let half_window_height = window_height / 2;

    let (mut new_x, mut new_y) = (cursor_x, cursor_y);

    // Calculate the safe boundaries for the window
    let safe_left = half_window_width;
    let safe_right = monitor_width - half_window_width;
    let safe_top = half_window_height;
    let safe_bottom = monitor_height - half_window_height;

    // Check if the cursor is within the bounds of the window
    let cursor_in_bounds = cursor_x >= new_x - half_window_width
        && cursor_x <= new_x + half_window_width
        && cursor_y >= new_y - half_window_height
        && cursor_y <= new_y + half_window_height;

    if cursor_in_bounds {
        // Calculate the distance of the cursor from the center of the window
        let distance_x = (cursor_x - new_x).abs();
        let distance_y = (cursor_y - new_y).abs();

        // Calculate the factor to adjust the new position based on the distance from the center
        let factor_x = ((half_window_width as f32 - distance_x as f32) / half_window_width as f32)
            .clamp(0.0, 1.0);
        let factor_y = ((half_window_height as f32 - distance_y as f32)
            / half_window_height as f32)
            .clamp(0.0, 1.0);

        // Adjust the new position based on the cursor's position
        if cursor_x < new_x && cursor_y < new_y {
            new_x = new_x.max(safe_left + (factor_x * factor * half_window_width as f32) as i32);
            new_y = new_y.max(safe_top + (factor_y * factor * half_window_height as f32) as i32);
        } else if cursor_x > new_x && cursor_y < new_y {
            new_x = new_x.min(safe_right - (factor_x * factor * half_window_width as f32) as i32);
            new_y = new_y.max(safe_top + (factor_y * factor * half_window_height as f32) as i32);
        } else if cursor_x < new_x && cursor_y > new_y {
            new_x = new_x.max(safe_left + (factor_x * factor * half_window_width as f32) as i32);
            new_y = new_y.min(safe_bottom - (factor_y * factor * half_window_height as f32) as i32);
        } else if cursor_x > new_x && cursor_y > new_y {
            new_x = new_x.min(safe_right - (factor_x * factor * half_window_width as f32) as i32);
            new_y = new_y.min(safe_bottom - (factor_y * factor * half_window_height as f32) as i32);
        }
    }

    (new_x, new_y)
}

pub fn pressing(key: i32) -> bool {
    unsafe { GetAsyncKeyState(key) == 1 }
}
