use std::{mem, ptr};

use common::*;

#[derive(Debug)]
pub struct Trackbar {
    parent: windef::HWND,
    hwnd: windef::HWND,
}

impl Trackbar {
    pub fn new(parent: windef::HWND) -> Self {
        let hwnd = Trackbar::create_track_bar(parent);

        let this = Self { parent, hwnd };
        this.show();
        this
    }

    fn create_track_bar(parent: windef::HWND) -> windef::HWND {
        unsafe {
            let hwnd = winuser::CreateWindowExW(
                0,
                commctrl::TRACKBAR_CLASS.to_wide(),
                "Trackbar".to_wide(),
                winuser::WS_CHILD
                    | winuser::WS_VISIBLE
                    | commctrl::TBS_HORZ
                    | commctrl::TBS_TRANSPARENTBKGND
                    | commctrl::TBS_NOTICKS,
                10,  // x
                10,  // y
                200, // w
                30,  // h
                parent,
                ptr::null_mut(),
                hinstance(),
                ptr::null_mut(),
            );

            //#define MAKEWPARAM(l, h)      ((WPARAM)(DWORD)MAKELONG(l, h))
            fn make_wparam(lo: minwindef::WORD, hi: minwindef::WORD) -> minwindef::WPARAM {
                minwindef::MAKELONG(lo, hi) as minwindef::WPARAM
            }

            winuser::ShowWindow(hwnd, winuser::SW_HIDE);
            winuser::SendMessageW(
                hwnd,
                winuser::WM_UPDATEUISTATE,
                make_wparam(winuser::UIS_SET, winuser::UISF_HIDEFOCUS),
                0,
            );

            hwnd
        }
    }

    pub fn show(&self) {
        unsafe { winuser::ShowWindow(self.hwnd, winuser::SW_SHOW) };
    }

    pub fn hide(&self) {
        unsafe { winuser::ShowWindow(self.hwnd, winuser::SW_HIDE) };
    }

    pub fn hwnd(&self) -> HWND {
        HWND::from(self.hwnd)
    }
}

/* need to think about what the trackbar will do
I should probably impl a Index trait
need a show and a hide
need to set the number of steps
*/
