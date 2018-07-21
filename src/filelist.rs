#![allow(dead_code)]
use std::sync::{Arc, Mutex};
use std::{mem, ptr, str};

use common::*;
use listview::ListView;

lazy_static! {
    static ref FILE_CLASS: () = {
        Class::create("PictFileListClass".to_wide());
    };
}

pub struct FileList {
    pub(crate) window: Window,
    listview: ListView,
    context: Arc<Mutex<Context>>,
}

impl Drop for FileList {
    fn drop(&mut self) {
        info!(
            "dropping filelist: {} / {}",
            Arc::weak_count(&self.context),
            Arc::strong_count(&self.context),
        );
    }
}

impl FileList {
    pub fn new(context: Arc<Mutex<Context>>) -> Self {
        ::lazy_static::initialize(&FILE_CLASS);

        let params = Params::builder()
            .class_name("PictFileListClass".to_wide())
            .window_name("filelist".to_wide())
            .ex_style(winuser::WS_EX_TOOLWINDOW)
            .style(winuser::WS_TILEDWINDOW)
            .width(200)
            .height(400)
            .build();

        let window = Window::new(&params);
        LIST_HWND.with(|hwnd| {
            let mut this = hwnd.lock().unwrap();
            if this.is_none() {
                *this = Some(window.hwnd())
            }
        });

        let listview = ListView::new(window.hwnd());
        listview.fit_list_view();

        Self {
            listview,
            window,
            context,
        }
    }

    pub fn hwnd(&self) -> HWND {
        self.window.hwnd().into()
    }

    pub fn show(&self) {
        debug!("showing file list");
        self.listview.fit_list_view();
        self.window.show();
    }

    pub fn hide(&self) {
        debug!("hiding file list");
        self.listview.fit_list_view();
        self.window.hide();
    }

    pub fn is_visible(&self) -> bool {
        debug!("checking visibility of filelist");
        unsafe { winuser::IsWindowVisible(self.window.hwnd()) == 1 }
    }

    pub fn select(&self, index: usize) {
        debug!("selecting index: {}", index);
        self.listview.select(index)
    }

    pub fn clear(&self) {
        debug!("clearing file list");
        self.listview.clear()
    }

    pub fn populate(&self, dir: &str, files: &[(String, usize)]) {
        debug!("populating ({}) filelist from {}", files.len(), dir);
        self.clear();

        let images = files.to_vec();
        self.set_title(&dir);
        for item in &images {
            self.listview.add_item(&item.0, item.1, {
                self.context.lock().unwrap().get_index() // XXX: does this really need to do this?
            });
        }
    }

    pub fn set_title(&self, title: &str) {
        debug!("setting title {}", title);
        unsafe { winuser::SetWindowTextW(self.window.hwnd(), title.to_wide()) };
    }

    pub fn align_to(&self, neighbor: windef::HWND) {
        let hwnd = self.window.hwnd();
        unsafe {
            let mut rect = mem::zeroed::<windef::RECT>();
            winuser::GetWindowRect(neighbor, &mut rect);

            let mut list = mem::zeroed::<windef::RECT>();
            winuser::GetWindowRect(hwnd, &mut list);

            let mut width = list.right - list.left;
            if width == 0 {
                width = 200;
            }

            winuser::SetWindowPos(
                hwnd,
                ptr::null_mut(),
                rect.left - width,
                rect.top,
                width,
                list.bottom - list.top,
                winuser::SWP_NOACTIVATE,
            );
        }
    }

    fn on_resized(&self) {
        self.listview.fit_list_view()
    }

    fn on_notify(&self, lp: isize) {
        use winapi::um::commctrl::{
            LVIS_SELECTED, LVN_ITEMACTIVATE, LVN_ITEMCHANGED, NMITEMACTIVATE, NMLISTVIEW, NM_CLICK,
            NM_RETURN, NM_SETFOCUS,
        };

        unsafe {
            let pnmlv = *(lp as *mut NMLISTVIEW);
            if pnmlv.hdr.hwndFrom != self.window.hwnd() {
                return;
            }
            match pnmlv.hdr.code {
                NM_SETFOCUS | NM_RETURN | NM_CLICK | LVN_ITEMACTIVATE | LVN_ITEMCHANGED => {
                    let item = *(lp as *mut NMITEMACTIVATE);
                    let mut index = {
                        let this = self.context.lock().unwrap();
                        this.get_index()
                    };

                    if item.iItem != -1
                        && (item.uNewState ^ item.uOldState) & LVIS_SELECTED == 0
                        && index != item.iItem as usize
                    {
                        self.context.lock().unwrap().set_index(item.iItem as usize)
                    }
                }
                _ => return,
            }
        };
    }
}
