use std::cell::RefCell;
use std::path::PathBuf;
use std::str;

use winit;

use app::Handler;

pub struct FileList {
    inner: RefCell<FileListInner>,
    id: winit::WindowId,
}

pub(crate) struct FileListInner {
    active: usize,
    list: Vec<String>, // owned strings for now
}

// we need handle selecting an item in this list to tell the mainwindow to draw it
impl FileList {
    pub fn new(events: &winit::EventsLoop) -> Self {
        let window = winit::WindowBuilder::new()
            .with_title("pict")
            .with_dimensions((400, 200).into())
            .with_resizable(true)
            .build(&events)
            .unwrap();

        let inner = FileListInner {
            active: 0,
            list: vec![],
        };

        Self {
            inner: RefCell::new(inner),
            id: window.id(),
        }
    }

    pub fn show(&self) {
        // ShowWindow(this.hwnd, SW_SHOW);
        eprintln!("showing file list")
    }

    pub fn hide(&self) {
        // ShowWindow(this.hwnd, SW_HIDE);
        eprintln!("hiding file list")
    }

    pub fn is_visible(&self) -> bool {
        // auto wp = WINDOWPLACEMENT{}
        // GetWindowPlacement(this.hwnd, &wp);
        // wp.showCmd == SW_SHOW
        false
    }

    pub fn select(&self, index: usize) {
        // determine if index is out of bounds
        // select item in the listview
        self.inner.borrow_mut().active = index;
    }

    pub fn clear(&self) {
        // clear the listview
        self.inner.borrow_mut().list.clear();
        self.inner.borrow_mut().list.shrink_to_fit();

        // maybe set the title to a default state here
    }

    pub fn populate(&self, dir: &str, files: &[String]) {
        let images = files
            .into_iter()
            .filter(|s| accepted_image_type(*s))
            .map(String::to_owned) // why do I need to do this?
            .collect::<Vec<_>>();

        self.set_title(&dir);

        self.clear();
        self.inner.borrow_mut().list.extend(images);

        // TODO actually update the UI
        eprintln!("dir: {}", dir);

        let list = &self.inner.borrow().list;
        for file in list {
            eprintln!("file: {}", file);
        }
    }

    fn set_title(&self, _name: &str) {
        // set the window title to name
    }
}

impl Handler for FileList {
    fn handle(&self, ev: &winit::WindowEvent) {
        match ev {
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
