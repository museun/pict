use std::fmt;
use std::ptr;

use winapi::shared::ntdef;
use winapi::um::{errhandlingapi, winbase};

pub enum Error {
    Win32Error(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Win32Error(e) => {
                let mut msg = vec![0u16; 127]; // fixed length strings in the windows api
                unsafe {
                    winbase::FormatMessageW(
                        winbase::FORMAT_MESSAGE_FROM_SYSTEM
                            | winbase::FORMAT_MESSAGE_IGNORE_INSERTS,
                        ptr::null_mut(),
                        e,
                        u32::from(ntdef::LANG_SYSTEM_DEFAULT),
                        msg.as_mut_ptr(),
                        msg.len() as u32,
                        ptr::null_mut(),
                    );
                }
                let s = String::from_utf16_lossy(&msg);
                writeln!(f, "#{}: {}", e, &s)
            }
           // _ => Ok(()),
        }
    }
}

impl From<u32> for Error {
    fn from(e: u32) -> Self {
        Error::Win32Error(e)
    }
}

pub fn get_last_windows_error() -> u32 {
    unsafe { errhandlingapi::GetLastError() }
}
