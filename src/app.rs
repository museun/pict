use std::cell::RefCell;
use std::rc::Rc;

use winit;

use filelist::FileList;
use mainwindow::MainWindow;

thread_local!{
    pub static APP: RefCell<Option<Box<Rc<Context>>>> = RefCell::new(None);
}

pub struct Context {
    pub mainwindow: MainWindow,
    pub filelist: FileList,
}

impl Context {
    pub fn new(events: &winit::EventsLoop) -> Self {
        Self {
            mainwindow: MainWindow::new(&events),
            filelist: FileList::new(&events),
        }
    }
}
pub trait Handler {
    fn handle(&self, ev: &winit::WindowEvent);
    fn id(&self) -> winit::WindowId;
}

pub struct App {
    events: winit::EventsLoop,
}

impl App {
    pub fn new() -> Self {
        let events = winit::EventsLoop::new();
        Self { events }
    }

    pub fn run<'a>(&'a mut self) {
        APP.with(|app| {
            let app = &mut *app.borrow_mut();
            if app.is_none() {
                *app = Some(Box::new(Rc::new(Context::new(&self.events))))
            }
        });

        let ctx = APP.with(|app| match *app.borrow() {
            Some(ref app) => Rc::clone(&app),
            None => unreachable!(),
        });

        let mainwindow = &ctx.mainwindow;
        let filelist = &ctx.filelist;

        let events = &mut self.events;
        events.run_forever(|ev| match ev {
            winit::Event::WindowEvent { event, window_id } => {
                if window_id == mainwindow.id() {
                    mainwindow.handle(&event)
                } else if window_id == filelist.id() {
                    filelist.handle(&event)
                }
                winit::ControlFlow::Continue
            }
            _ => winit::ControlFlow::Continue,
        });
    }
}
