use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

use winit;

use winapi::shared::windef::HWND;

use config::{self, Config, Position, Size};
use context::Context;
use filelist::FileList;
use mainwindow::MainWindow;

thread_local!{
    pub static APP: RefCell<Option<Mutex<Context>>> = RefCell::new(None);
}

pub trait Handler {
    fn handle(&self, ev: &winit::WindowEvent);
    fn id(&self) -> winit::WindowId;
    fn hwnd(&self) -> HWND;
}

pub struct App {
    events: winit::EventLoop,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let config = Config::load();
        let events = winit::EventLoop::new();

        // set up a thread local reference to the context
        APP.with(|app| {
            let app = &mut *app.borrow_mut();
            if app.is_none() {
                *app = Some(Mutex::new(Context::new(&events, &config)))
            }
        });

        Self { events }
    }

    fn save() {
        let mainwindow = Self::get_mainwindow();
        let pos = mainwindow.get_position();
        let size = mainwindow.get_size();

        Config {
            position: Position { x: pos.0, y: pos.1 },
            size: Size {
                w: size.0,
                h: size.1,
            },
            filelist: config::FileList {
                snap: App::with_context(|app| app.lock().unwrap().get_snap()),
            },
        }.save();
    }

    pub fn run(self) {
        let (mainwindow, filelist) = Self::with_context(|app| {
            let app = app.lock().unwrap();
            (Rc::clone(&app.mainwindow), Rc::clone(&app.filelist))
        });

        self.events
            .run_forever(move |ev, _: &winit::EventLoop| match ev {
                winit::Event::WindowEvent { event, window_id } => {
                    if window_id == mainwindow.id() {
                        // if the mainwindow gets a closerequested, shut down the event loop
                        if let winit::WindowEvent::CloseRequested = event {
                            Self::save();
                            return winit::ControlFlow::Break;
                        }

                        // if let winit::WindowEvent::CustomMove(l, t, r, b) = event {
                        //     eprintln!("{},{},{},{}", l, t, r, b);
                        // }

                        // mainwindow.handle(&event)
                        // } else if window_id == filelist.id() {
                        //     filelist.handle(&event)
                        // }
                    }
                    winit::ControlFlow::Continue
                }
                winit::Event::Suspended(ok) => {
                    eprintln!("suspend: {}", ok);
                    winit::ControlFlow::Continue
                }
                winit::Event::Awakened => {
                    eprintln!("awakened!");
                    winit::ControlFlow::Continue
                }
                _ => winit::ControlFlow::Continue,
            });
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
