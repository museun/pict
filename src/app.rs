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
    pub static APP: Mutex<Option<Arc<App>>> = Mutex::new(None);
}

#[derive(Debug)]
pub struct App {
    mainwindow: MainWindow,
    filelist: FileList,
    context: Arc<Mutex<Context>>,
}

impl App {
    pub fn new() -> Arc<Self> {
        COM_INITIALIZED.with(|_| {});

        let context = Arc::new(Mutex::new(Context::new()));
        let mainwindow = MainWindow::new(Arc::clone(&context));
        let filelist = FileList::new(Arc::clone(&context));

        let this = Arc::new(Self {
            mainwindow,
            filelist,
            context,
        });

        APP.with(|app| {
            let app = &mut *app.lock().expect("unwrap at set app");
            if app.is_none() {
                *app = Some(Arc::clone(&this));
            }
        });

        this.mainwindow.window.show();

        this
    }

    pub fn run(&self) {
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
        MAIN_HWND
            .with(|hwnd| {
                hwnd.lock()
                    .expect("unwrap lock at get MAIN_HWND")
                    .expect("second uwnrap MAIN_HWND")
            })
            .into()
    }

    fn list_hwnd() -> ::window::HWND {
        LIST_HWND
            .with(|hwnd| {
                hwnd.lock()
                    .expect("unwrap lock at get LIST_HWND")
                    .expect("second unwrap LIST_HWND")
            })
            .into()
    }

    pub fn with_mainwindow<T>(f: impl Fn(&MainWindow) -> T) -> Option<T> {
        APP.with(|app| {
            let this = &*app.lock().expect("unwrap at with_mainwindow");
            if let Some(this) = this.as_ref() {
                Some(f(&this.mainwindow))
            } else {
                None
            }
        })
    }

    pub fn with_filelist<T>(f: impl Fn(&FileList) -> T) -> Option<T> {
        APP.with(|app| {
            let this = &*app.lock().expect("unwrap at with_filelist");
            if let Some(this) = this.as_ref() {
                Some(f(&this.filelist))
            } else {
                None
            }
        })
    }

    pub fn handle(ev: &Event) {
        let main: ::window::HWND = Self::main_hwnd();
        let list: ::window::HWND = Self::list_hwnd();

        if ev.event == EventType::CloseRequest && ev.hwnd == main {
            APP.with(|app| {
                if let Some(app) = app.lock().unwrap().as_ref() {
                    app.save()
                }
            });
            // I need to do cleanup here or something
            unsafe { winuser::PostThreadMessageA(TID.with(|tid| *tid), winuser::WM_QUIT, 0, 0) };
        }

        if ev.hwnd == main {
            App::with_mainwindow(|m| m.handle(&ev.event));
        }

        if ev.hwnd == list {
            App::with_filelist(|m| m.handle(&ev.event));
        }
    }

    fn save(&self) {
        let pos = self.mainwindow.window.get_pos();
        let size = self.mainwindow.window.get_size();

        Config {
            position: Position { x: pos.0, y: pos.1 },
            size: Size {
                w: size.0 as i32,
                h: size.1 as i32,
            },
            filelist: ::config::FileList {
                snap: self
                    .context
                    .lock()
                    .expect("at unwrap for get snap")
                    .get_snap(),
            },
        }.save();
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
