use std::{mem, ptr};

use common::*;

#[derive(Debug)]
pub struct Window {
    hwnd: HWND,
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

    /// w,h in client coords
    pub fn get_size(&self) -> (i32, i32) {
        unsafe {
            let mut rect: windef::RECT = mem::zeroed();
            winuser::GetClientRect(self.hwnd(), &mut rect);
            (rect.right, rect.bottom)
        }
    }

    /// w,h in window coords
    pub fn get_size_win(&self) -> (i32, i32) {
        unsafe {
            let mut rect: windef::RECT = mem::zeroed();
            winuser::GetWindowRect(self.hwnd(), &mut rect);
            (rect.right - rect.left, rect.bottom - rect.top)
        }
    }

    pub fn set_size(&self, w: i32, h: i32) {
        unsafe {
            winuser::SetWindowPos(
                self.hwnd(),
                ptr::null_mut(), // ignore
                0,               // ignore
                0,               // ignore
                w,
                h,
                winuser::SWP_NOZORDER | winuser::SWP_NOMOVE | winuser::SWP_NOOWNERZORDER,
            );
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

    pub fn set_pos(&self, x: i32, y: i32) {
        unsafe {
            winuser::SetWindowPos(
                self.hwnd(),
                ptr::null_mut(), // ignore
                x,
                y,
                0, // ignore
                0, // ignore
                winuser::SWP_NOZORDER | winuser::SWP_NOSIZE | winuser::SWP_NOOWNERZORDER,
            );
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
    let target = HWND(hwnd);

    use winapi::um::winuser::*;
    match msg {
        WM_NCCREATE => commctrl::DefSubclassProc(hwnd, msg, wp, lp),
        WM_CLOSE => App::handle(&Event {
            event: EventType::CloseRequest,
            hwnd: target,
        }),
        winuser::WM_WINDOWPOSCHANGED => {
            let pos = lp as *const WINDOWPOS;
            if (*pos).flags & SWP_NOMOVE != SWP_NOMOVE {
                let (x, y) = ((*pos).x, (*pos).y);
                App::handle(&Event {
                    event: EventType::Moved { x, y },
                    hwnd: target,
                })
            } else {
                commctrl::DefSubclassProc(hwnd, msg, wp, lp)
            }
        }

        WM_WINDOWPOSCHANGING => {
            let pos = lp as *const WINDOWPOS;
            if (*pos).flags & SWP_NOMOVE != SWP_NOMOVE {
                let (x, y) = ((*pos).x, (*pos).y);
                App::handle(&Event {
                    event: EventType::Moving { x, y },
                    hwnd: target,
                })
            } else {
                0
            }
        }

        WM_KEYDOWN => {
            let key: Key = (wp as i32).into();
            App::handle(&Event {
                event: EventType::KeyDown { key },
                hwnd: target,
            })
        }

        WM_LBUTTONDOWN | WM_MBUTTONDOWN | WM_RBUTTONDOWN => {
            let x = windowsx::GET_X_LPARAM(lp);
            let y = windowsx::GET_Y_LPARAM(lp);
            let button: MouseButton = wp.into();
            App::handle(&Event {
                event: EventType::MouseDown { button, x, y },
                hwnd: target,
            })
        }

        WM_MOUSEMOVE => {
            let x = windowsx::GET_X_LPARAM(lp);
            let y = windowsx::GET_Y_LPARAM(lp);
            App::handle(&Event {
                event: EventType::MouseMove { x, y },
                hwnd: target,
            })
        }

        WM_MOUSEWHEEL => {
            let delta = winuser::GET_WHEEL_DELTA_WPARAM(wp);
            let x = windowsx::GET_X_LPARAM(lp);
            let y = windowsx::GET_Y_LPARAM(lp);
            App::handle(&Event {
                event: EventType::MouseWheel { delta, x, y },
                hwnd: target,
            })
        }

        WM_DROPFILES => {
            let hdrop = wp as shellapi::HDROP;
            let count = shellapi::DragQueryFileW(hdrop, 0xFFFF_FFFF, ptr::null_mut(), 0);

            let mut buf: [u16; minwindef::MAX_PATH] = mem::uninitialized();
            for i in 0..count {
                let n = shellapi::DragQueryFileW(
                    hdrop,
                    i,
                    buf.as_mut_ptr(),
                    minwindef::MAX_PATH as u32,
                ) as usize;
                if n > 0 {
                    App::handle(&Event {
                        event: EventType::DropFile {
                            file: String::from_utf16_lossy(&buf[0..n]),
                        },
                        hwnd: target,
                    });
                }
            }

            shellapi::DragFinish(hdrop);
            0
        }

        WM_NOTIFY => App::handle(&Event {
            event: EventType::Notify { lp },
            hwnd: target,
        }),

        WM_HSCROLL => App::handle(&Event {
            event: EventType::HScroll { wp, lp },
            hwnd: target,
        }),

        WM_CTLCOLORSTATIC => App::handle(&Event {
            event: EventType::CtrlColorStatic { wp, lp },
            hwnd: target,
        }),

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
