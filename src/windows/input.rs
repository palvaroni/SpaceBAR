use std::mem;

use winapi::um::winuser::*;

use crate::VkCodes;

pub fn keypress(virtual_key: &VkCodes) -> u32 {
    return send_input_key(*virtual_key as u16, false) + send_input_key(*virtual_key as u16, true);
}

pub fn send_input_key(virtual_key: u16, up: bool) -> u32 {
    unsafe {
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: std::mem::zeroed(),
        };
        *input.u.ki_mut() = KEYBDINPUT {
            wVk: virtual_key,
            dwFlags: if up { KEYEVENTF_KEYUP } else { 0 },
            dwExtraInfo: 1,
            wScan: 0,
            time: 0,
        };

        return SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    }
}
