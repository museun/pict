use std::{mem, ptr};

use common::*;

#[derive(Debug)]
pub struct Class(pub ntdef::LPCWSTR);

impl Class {
    pub fn create(name: ntdef::LPCWSTR) {
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
}

const WINDOW_SUBCLASS_ID: basetsd::UINT_PTR = 0;
pub fn subclass_window(hwnd: windef::HWND) {
    let ptr = Box::into_raw(Box::new(App::handle));
    let res = unsafe {
        commctrl::SetWindowSubclass(
            hwnd,
            Some(callback),
            WINDOW_SUBCLASS_ID,
            ptr as basetsd::DWORD_PTR,
        )
    };
    assert_eq!(res, 1);
}

// impl drop to Unregister Class
