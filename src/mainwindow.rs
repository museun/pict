use std::fs;
use std::path::PathBuf;

use winit;

use app::{App, Handler};

pub struct MainWindow {
    #[allow(dead_code)]
    window: winit::Window,
    id: winit::WindowId,
}

impl MainWindow {
    pub fn new(events: &winit::EventsLoop) -> Self {
        // TODO: load/save the position and size

        let window = winit::WindowBuilder::new()
            .with_title("pict")
            .with_dimensions((400, 200).into())
            .with_resizable(true)
            .build(&events)
            .unwrap();

        let id = window.id();
        Self { window, id }
    }

    fn next(&self) {
        let len = App::get_list_len();
        if len == 0 {
            debug!("can't move to next index. list empty");
            return;
        }

        App::with_context(|app| {
            let index = { app.lock().unwrap().get_index() };
            let index = if index == len {
                app.lock().unwrap().set_index(0);
                0
            } else {
                app.lock().unwrap().set_index(index + 1);
                index + 1
            };
            debug!("moving to next index: {}", index);
        });

        App::update_filelist_index();
    }

    fn previous(&self) {
        let len = App::get_list_len();
        if len == 0 {
            debug!("can't move to previous index. list empty");
            return;
        }

        App::with_context(|app| {
            let (index, len) = {
                let app = app.lock().unwrap();
                (app.get_index(), app.get_len())
            };

            if index == 0 {
                app.lock().unwrap().set_index(len);
            } else {
                app.lock().unwrap().set_index(index - 1)
            }
            debug!("moving to previous index: {}", index);
        });

        App::update_filelist_index();
    }

    fn toggle_filelist(&self) {
        debug!("toggling filelist");
        App::with_context(|app| {
            let filelist = { &app.lock().unwrap().filelist };
            if filelist.is_visible() {
                filelist.hide()
            } else {
                filelist.show()
            }
        });
    }

    fn align_filelist(&self) {
        debug!("aligning filelist");
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
            let filelist = { &app.lock().unwrap().filelist };
            filelist.populate(dir.to_str().unwrap(), &list);
        });

        App::with_context(|app| {
            let app = &mut app.lock().unwrap();
            app.set_index(0);
            app.clear_list();
            app.extend_list(&list);
        });

        Some(())
    }
}

impl Handler for MainWindow {
    fn handle(&self, ev: &winit::WindowEvent) {
        match *ev {
            winit::WindowEvent::KeyboardInput { input, .. } => {
                if input.state == winit::ElementState::Pressed {
                    if let Some(key) = input.virtual_keycode {
                        self.on_key_down(key, input.modifiers);
                    }
                }
            }
            winit::WindowEvent::DroppedFile(ref path) => {
                self.on_drop_file(&path);
            }

            winit::WindowEvent::MouseWheel {
                delta, modifiers, ..
            } => {
                self.on_mouse_wheel(delta, modifiers);
            }

            winit::WindowEvent::MouseInput {
                state,
                button,
                modifiers,
                ..
            } => {
                if state == winit::ElementState::Pressed {
                    self.on_mouse_down(button, modifiers);
                }
            }
            winit::WindowEvent::Resized(size) => {
                self.on_resize(size);
            }
            _ => {}
        };
    }

    fn id(&self) -> winit::WindowId {
        self.id
    }
}
