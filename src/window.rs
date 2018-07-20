use std::mem;
use std::ptr;

use winapi::shared::{minwindef, ntdef, windef};
use winapi::um::winuser;

use util::*;

#[derive(Debug)]
pub struct Window {
    hwnd: windef::HWND,
}

impl Window {
    pub fn new(params: Params) -> Self {
        unsafe {
            let hwnd = winuser::CreateWindowExW(
                params.ex_style,
                params.class_name,
                params.window_name,
                params.style,
                params.x,
                params.y,
                params.width,
                params.height,
                params.parent,
                params.menu,
                hinstance(),
                params.lp_param,
            );
            assert!(!hwnd.is_null());

            Self { hwnd }
        }
    }

    pub fn hwnd(&self) -> windef::HWND {
        self.hwnd
    }

    pub fn post_message(&self, msg: u32, wp: minwindef::WPARAM) {
        unsafe { winuser::SendMessageW(self.hwnd, msg, wp, 0) };
    }

    pub fn send_message(&self, msg: u32, wp: minwindef::WPARAM, lp: minwindef::LPARAM) {
        unsafe { winuser::SendMessageW(self.hwnd, msg, wp, lp) };
    }

    // TODO need to figure out which monitor is the main monitor then to calculate the offset for the position
    pub fn set_position(&self, x: i32, y: i32, cx: i32, cy: i32, flags: u32) {
        unsafe { winuser::SetWindowPos(self.hwnd, ptr::null_mut(), x, y, cx, cy, flags) };
    }

    // x y w h
    pub fn get_position(&self) -> (i32, i32, i32, i32) {
        unsafe {
            let mut rect: windef::RECT = mem::zeroed();
            winuser::GetWindowRect(self.hwnd, &mut rect);
            (rect.left, rect.top, rect.right, rect.bottom)
        }
    }

    // XXX: this does window size, not the client size
    pub fn set_size(&self, w: i32, h: i32) {
        unsafe {
            let mut rect: windef::RECT = mem::zeroed();
            winuser::GetWindowRect(self.hwnd, &mut rect);

            let sz = windef::RECT {
                top: rect.top,
                left: rect.left,
                bottom: w,
                right: h,
            };

            winuser::SetWindowPos(
                self.hwnd,
                winuser::HWND_NOTOPMOST,
                sz.left,
                sz.top,
                sz.right,
                sz.bottom,
                winuser::SWP_NOACTIVATE,
            );

            // AdjustWindowRectEx
            // SetWindowPos
            // UpdateWindow // maybe don't do this yet
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        unsafe {
            let mut rect: windef::RECT = mem::zeroed();
            winuser::GetClientRect(self.hwnd, &mut rect);
            (
                (rect.right - rect.left) as u32,
                (rect.bottom - rect.top) as u32,
            )
        }
    }

    pub fn show(&self) {
        unsafe { winuser::ShowWindow(self.hwnd, winuser::SW_SHOW) };
    }

    pub fn hide(&self) {
        unsafe { winuser::ShowWindow(self.hwnd, winuser::SW_HIDE) };
    }

    // TODO add update and get_rect calculations
}

#[derive(TypedBuilder, Debug)]
pub struct Params {
    #[default = "0"]
    ex_style: minwindef::DWORD,

    class_name: ntdef::LPCWSTR,
    window_name: ntdef::LPCWSTR,

    style: minwindef::DWORD,

    #[default = "0"]
    x: i32,
    #[default = "0"]
    y: i32,
    #[default = "winuser::CW_USEDEFAULT"]
    width: i32,
    #[default = "winuser::CW_USEDEFAULT"]
    height: i32,

    #[default = "ptr::null_mut()"]
    parent: windef::HWND,
    #[default = "ptr::null_mut()"]
    menu: windef::HMENU,

    #[default = "None"]
    instance: Option<minwindef::HINSTANCE>,

    #[default = "ptr::null_mut()"]
    lp_param: minwindef::LPVOID,
}
