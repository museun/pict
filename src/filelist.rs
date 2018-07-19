use std::path::PathBuf;
use std::str;

use winit;
use winit::os::windows::WindowExt;

use winapi::shared::windef::HWND;
use winapi::um::winuser::{IsWindowVisible, SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_TOOLWINDOW};

use app::Handler;

pub struct FileList {
    window: winit::Window,
    id: winit::WindowId,
}

// we need handle selecting an item in this list to tell the mainwindow to draw it
impl FileList {
    pub fn new(events: &winit::EventsLoop) -> Self {
        let window = winit::WindowBuilder::new()
            .with_title("filelist")
            .with_dimensions((200, 400).into())
            .with_resizable(true)
            .build(&events)
            .unwrap();

        window.hide();

        // set the filelist to be a tool window
        unsafe {
            let hwnd = window.get_hwnd() as HWND;
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, WS_EX_TOOLWINDOW as isize);
        };

        let id = window.id();
        Self { window, id }
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
        let hwnd = self.window.get_hwnd();
        let hwnd = hwnd as HWND;
        unsafe { IsWindowVisible(hwnd) == 1 }
    }

    pub fn select(&self, index: usize) {
        debug!("selecting index: {}", index);
        // determine if index is out of bounds
        // select item in the listview
    }

    pub fn clear(&self) {
        debug!("clearing file list");
        // clear the listview

        // maybe set the title to a default state here
    }

    pub fn populate(&self, dir: &str, files: &[String]) {
        debug!("populating filelist from {}", dir);

        let images = files
            .into_iter()
            .filter(|s| accepted_image_type(*s))
            .map(String::to_owned) // why do I need to do this?
            .collect::<Vec<_>>();

        self.set_title(&dir);
        self.clear();

        // TODO actually update the UI
        eprintln!("dir: {}", dir);
        for file in &images {
            eprintln!("file: {}", file);
        }
    }

    fn set_title(&self, name: &str) {
        debug!("setting title {}", name);
        // set the window title to name
    }
}

impl Handler for FileList {
    fn handle(&self, ev: &winit::WindowEvent) {
        match ev {
            winit::WindowEvent::CloseRequested => {
                self.hide();
            }

            _ => {}
        }
    }

    fn id(&self) -> winit::WindowId {
        self.id
    }
}

const ACCEPTED_EXTENSIONS: [&str; 4] = ["PNG", "JPG", "JPEG", "GIF"];
fn accepted_image_type<P: Into<PathBuf>>(path: P) -> bool {
    fn find(path: &PathBuf) -> Option<()> {
        let ext = path.extension()?.to_str()?.to_ascii_uppercase();
        for e in &ACCEPTED_EXTENSIONS {
            if *e == ext {
                return Some(());
            }
        }
        None
    }
    find(&path.into()).is_some()
}
