use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use std::{mem, ptr};

use winapi::shared::{minwindef, windef};
use winapi::um::winuser;

use config::{self, Config, Position, Size};
use context::Context;
use filelist::FileList;
use mainwindow::MainWindow;

thread_local!{
    pub static APP: RefCell<Option<Mutex<Context>>> = RefCell::new(None);
}

pub struct App {
    mainwindow: windef::HWND,
    filelist: windef::HWND,
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

impl App {
    pub fn new() -> Self {
        let config = Config::load();

        // set up a thread local reference to the context
        APP.with(|app| {
            let app = &mut *app.borrow_mut();
            if app.is_none() {
                *app = Some(Mutex::new(Context::new(&config)))
            }
        });

        let (mainwindow, filelist) = App::with_context(|app| {
            let app = app.lock().unwrap();
            (app.mainwindow.hwnd(), app.filelist.hwnd())
        });

        Self {
            mainwindow,
            filelist,
        }
    }

    pub unsafe extern "system" fn wndproc(
        hwnd: windef::HWND,
        msg: minwindef::UINT,
        wp: minwindef::WPARAM,
        lp: minwindef::LPARAM,
    ) -> minwindef::LRESULT {
        trace!("hwnd: {:?}, {}", hwnd, msg);

        let ctx = winuser::GetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA);
        if ctx == 0 {
            if winuser::WM_CREATE == msg {
                let cs: &mut winuser::CREATESTRUCTW = mem::transmute(lp);
                winuser::SetWindowLongPtrW(
                    hwnd,
                    winuser::GWLP_USERDATA,
                    cs.lpCreateParams as isize,
                );
            }
            return winuser::DefWindowProcW(hwnd, msg, wp, lp);
        }

        use winapi::um::winuser::*;
        match msg {
            WM_DESTROY => {
                winuser::PostQuitMessage(0);
                0
            }
            _ => winuser::DefWindowProcW(hwnd, msg, wp, lp),
        }
    }

    fn save() {
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
    }

    pub fn run(self) {
        unsafe {
            loop {
                let mut msg = mem::uninitialized();
                if winuser::GetMessageW(&mut msg, ptr::null_mut(), 0, 0) <= 0 {
                    error!("no message to get!");
                    return;
                }
                winuser::TranslateMessage(&msg);
                winuser::DispatchMessageW(&msg);
            }
        }
    }

    pub fn get_list_len() -> usize {
        Self::with_context(|app| app.lock().unwrap().get_len())
    }

    pub fn get_index() -> usize {
        Self::with_context(|app| app.lock().unwrap().get_index())
    }

    pub fn set_index(index: usize) {
        Self::with_context(|app| app.lock().unwrap().set_index(index))
    }

    pub fn get_filelist() -> Rc<FileList> {
        Self::with_context(|app| {
            let app = app.lock().unwrap();
            Rc::clone(&app.filelist)
        })
    }

    pub fn get_mainwindow() -> Rc<MainWindow> {
        Self::with_context(|app| {
            let app = app.lock().unwrap();
            Rc::clone(&app.mainwindow)
        })
    }

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
