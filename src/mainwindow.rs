use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use common::*;

lazy_static! {
    static ref MAIN_CLASS: () = {
        Class::create("PictMainWindowClass".to_wide());
    };
}

#[derive(Debug)]
pub struct MainWindow {
    pub(crate) window: Window,
    context: Arc<Mutex<Context>>,
}

impl MainWindow {
    pub fn new(context: Arc<Mutex<Context>>) -> Self {
        ::lazy_static::initialize(&MAIN_CLASS);

        let conf = Config::get();

        let params = Params::builder()
            .class_name("PictMainWindowClass".to_wide())
            .window_name("pict".to_wide())
            .x(conf.position.x)
            .y(conf.position.y)
            .style(winuser::WS_TILEDWINDOW)
            .ex_style(winuser::WS_EX_APPWINDOW | winuser::WS_EX_ACCEPTFILES)
            .build();

        let window = Window::new(&params);
        MAIN_HWND.with(|hwnd| {
            let mut this = hwnd.lock().unwrap();
            if this.is_none() {
                *this = Some(window.hwnd())
            }
        });

        window.set_size(conf.size.w, conf.size.h);
        Self { window, context }
    }

    pub fn hwnd(&self) -> HWND {
        self.window.hwnd().into()
    }

    fn next(&self) {
        let ctx = Arc::clone(&self.context);
        let this = &mut ctx.lock().unwrap();

        let len = this.get_len();
        if len == 0 {
            debug!("can't move to next index. list empty");
            return;
        }

        let index = this.get_index();
        let next = if index + 1 == len { 0 } else { index + 1 };
        this.set_index(next);
        debug!("moving to next index: {}", next);

        App::with_filelist(|f| f.select(next));
    }

    fn previous(&self) {
        let this = &mut self.context.lock().unwrap();
        let len = this.get_len();
        if len == 0 {
            debug!("can't move to previous index. list empty");
            return;
        }

        let index = this.get_index();
        let prev = if index == 0 { len - 1 } else { index - 1 };
        this.set_index(prev);
        debug!("moving to previous index: {}", prev);

        App::with_filelist(|f| f.select(prev));
    }

    fn toggle_filelist(&self) {
        debug!("toggling filelist");

        let snap = {
            let this = self.context.lock().unwrap();
            this.get_snap()
        };

        App::with_filelist(|f| {
            if f.is_visible() {
                f.hide();
            } else {
                f.show();
                if snap {
                    f.align_to(self.hwnd().into());
                }
            }
        });
    }

    fn align_filelist(&self) {
        debug!("aligning filelist");
        {
            App::with_filelist(|f| f.align_to(self.hwnd().into()));
        }

        let this = &mut self.context.lock().unwrap();
        let snap = this.get_snap();
        this.set_snap(!snap);
    }

    fn scale(&self, key: &Key) {
        let n = match key {
            Key::Key1 => 0.5,
            Key::Key2 => 1.0,
            Key::Key3 => 1.5,
            Key::Key4 => 2.0,
            _ => unreachable!(),
        };

        debug!("scaling to {:?}", n);
    }

    fn previous_frame(&self) {
        debug!("previous frame");
    }

    fn next_frame(&self) {
        debug!("next frame");
    }

    fn toggle_playing(&self) {
        debug!("toggling playing");
    }

    fn on_key_down(&self, key: &Key) {
        match key {
            Key::Other(_) => return,
            _ => {
                trace!("on keydown: {:?}", key);
            }
        };

        match *key {
            Key::A => self.previous(),
            Key::D => self.next(),
            Key::L => self.toggle_filelist(),
            Key::K => self.align_filelist(),

            Key::Key1 | Key::Key2 | Key::Key3 | Key::Key4 => self.scale(key),

            // for animated images
            Key::Left => self.previous_frame(),
            Key::Right => self.next_frame(),
            Key::Space => self.toggle_playing(),
            _ => {}
        }
    }

    fn on_mouse_down(&self, button: &MouseButton, pos: (i32, i32)) {
        // middle click is for panning
        // right click will do nothing
        // left click maybe gets forwarded to containing controls?
        trace!("click: {:?} {},{}", button, pos.0, pos.1)
    }

    fn on_mouse_wheel(&self, delta: i16, pos: (i32, i32)) {
        // zoom in and out
        trace!("scroll: {:?} {},{}", delta, pos.0, pos.1)
    }

    // fn on_resize(&self, size: (i32, i32)) {
    //     // resize the canvas
    //     trace!("resized: {:?}", size)
    // }

    fn on_moving(&self, _pos: (i32, i32)) {
        let this = self.context.lock().unwrap();
        if this.get_snap() {
            App::with_filelist(|f| f.align_to(self.hwnd().into()));
        }
    }

    // TODO determine if we actually need to handle errors, instead of silently bailing
    fn on_drop_file<P: Into<PathBuf>>(&self, path: P) {
        fn inner(dir: &Path) -> Option<Vec<(String, usize)>> {
            debug!("file drop directory: {:?}", dir.to_str());
            let mut list = vec![]; // TODO set the capacity for this.
            for entry in fs::read_dir(&dir).ok()? {
                let entry = entry.ok()?;
                let path = entry.path();
                if !path.is_dir() {
                    // this does contain path/filename
                    // let len = path.iter().collect::<Vec<_>>().len();
                    // let res = path.iter().skip(len - 2).take(1).next().unwrap();
                    // let parent: PathBuf = res.into();
                    // let file = parent.join(path.file_name()?).to_str()?.to_string();

                    let file = path.file_name()?.to_str()?.to_string();
                    if is_accepted_image_type(&file) {
                        list.push((file, entry.metadata().ok()?.len() as usize));
                    }
                }
            }
            Some(list)
        }

        let path = &path.into();
        let dir = if path.is_dir() {
            path
        } else {
            path.parent().expect("to get parent path") // maybe this'll fail on UNC. idk
        };

        if let Some(list) = inner(dir) {
            debug!("got {} files", list.len());
            {
                let this = &mut self.context.lock().unwrap();
                this.clear_list();
                this.set_index(0);
                this.extend_list(&list);
            }

            App::with_filelist(|f| f.populate(dir.to_str().unwrap(), &list));
        } else {
            error!("cannot get a file listing for: {}", path.to_str().unwrap())
        }
    }

    pub fn handle(&self, ev: &EventType) {
        match *ev {
            //EventType::MouseMove { x, y } => {}
            EventType::MouseDown { ref button, x, y } => self.on_mouse_down(button, (x, y)),
            EventType::MouseWheel { delta, x, y } => self.on_mouse_wheel(delta, (x, y)),
            EventType::KeyDown { ref key } => self.on_key_down(key),
            //EventType::Moved { x, y } => {}
            EventType::Moving { x, y } => self.on_moving((x, y)),
            EventType::DropFile { ref file } => self.on_drop_file(&file),
            _ => return,
        }
    }
}
