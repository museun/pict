use std::fs;
use std::path::PathBuf;

use winit;
use winit::os::windows::WindowExt;

use winapi::shared::windef::HWND;

use app::{App, Handler};
use config::Config;

pub struct MainWindow {
    #[allow(dead_code)]
    window: winit::Window,
    id: winit::WindowId,
}

impl MainWindow {
    pub fn new(events: &winit::EventsLoop, conf: &Config) -> Self {
        // this is gonna flash while the window is being moved.
        let window = winit::WindowBuilder::new()
            .with_title("pict")
            .with_dimensions((conf.size.w, conf.size.h).into())
            .with_resizable(true)
            .build(&events)
            .unwrap();

        window.set_position((conf.position.x, conf.position.y).into());

        let id = window.id();
        Self { window, id }
    }

    pub fn get_position(&self) -> (f64, f64) {
        let pos = self.window.get_position().expect("to get position");
        (pos.x, pos.y)
    }

    pub fn get_size(&self) -> (f64, f64) {
        let size = self.window.get_inner_size().expect("to get size");
        (size.width, size.height)
    }

    fn next(&self) {
        let len = App::get_list_len();
        if len == 0 {
            debug!("can't move to next index. list empty");
            return;
        }

        let index = App::get_index();
        let next = if index == len { 0 } else { index + 1 };
        App::set_index(next);
        debug!("moving to next index: {}", next);
    }

    fn previous(&self) {
        let len = App::get_list_len();
        if len == 0 {
            debug!("can't move to previous index. list empty");
            return;
        }

        let index = App::get_index();
        let prev = if index == 0 { len } else { index - 1 };
        App::set_index(prev);
        debug!("moving to previous index: {}", prev);
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
        App::get_filelist().align_to(self.window.get_hwnd() as HWND);
        App::with_context(|app| {
            let mut app = app.lock().unwrap();
            let snap = app.get_snap();
            app.set_snap(!snap);
        })
    }

    fn scale(&self, key: winit::VirtualKeyCode) {
        let n = match key {
            winit::VirtualKeyCode::Key1 => 0.5,
            winit::VirtualKeyCode::Key2 => 1.0,
            winit::VirtualKeyCode::Key3 => 1.5,
            winit::VirtualKeyCode::Key4 => 2.0,
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

    fn on_key_down(&self, key: winit::VirtualKeyCode, mods: winit::ModifiersState) {
        trace!("on keydown: {:?} | {:?}", key, mods);
        match key {
            winit::VirtualKeyCode::A => self.previous(),
            winit::VirtualKeyCode::D => self.next(),
            winit::VirtualKeyCode::L => self.toggle_filelist(),
            winit::VirtualKeyCode::K => self.align_filelist(),

            winit::VirtualKeyCode::Key1
            | winit::VirtualKeyCode::Key2
            | winit::VirtualKeyCode::Key3
            | winit::VirtualKeyCode::Key4 => self.scale(key),

            // for animated images
            winit::VirtualKeyCode::Left => self.previous_frame(),
            winit::VirtualKeyCode::Right => self.next_frame(),
            winit::VirtualKeyCode::Space => self.toggle_playing(),
            _ => {}
        }
    }

    fn on_mouse_down(&self, button: winit::MouseButton, mods: winit::ModifiersState) {
        // middle click is for panning
        // right click will do nothing
        // left click maybe gets forwarded to containing controls?
        trace!("click: {:?}, {:?}", button, mods)
    }

    fn on_mouse_wheel(&self, delta: winit::MouseScrollDelta, mods: winit::ModifiersState) {
        // zoom in and out
        trace!("scroll: {:?}, {:?}", delta, mods)
    }

    fn on_resize(&self, size: winit::dpi::LogicalSize) {
        // resize the canvas
        trace!("resized: {:?}", size)
    }

    fn on_moved(&self, pos: winit::dpi::LogicalPosition) {
        trace!("moved: {:?}", pos);
        if App::with_context(|app| app.lock().unwrap().get_snap()) {
            App::get_filelist().align_to(self.window.get_hwnd() as HWND);
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
            let path = entry.ok()?.path();
            if !path.is_dir() {
                list.push(path.to_str()?.to_string())
            }
        }
        debug!("got {} files", list.len());

        App::with_context(|app| {
            let app = &mut app.lock().unwrap();
            app.set_index(0);
            app.clear_list();
            app.extend_list(&list);
        });

        App::get_filelist().populate(dir.to_str().unwrap(), &list);

        Some(())
    }
}

impl Handler for MainWindow {
    fn handle(&self, ev: &winit::WindowEvent) {
        use winit::WindowEvent as E;

        match *ev {
            E::KeyboardInput { input, .. } => {
                if input.state == winit::ElementState::Pressed {
                    if let Some(key) = input.virtual_keycode {
                        self.on_key_down(key, input.modifiers);
                    }
                }
            }
            E::DroppedFile(ref path) => {
                self.on_drop_file(&path);
            }
            E::Moved(pos) => self.on_moved(pos),
            E::MouseWheel {
                delta, modifiers, ..
            } => self.on_mouse_wheel(delta, modifiers),
            E::MouseInput {
                state,
                button,
                modifiers,
                ..
            } => {
                if state == winit::ElementState::Pressed {
                    self.on_mouse_down(button, modifiers);
                }
            }
            E::Resized(size) => self.on_resize(size),
            _ => {}
        };
    }

    fn id(&self) -> winit::WindowId {
        self.id
    }

    fn hwnd(&self) -> HWND {
        self.window.get_hwnd() as HWND
    }
}
