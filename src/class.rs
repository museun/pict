use std::mem;
use std::ptr;

use winapi::shared::{ntdef, windef};
use winapi::um::{libloaderapi, winuser};

use error::*;

#[derive(Debug)]
pub struct Class(pub ntdef::LPCWSTR);

impl Class {
    pub fn create(name: ntdef::LPCWSTR) -> ntdef::LPCWSTR {
        unsafe {
            let class = winuser::WNDCLASSEXW {
                cbSize: mem::size_of::<winuser::WNDCLASSEXW>() as u32,
                style: winuser::CS_DBLCLKS,
                lpfnWndProc: Some(winuser::DefWindowProcW),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: libloaderapi::GetModuleHandleW(ptr::null_mut()),
                hIcon: ptr::null_mut(),
                hCursor: ptr::null_mut(),
                hbrBackground: (winuser::COLOR_WINDOW + 1) as windef::HBRUSH,
                lpszMenuName: ptr::null_mut(),
                lpszClassName: name,
                hIconSm: ptr::null_mut(),
            };

            Class::register(&class);
            name
        }
    }

    fn register(class: &winuser::WNDCLASSEXW) {
        unsafe {
            let v = winuser::RegisterClassExW(class);
            if v == 0 {
                let err = get_last_windows_error();
                error!("cannot register class: {}", err);
                panic!("invalid state");
            }
        }
    }

    // initialize COM here
}

// impl drop to Unregister Class
