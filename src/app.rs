use std::cell::RefCell;
use std::ptr;
use std::sync::Mutex;

use winapi::um::{combaseapi, objbase};

use config::Config;
use context::Context;
use event::*;

use mainwindow::MainWindow;

thread_local!{
    pub static APP: RefCell<Option<Mutex<Context>>> = RefCell::new(None);
}

pub struct App {
    mainwindow: MainWindow,
    queue: EventQueue,
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

impl App {
    pub fn new() -> Self {
        COM_INITIALIZED.with(|_| {});

        let config = Config::load();

        // set up a thread local reference to the context
        APP.with(|app| {
            let app = &mut *app.borrow_mut();
            if app.is_none() {
                *app = Some(Mutex::new(Context::new(&config)))
            }
        });

        let queue = EventQueue::new();

        let mainwindow = MainWindow::new(&config, &queue);
        Self { mainwindow, queue }
    }

    pub fn run(self) {
        let main = self.mainwindow.hwnd();

        self.queue.run(move |ev: Event| {
            trace!("ev: {:?}", ev);
            if ev.event == EventType::CloseRequest && ev.hwnd == main {
                return ControlFlow::Break;
            }

            ControlFlow::Continue
        });
    }

    //fn save() {
    // let mainwindow = Self::get_mainwindow();

    // Config {
    //     position: Position { x: pos.0, y: pos.1 },
    //     size: Size {
    //         w: size.0 as i32,
    //         h: size.1 as i32,
    //     },
    //     filelist: config::FileList {
    //         snap: App::with_context(|app| app.lock().unwrap().get_snap()),
    //     },
    // }.save();
    //}

    pub fn get_list_len() -> usize {
        Self::with_context(|app| app.lock().unwrap().get_len())
    }

    pub fn get_index() -> usize {
        Self::with_context(|app| app.lock().unwrap().get_index())
    }

    pub fn set_index(index: usize) {
        Self::with_context(|app| app.lock().unwrap().set_index(index))
    }

    // pub fn get_filelist() -> Rc<FileList> {
    //     Self::with_context(|app| {
    //         let app = app.lock().unwrap();
    //         Rc::clone(&app.filelist)
    //     })
    // }

    // pub fn get_mainwindow() -> Rc<MainWindow> {
    //     Self::with_context(|app| {
    //         let app = app.lock().unwrap();
    //         Rc::clone(&app.mainwindow)
    //     })
    // }

    pub fn with_context<T>(f: impl Fn(&Mutex<Context>) -> T) -> T {
        APP.with(|app| {
            if let Some(ref app) = *app.borrow() {
                f(app)
            } else {
                Self::die_invalid_state();
            }
        })
    }

    fn die_invalid_state() -> ! {
        panic!("APP has not been created!");
    }
}

struct ComInitialized(*mut ());
impl Drop for ComInitialized {
    fn drop(&mut self) {
        unsafe { combaseapi::CoUninitialize() };
    }
}

thread_local!{
    static COM_INITIALIZED: ComInitialized = {
        unsafe {
            combaseapi::CoInitializeEx(ptr::null_mut(), objbase:: COINIT_MULTITHREADED);
            ComInitialized(ptr::null_mut())
        }
    };
}
