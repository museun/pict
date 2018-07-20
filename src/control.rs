#![allow(dead_code)]
use std::mem;
use std::ptr;

use winapi::shared::{minwindef, ntdef, windef};
use winapi::um::{commctrl, winuser};

use error::*;
use util::*;

// TODO: determine if we should wrap the HWND so we can have "methods" on it (like SendMessage, ShowWindow)

type Handler = unsafe extern "system" fn(
    windef::HWND,
    msg: minwindef::UINT,
    wp: minwindef::WPARAM,
    lp: minwindef::LPARAM,
    id: usize,
    data: usize,
) -> minwindef::LRESULT;

// maybe use a builder here
#[allow(too_many_arguments)]
pub fn create_control_hwnd(
    name: &str,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    parent: windef::HWND,
    ex_style: minwindef::DWORD,
    class_name: ntdef::LPCWSTR,
    control_name: ntdef::LPCWSTR,
    style: minwindef::DWORD,
    param: minwindef::LPVOID,
    handler: Option<Handler>,
) -> (windef::HWND, usize) {
    unsafe {
        let mut style = style;
        if (style & winuser::WS_TABSTOP) != 0 {
            style |= winuser::WS_GROUP
        }

        let hwnd = winuser::CreateWindowExW(
            ex_style,
            class_name,
            control_name,
            style | winuser::WS_CHILD | winuser::WS_VISIBLE,
            x,
            y,
            w,
            h,
            parent,
            ptr::null_mut(),
            hinstance(),
            param,
        );

        if hwnd.is_null() {
            let err = get_last_windows_error();
            error!("cannot create window: {}: {}", name, err);
        }
        // why is this null
        assert!(!hwnd.is_null(), "cannot make {} window", name);

        let subclass_id = generate_id(class_name);
        commctrl::SetWindowSubclass(hwnd, handler, subclass_id, param as usize);
        // maybe set the default font, etc here
        (hwnd, subclass_id)
    }
}

// maybe this can be used in a RAII fashion. I'm not sure when window destruction will be used. better be explicit for now.
// although I guess we don't need to destroy the window if we'll never be dynamically creeating controls
pub fn destroy_hwnd(hwnd: windef::HWND, subclass_id: usize, handler: Option<Handler>) {
    unsafe {
        if subclass_id != 0 {
            commctrl::RemoveWindowSubclass(hwnd, handler, subclass_id);
        }
        if winuser::DestroyWindow(hwnd) == 0 && winuser::IsWindow(hwnd) > 0 {
            let err = get_last_windows_error();
            error!(
                "cannot destroy subclass: {:?}, {}: {}",
                hwnd,
                subclass_id,
                Error::from(err)
            )
        }
    }
}

fn generate_id(s: ntdef::LPCWSTR) -> usize {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    let mut hasher = DefaultHasher::new();
    hasher.write_usize(s as usize); // hash the pointer of the LPCWSTR
    hasher.finish() as usize
}

#[inline]
unsafe fn cast_hwnd<'a, T>(hwnd: windef::HWND) -> &'a mut T
where
    T: Sized,
{
    let p = winuser::GetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA);
    // XXX: is this "Unsound Rust"?
    mem::transmute(p as *mut ntdef::PVOID)
}

#[inline]
/// (this gets the client rect)
pub unsafe fn window_rect(hwnd: windef::HWND) -> windef::RECT {
    let mut rect: windef::RECT = mem::zeroed();
    winuser::GetClientRect(hwnd, &mut rect);
    rect
}

#[inline]
pub unsafe fn show_window(hwnd: windef::HWND) {
    winuser::ShowWindow(hwnd, winuser::SW_SHOW);
}

#[inline]
pub unsafe fn hide_window(hwnd: windef::HWND) {
    winuser::ShowWindow(hwnd, winuser::SW_HIDE);
}
