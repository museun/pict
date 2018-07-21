use std::{mem, ptr};

use common::*;

pub struct Window {
    hwnd: HWND,
}

impl Drop for Window {
    fn drop(&mut self) {
        info!("dropping window");
    }
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

impl Window {
    pub fn new(params: &Params) -> Self {
        unsafe { Window::init(&params) }
    }

    pub fn hwnd(&self) -> windef::HWND {
        self.hwnd.0
    }

    pub fn show(&self) {
        unsafe { winuser::ShowWindow(self.hwnd(), winuser::SW_SHOW) };
    }

    pub fn hide(&self) {
        unsafe { winuser::ShowWindow(self.hwnd(), winuser::SW_HIDE) };
    }

    /// w,h
    pub fn get_size(&self) -> (i32, i32) {
        unsafe {
            let mut rect: windef::RECT = mem::zeroed();
            winuser::GetClientRect(self.hwnd(), &mut rect);
            (rect.right, rect.bottom)
        }
    }

    /// x,y
    pub fn get_pos(&self) -> (i32, i32) {
        unsafe {
            let mut rect: windef::RECT = mem::zeroed();
            winuser::GetWindowRect(self.hwnd(), &mut rect);
            (rect.left, rect.top)
        }
    }

    pub fn set_title(&self, title: &str) {
        unsafe { winuser::SetWindowTextW(self.hwnd(), title.to_wide()) };
    }

    unsafe fn init(params: &Params) -> Self {
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
            ::util::hinstance(),
            params.lp_param,
        );
        if hwnd.is_null() {
            let err = get_last_windows_error();
            error!("cannot create window: {:?}, {}", params.window_name, err);
        }
        assert!(!hwnd.is_null());

        subclass_window(hwnd);
        Self { hwnd: HWND(hwnd) }
    }
}

pub unsafe extern "system" fn callback(
    hwnd: windef::HWND,
    msg: minwindef::UINT,
    wp: minwindef::WPARAM,
    lp: minwindef::LPARAM,
    _: basetsd::UINT_PTR,
    _data: basetsd::DWORD_PTR,
) -> minwindef::LRESULT {
    // get the app here, pass it as a reference with the
    let target = HWND(hwnd);

    use winapi::um::winuser::*;
    match msg {
        WM_NCCREATE => commctrl::DefSubclassProc(hwnd, msg, wp, lp),
        WM_CLOSE => {
            App::handle(&Event {
                event: EventType::CloseRequest,
                hwnd: target,
            });
            0
        }
        winuser::WM_WINDOWPOSCHANGED => {
            let pos = lp as *const WINDOWPOS;
            if (*pos).flags & SWP_NOMOVE != SWP_NOMOVE {
                let (x, y) = ((*pos).x, (*pos).y); // calc x,y
                App::handle(&Event {
                    event: EventType::Moved { x, y },
                    hwnd: target,
                });
            }
            commctrl::DefSubclassProc(hwnd, msg, wp, lp)
        }
        WM_WINDOWPOSCHANGING => {
            let pos = lp as *const WINDOWPOS;
            if (*pos).flags & SWP_NOMOVE != SWP_NOMOVE {
                let (x, y) = ((*pos).x, (*pos).y);
                App::handle(&Event {
                    event: EventType::Moving { x, y },
                    hwnd: target,
                });
            }
            0
        }

        WM_DESTROY => {
            winuser::PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wp, lp),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HWND(windef::HWND);
impl From<windef::HWND> for HWND {
    fn from(hwnd: windef::HWND) -> Self {
        HWND(hwnd)
    }
}

impl From<HWND> for windef::HWND {
    fn from(hwnd: HWND) -> Self {
        hwnd.0
    }
}

// https://github.com/retep998/winapi-rs/issues/360
unsafe impl Send for HWND {}

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
