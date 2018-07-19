use std::mem;
use std::ptr;

use winapi::shared::minwindef::LPARAM;
use winapi::shared::windef::{HWND, RECT};
use winapi::um::commctrl::*; // TODO fully qualify these symbols
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::RGB;
use winapi::um::winuser::*; // TODO fully qualify these symbols

use app::App;
use util::*;

pub struct ListView {
    hwnd: HWND,
    parent: HWND,
}

impl ListView {
    pub fn new(parent: HWND) -> Self {
        unsafe {
            let mut rect = mem::uninitialized::<RECT>();
            GetClientRect(parent, &mut rect);
            let hwnd = CreateWindowExW(
                0,
                WC_LISTVIEW.to_wide().as_ptr(),
                "".to_wide().as_ptr(),
                WS_CHILD
                    | WS_VISIBLE
                    | LVS_NOCOLUMNHEADER
                    | LVS_REPORT
                    | LVS_NOSORTHEADER
                    | LVS_SHOWSELALWAYS
                    | LVS_SINGLESEL,
                0,
                0,
                rect.right,
                rect.bottom,
                parent,
                ptr::null_mut(),
                GetModuleHandleW(ptr::null_mut()),
                ptr::null_mut(),
            );

            SendMessageW(
                hwnd,
                LVM_SETEXTENDEDLISTVIEWSTYLE,
                (LVS_EX_FULLROWSELECT | LVS_EX_DOUBLEBUFFER) as usize,
                (LVS_EX_FULLROWSELECT | LVS_EX_DOUBLEBUFFER) as isize,
            );

            let mut lvc = mem::uninitialized::<LVCOLUMNW>();

            lvc.mask = LVCF_TEXT | LVCF_WIDTH | LVCF_MINWIDTH | LVCF_IDEALWIDTH;
            lvc.pszText = "File".to_wide().as_mut_ptr();
            lvc.cx = 200;
            lvc.cxIdeal = 200;
            lvc.cxMin = 100;
            SendMessageW(hwnd, LVM_INSERTCOLUMNW, 0xFFFF, &lvc as *const _ as LPARAM);

            lvc.mask |= LVCF_FMT | LVCF_SUBITEM;
            lvc.fmt = LVCFMT_RIGHT;
            lvc.pszText = "Size".to_wide().as_mut_ptr();
            lvc.cx = 30;
            lvc.cxIdeal = 30;
            lvc.cxMin = 20;
            SendMessageW(hwnd, LVM_INSERTCOLUMNW, 0xFFFF, &lvc as *const _ as LPARAM);

            // SendMessageW(hwnd, LVM_SETBKCOLOR, 0, RGB(0, 0, 0) as isize);
            // SendMessageW(hwnd, LVM_SETTEXTBKCOLOR, 0, RGB(0, 0, 0) as isize);
            // SendMessageW(hwnd, LVM_SETTEXTCOLOR, 0, RGB(255, 255, 255) as isize);

            Self { hwnd, parent }
        }
    }

    pub fn fit_list_view(&self) {
        unsafe {
            let mut rect = mem::uninitialized::<RECT>();
            GetClientRect(self.parent, &mut rect);

            MoveWindow(self.hwnd, 0, 0, rect.right, rect.bottom, 1);

            GetClientRect(self.hwnd, &mut rect);
            let mut lvc = mem::uninitialized::<LVCOLUMNW>();
            lvc.mask = LVCF_WIDTH;
            SendMessageW(self.hwnd, LVM_GETCOLUMNW, 1, &lvc as *const _ as LPARAM);

            SendMessageW(
                self.hwnd,
                LVM_SETCOLUMNWIDTH,
                0,
                (rect.right - lvc.cx) as isize,
            );
            SendMessageW(
                self.hwnd,
                LVM_SETCOLUMNWIDTH,
                1,
                (LVSCW_AUTOSIZE_USEHEADER) as isize,
            );
        }
    }

    pub fn select(&self, index: usize) {
        unsafe {
            let mut item = mem::uninitialized::<LVITEMW>();
            item.state = LVIS_SELECTED;
            item.stateMask = LVIS_SELECTED;
            SendMessageW(
                self.hwnd,
                LVM_SETITEMSTATE,
                index,
                &item as *const _ as LPARAM,
            );
        }
    }

    pub fn clear(&self) {
        unsafe {
            SendMessageW(self.hwnd, LVM_DELETEALLITEMS, 0, 0);
        }
    }

    pub fn add_item(&self, name: &str, size: usize) {
        debug!("adding item: [{}] {}", size, name);

        let index = App::get_index() + 1;
        unsafe {
            let mut item = mem::uninitialized::<LVITEMW>();
            item.pszText = name.to_wide().as_mut_ptr();
            item.mask = LVIF_TEXT;
            item.iItem = index as i32;

            SendMessageW(self.hwnd, LVM_INSERTITEMW, 0, &item as *const _ as LPARAM);
            let mut sz = humanize_size(size).to_wide();
            item.iSubItem = 1;
            item.pszText = sz.as_mut_ptr();
            SendMessageW(
                self.hwnd,
                LVM_SETITEMTEXTW,
                index,
                &item as *const _ as LPARAM,
            );
        };

        App::set_index(index);

        self.fit_list_view()
    }
}
