use std::mem;
use std::ptr;
use std::str;

use winit;
use winit::os::windows::WindowExt;

use winapi::shared::windef::{HWND, RECT};
use winapi::um::commctrl::{
    LVIS_SELECTED, LVN_ITEMACTIVATE, LVN_ITEMCHANGED, NMITEMACTIVATE, NMLISTVIEW, NM_CLICK,
    NM_RETURN, NM_SETFOCUS,
};

use winapi::um::winuser::{
    GetWindowRect, IsWindowVisible, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, SWP_NOACTIVATE,
    WS_EX_TOOLWINDOW,
};

use app::{App, Handler};
use listview::ListView;
use util::*;

pub struct FileList {
    window: winit::Window,
    id: winit::WindowId,
    listview: ListView,
}

impl FileList {
    pub fn new(events: &winit::EventsLoop) -> Self {
        let window = winit::WindowBuilder::new()
            .with_title("filelist")
            .with_dimensions((200, 400).into())
            .with_resizable(true)
            .build(&events)
            .unwrap();

        // set the filelist to be a tool window
        unsafe {
            let hwnd = window.get_hwnd() as HWND;
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, WS_EX_TOOLWINDOW as isize);
        };

        window.hide();
        let id = window.id();

        let listview = ListView::new(window.get_hwnd() as HWND);
        listview.fit_list_view();

        Self {
            window,
            id,
            listview,
        }
    }

    pub fn show(&self) {
        debug!("showing file list");
        self.window.show();
    }

    pub fn hide(&self) {
        debug!("hiding file list");
        self.window.hide();
    }

    pub fn is_visible(&self) -> bool {
        debug!("checking visibility of filelist");
        unsafe { IsWindowVisible(self.hwnd()) == 1 }
    }

    pub fn select(&self, index: usize) {
        debug!("selecting index: {}", index);
        self.listview.select(index)
    }

    pub fn clear(&self) {
        debug!("clearing file list");
        self.listview.clear()
    }

    pub fn populate(&self, dir: &str, files: &[String]) {
        debug!("populating filelist from {}", dir);
        self.clear();

        let images = files
            .into_iter()
            .filter(|s|is_accepted_image_type(*s))
            .map(String::to_owned) // why do I need to do this?
            .collect::<Vec<_>>();

        self.set_title(&dir);

        for image in &images {
            self.listview.add_item(image, 1024 << 3);
        }
    }

    pub fn set_title(&self, name: &str) {
        debug!("setting title {}", name);
        self.window.set_title(name);
    }

    pub fn align_to(&self, neighbor: HWND) {
        let hwnd = self.window.get_hwnd() as HWND;
        unsafe {
            let mut rect = mem::uninitialized::<RECT>();
            GetWindowRect(neighbor, &mut rect);

            let mut list = mem::uninitialized::<RECT>();
            GetWindowRect(hwnd, &mut list);

            let mut width = list.right - list.left;
            if width == 0 {
                width = 200;
            }

            SetWindowPos(
                hwnd,
                ptr::null_mut(),
                rect.left - width,
                rect.top,
                width,
                list.bottom - list.top,
                SWP_NOACTIVATE,
            );
        }
    }

    fn on_resized(&self) {
        self.listview.fit_list_view()
    }

    fn on_notify(&self, lp: isize) {
        unsafe {
            let pnmlv = *(lp as *mut NMLISTVIEW);
            if pnmlv.hdr.hwndFrom != self.hwnd() {
                return;
            }
            match pnmlv.hdr.code {
                NM_SETFOCUS | NM_RETURN | NM_CLICK | LVN_ITEMACTIVATE | LVN_ITEMCHANGED => {
                    let item = *(lp as *mut NMITEMACTIVATE);
                    let mut index = App::get_index();
                    if item.iItem != -1
                        && (item.uNewState ^ item.uOldState) & LVIS_SELECTED == 0
                        && index != item.iItem as usize
                    {
                        App::set_index(item.iItem as usize)
                    }
                }
                _ => return,
            }
        };
    }
}

impl Handler for FileList {
    fn handle(&self, ev: &winit::WindowEvent) {
        match *ev {
            winit::WindowEvent::CloseRequested => {
                self.hide();
            }
            winit::WindowEvent::Resized(_) => {
                self.on_resized();
            }
            winit::WindowEvent::Notify(lp) => {
                trace!("on notify: {0:X}", lp);
                self.on_notify(lp);
            }
            _ => {
                trace!("filelist event: {:#?}", ev);
            }
        }
    }

    fn id(&self) -> winit::WindowId {
        self.id
    }

    fn hwnd(&self) -> HWND {
        self.window.get_hwnd() as HWND
    }
}
