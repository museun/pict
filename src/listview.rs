use std::{mem, ptr};

use common::*;

#[derive(Debug)]
pub struct ListView {
    hwnd: windef::HWND,
    parent: windef::HWND,
}

impl ListView {
    pub fn new(parent: windef::HWND) -> Self {
        use winapi::um::commctrl::*;
        use winapi::um::winuser::{WS_CHILD, WS_VISIBLE};

        unsafe {
            let mut rect = mem::zeroed::<windef::RECT>();
            winuser::GetClientRect(parent, &mut rect);
            let hwnd = winuser::CreateWindowExW(
                0,
                WC_LISTVIEW.to_wide(),
                "".to_wide(),
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
                hinstance(),
                ptr::null_mut(),
            );

            winuser::SendMessageW(
                hwnd,
                LVM_SETEXTENDEDLISTVIEWSTYLE,
                (LVS_EX_FULLROWSELECT | LVS_EX_DOUBLEBUFFER) as usize,
                (LVS_EX_FULLROWSELECT | LVS_EX_DOUBLEBUFFER) as isize,
            );

            let file = "File".to_wide_mut();
            let mut lvc = mem::uninitialized::<LVCOLUMNW>();
            lvc.mask = LVCF_TEXT | LVCF_WIDTH | LVCF_MINWIDTH | LVCF_IDEALWIDTH;
            lvc.pszText = file;
            lvc.cx = 200;
            lvc.cxIdeal = 200;
            lvc.cxMin = 100;
            winuser::SendMessageW(
                hwnd,
                LVM_INSERTCOLUMNW,
                0xFFFF,
                &lvc as *const _ as minwindef::LPARAM,
            );

            let size = "Size".to_wide_mut();
            lvc.mask |= LVCF_FMT | LVCF_SUBITEM;
            lvc.fmt = LVCFMT_RIGHT;
            lvc.pszText = size;
            lvc.cx = 30;
            lvc.cxIdeal = 30;
            lvc.cxMin = 20;
            winuser::SendMessageW(
                hwnd,
                LVM_INSERTCOLUMNW,
                0xFFFF,
                &lvc as *const _ as minwindef::LPARAM,
            );

            winuser::SendMessageW(hwnd, LVM_SETBKCOLOR, 0, wingdi::RGB(0, 0, 0) as isize);
            winuser::SendMessageW(hwnd, LVM_SETTEXTBKCOLOR, 0, wingdi::RGB(0, 0, 0) as isize);
            winuser::SendMessageW(
                hwnd,
                LVM_SETTEXTCOLOR,
                0,
                wingdi::RGB(255, 255, 255) as isize,
            );

            Self { hwnd, parent }
        }
    }

    pub fn fit_list_view(&self) {
        use winapi::um::commctrl::*;
        unsafe {
            let mut rect = mem::zeroed::<windef::RECT>();
            winuser::GetClientRect(self.parent, &mut rect);

            winuser::MoveWindow(self.hwnd, 0, 0, rect.right, rect.bottom, 1);

            winuser::GetClientRect(self.hwnd, &mut rect);
            let mut lvc = mem::zeroed::<LVCOLUMNW>();
            lvc.mask = LVCF_WIDTH;
            winuser::SendMessageW(
                self.hwnd,
                LVM_GETCOLUMNW,
                1,
                &lvc as *const _ as minwindef::LPARAM,
            );

            winuser::SendMessageW(
                self.hwnd,
                LVM_SETCOLUMNWIDTH,
                0,
                (rect.right - lvc.cx) as isize,
            );
            winuser::SendMessageW(
                self.hwnd,
                LVM_SETCOLUMNWIDTH,
                1,
                (LVSCW_AUTOSIZE_USEHEADER) as isize,
            );
        }
    }

    pub fn select(&self, index: usize) {
        use winapi::um::commctrl::*;
        debug!("selecting: {}", index);

        unsafe {
            let mut item = mem::zeroed::<LVITEMW>();
            item.state |= LVIS_SELECTED;
            item.stateMask |= LVIS_SELECTED;
            winuser::SendMessageW(
                self.hwnd,
                LVM_SETITEMSTATE,
                index,
                &item as *const _ as minwindef::LPARAM,
            );
        }
    }

    pub fn clear(&self) {
        unsafe {
            winuser::SendMessageW(self.hwnd, commctrl::LVM_DELETEALLITEMS, 0, 0);
        }
    }

    pub fn add_item(&self, name: &str, size: usize, index: usize) {
        use winapi::um::commctrl::*;

        unsafe {
            let name = name.to_wide_mut();
            let size = humanize_size(size).to_wide_mut();

            let mut item = mem::zeroed::<LVITEMW>();
            item.pszText = name;
            item.mask = LVIF_TEXT;
            item.iItem = index as i32;
            let n = winuser::SendMessageW(
                self.hwnd,
                LVM_INSERTITEMW,
                0,
                &item as *const _ as minwindef::LPARAM,
            );

            item.iSubItem = 1;
            item.pszText = size;
            winuser::SendMessageW(
                self.hwnd,
                LVM_SETITEMTEXTW,
                n as usize,
                &item as *const _ as minwindef::LPARAM,
            );
        };

        self.fit_list_view()
    }
}
