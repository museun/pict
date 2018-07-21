use std::ptr;

use winapi::shared::{basetsd, minwindef, ntdef, windef};
use winapi::um::{commctrl, winuser};

use event::*;

pub struct Window {
    hwnd: HWND,
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

impl Window {
    pub fn new(queue: &EventQueue, params: &Params) -> Self {
        unsafe { Window::init(&queue, &params) }
    }

    pub fn hwnd(&self) -> windef::HWND {
        self.hwnd.0
    }

    pub fn show(&self) {
        unsafe { winuser::ShowWindow(self.hwnd(), winuser::SW_SHOW) };
    }

    unsafe fn init(queue: &EventQueue, params: &Params) -> Self {
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
        assert!(!hwnd.is_null());

        let queue = EventQueue(queue.0.clone());
        subclass_window(hwnd, queue);
        Self { hwnd: HWND(hwnd) }
    }
}

unsafe extern "system" fn callback(
    hwnd: windef::HWND,
    msg: minwindef::UINT,
    wp: minwindef::WPARAM,
    lp: minwindef::LPARAM,
    _: basetsd::UINT_PTR,
    data: basetsd::DWORD_PTR,
) -> minwindef::LRESULT {
    let queue = &mut *(data as *mut EventQueue);
    let target = HWND(hwnd);

    use winapi::um::winuser::*;
    match msg {
        WM_NCCREATE => commctrl::DefSubclassProc(hwnd, msg, wp, lp),
        WM_CLOSE => {
            queue.send(Event {
                event: EventType::CloseRequest,
                hwnd: target,
            });
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

// https://github.com/retep998/winapi-rs/issues/360
unsafe impl Send for HWND {}

const WINDOW_SUBCLASS_ID: basetsd::UINT_PTR = 0;
pub fn subclass_window(hwnd: windef::HWND, queue: EventQueue) {
    let ptr = Box::into_raw(Box::new(queue));
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
