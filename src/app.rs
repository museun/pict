use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

use winit;

use context::Context;
use filelist::FileList;
use mainwindow::MainWindow;

thread_local!{
    pub static APP: RefCell<Option<Mutex<Context>>> = RefCell::new(None);
}

pub trait Handler {
    fn handle(&self, ev: &winit::WindowEvent);
    fn id(&self) -> winit::WindowId;
}

pub struct App {
    events: winit::EventsLoop,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let events = winit::EventsLoop::new();
        Self { events }
    }

    pub fn run(&mut self) {
        // this is a special case
        APP.with(|app| {
            let app = &mut *app.borrow_mut();
            if app.is_none() {
                *app = Some(Mutex::new(Context::new(&self.events)))
            }
        });

        let (mainwindow, filelist) = Self::with_context(|app| {
            let app = app.lock().unwrap();
            (Rc::clone(&app.mainwindow), Rc::clone(&app.filelist))
        });

        let events = &mut self.events;
        events.run_forever(|ev| match ev {
            winit::Event::WindowEvent { event, window_id } => {
                if window_id == mainwindow.id() {
                    // if the mainwindow gets a closerequested, shut down the event loop
                    if let winit::WindowEvent::CloseRequested = event {
                        return winit::ControlFlow::Break;
                    }

                    mainwindow.handle(&event)
                } else if window_id == filelist.id() {
                    filelist.handle(&event)
                }
                winit::ControlFlow::Continue
            }
            _ => winit::ControlFlow::Continue,
        });
    }

    pub fn update_filelist_index() {
        Self::with_context(|app| {
            let (filelist, index) = {
                let app = app.lock().unwrap();
                (Rc::clone(&app.filelist), app.get_index())
            };

            trace!("selecting index");
            filelist.select(index);
        });
    }

    pub fn get_list_len() -> usize {
        Self::with_context(|app| app.lock().unwrap().get_len())
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
