use std::fs;
use std::path::PathBuf;
use std::{mem, ptr};

use winapi::shared::{minwindef, windef};
use winapi::um::{commctrl, winuser};

use app::App;
use class::Class;
use config::Config;
use error::*;
use event::{self, *};
use util::*;
use window::{Params, Window};

lazy_static! {
    pub static ref MAIN_WINDOW_CLASS: Vec<u16> = {
        let name = "PictMainWindowClass".to_wide();
        Class::create(name.as_ptr());
        name
    };
}

#[repr(C)]
pub struct MainWindow {
    window: Window,
}

impl MainWindow {
    pub fn new(conf: &Config) -> Self {
        let x = conf.position.x;
        let y = conf.position.y;
        let w = conf.size.w;
        let h = conf.size.h;

        let params = Params::builder().x(x).y(y).width(w).height(h)
        .class_name(MAIN_WINDOW_CLASS.as_ptr())
        .window_name("pict".to_wide().as_ptr())
        .style(winuser::WS_TILEDWINDOW) // what styles do I need?
        .ex_style(winuser::WS_EX_APPWINDOW | winuser::WS_EX_ACCEPTFILES)
        .build();

        let window = Window::new(params);
        window.show(); // nShowCmd what do

        Self { window }
    }

    pub fn hwnd(&self) -> windef::HWND {
        self.window.hwnd()
    }

    fn next(&self) {
        let len = App::get_list_len();
        if len == 0 {
            debug!("can't move to next index. list empty");
            return;
        }

        let index = App::get_index();
        let next = if index + 1 == len { 0 } else { index + 1 };
        App::set_index(next);
        debug!("moving to next index: {}", next);

        App::get_filelist().select(next);
    }

    fn previous(&self) {
        let len = App::get_list_len();
        if len == 0 {
            debug!("can't move to previous index. list empty");
            return;
        }

        let index = App::get_index();
        let prev = if index == 0 { len - 1 } else { index - 1 };
        App::set_index(prev);
        debug!("moving to previous index: {}", prev);

        App::get_filelist().select(prev);
    }

    fn toggle_filelist(&self) {
        debug!("toggling filelist");
        let filelist = App::get_filelist();
        if filelist.is_visible() {
            filelist.hide()
        } else {
            filelist.show()
        }
    }

    fn align_filelist(&self) {
        debug!("aligning filelist");
        App::get_filelist().align_to(self.hwnd());
        App::with_context(|app| {
            let mut app = app.lock().unwrap();
            let snap = app.get_snap();
            app.set_snap(!snap);
        })
    }

    fn scale(&self, key: event::Key) {
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

    fn on_key_down(&self, key: event::Key) {
        trace!("on keydown: {:?}", key);
        match key {
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

    fn on_mouse_down(&self, button: event::MouseButton) {
        // middle click is for panning
        // right click will do nothing
        // left click maybe gets forwarded to containing controls?
        trace!("click: {:?}", button)
    }

    fn on_mouse_wheel(&self, delta: i32) {
        // zoom in and out
        trace!("scroll: {:?}", delta)
    }

    fn on_resize(&self, size: (f32, f32)) {
        // resize the canvas
        trace!("resized: {:?}", size)
    }

    fn on_moved(&self, pos: (f32, f32)) {
        trace!("moved: {:?}", pos);
        if App::with_context(|app| app.lock().unwrap().get_snap()) {
            App::get_filelist().align_to(self.hwnd());
        }
    }

    // TODO determine if we actually need to handle errors, instead of silently bailing
    fn on_drop_file(&self, path: &PathBuf) -> Option<()> {
        let dir = if path.is_dir() {
            path
        } else {
            path.parent()?
        };

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

        debug!("got {} files", list.len());
        App::with_context(|app| {
            let app = &mut app.lock().unwrap();
            app.clear_list();
            app.set_index(0);
            app.extend_list(&list);
        });

        App::get_filelist().populate(dir.to_str().unwrap(), &list);

        Some(())
    }

    unsafe extern "system" fn wndproc(
        hwnd: windef::HWND,
        msg: minwindef::UINT,
        wp: minwindef::WPARAM,
        lp: minwindef::LPARAM,
        id: usize,
        data: usize,
    ) -> minwindef::LRESULT {
        use winapi::um::winuser::*;

        match msg {
            WM_NCDESTROY => {
                if commctrl::RemoveWindowSubclass(hwnd, Some(MainWindow::wndproc), id) != 0 {
                    let err = get_last_windows_error();
                    error!("cannot removed windowsubclass: {}", err)
                }
                commctrl::DefSubclassProc(hwnd, msg, wp, lp)
            }
            _ => commctrl::DefSubclassProc(hwnd, msg, wp, lp),
        }
    }
}

// use winit::WindowEvent as E;

// match *ev {
//     E::KeyboardInput { input, .. } => {
//         if input.state == winit::ElementState::Pressed {
//             if let Some(key) = input.virtual_keycode {
//                 self.on_key_down(key, input.modifiers);
//             }
//         }
//     }
//     E::DroppedFile(ref path) => {
//         self.on_drop_file(&path);
//     }
//     E::CustomMove(left, top, right, bottom) => {
//         eprintln!("{},{},{},{}", left, right, top, bottom);
//     }

//     E::Moved(pos) => self.on_moved(pos),
//     E::MouseWheel {
//         delta, modifiers, ..
//     } => self.on_mouse_wheel(delta, modifiers),
//     E::MouseInput {
//         state,
//         button,
//         modifiers,
//         ..
//     } => {
//         if state == winit::ElementState::Pressed {
//             self.on_mouse_down(button, modifiers);
//         }
//     }
//     E::Resized(size) => self.on_resize(size),
//     _ => {}
// };
