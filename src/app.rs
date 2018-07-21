#![allow(dead_code)]
use std::sync::{Arc, Mutex};
use std::{mem, ptr};

use common::*;

use filelist::FileList;
use mainwindow::MainWindow;

thread_local!{
    pub static MAIN_HWND: Mutex<Option<windef::HWND>> = Mutex::new(None);
    pub static LIST_HWND: Mutex<Option<windef::HWND>> = Mutex::new(None);
    static TID: minwindef::DWORD = unsafe { processthreadsapi::GetCurrentThreadId() };
}

thread_local! {
    pub static APP: Mutex<Option<App>> = Mutex::new(None);
}

pub struct App {
    mainwindow: MainWindow,
    filelist: FileList,
    context: Arc<Mutex<Context>>,
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

impl Drop for App {
    fn drop(&mut self) {
        info!("dropping context");
        let pos = self.mainwindow.window.get_pos();
        let size = self.mainwindow.window.get_size();

        Config {
            position: Position { x: pos.0, y: pos.1 },
            size: Size {
                w: size.0 as i32,
                h: size.1 as i32,
            },
            filelist: ::config::FileList {
                snap: self.context.lock().unwrap().get_snap(),
            },
        }.save();
    }
}

impl App {
    pub fn new() -> Self {
        COM_INITIALIZED.with(|_| {});

        let context = Arc::new(Mutex::new(Context::new()));
        let mainwindow = MainWindow::new(Arc::clone(&context));
        let filelist = FileList::new(Arc::clone(&context));

        Self {
            mainwindow,
            filelist,
            context,
        }
    }

    pub fn run(self) {
        unsafe {
            winuser::IsGUIThread(1);

            let mut msg = mem::uninitialized();
            loop {
                if winuser::GetMessageW(&mut msg, ptr::null_mut(), 0, 0) == 0 {
                    warn!("no message to get!");
                    return;
                }

                winuser::TranslateMessage(&msg);
                winuser::DispatchMessageW(&msg);
            }
        }
    }

    fn main_hwnd() -> ::window::HWND {
        MAIN_HWND.with(|hwnd| hwnd.lock().unwrap().unwrap()).into()
    }

    fn list_hwnd() -> ::window::HWND {
        LIST_HWND.with(|hwnd| hwnd.lock().unwrap().unwrap()).into()
    }

    pub fn with_mainwindow<T>(f: impl Fn(&MainWindow) -> T) -> T {
        APP.with(|app| {
            let this = &*app.lock().unwrap();
            if let Some(this) = this.as_ref() {
                f(&this.mainwindow)
            } else {
                panic!("invalid state getting mainwindow");
            }
        })
    }

    pub fn with_filelist<T>(f: impl Fn(&FileList) -> T) -> T {
        APP.with(|app| {
            let this = &*app.lock().unwrap();
            if let Some(this) = this.as_ref() {
                f(&this.filelist)
            } else {
                panic!("invalid state getting filelist");
            }
        })
    }

    pub fn handle(ev: &Event) {
        if let EventType::Moving { x, y } = ev.event {
            //   trace!("moving: {},{}", x, y);
        }

        let main: ::window::HWND = Self::main_hwnd();
        if ev.event == EventType::CloseRequest && ev.hwnd == main {
            // I need to do cleanup here or something
            unsafe { winuser::PostThreadMessageA(TID.with(|tid| *tid), winuser::WM_QUIT, 0, 0) };
        }
    }
}

struct ComInitialized(*mut ());
impl Drop for ComInitialized {
    fn drop(&mut self) {
        info!("droping com");
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
