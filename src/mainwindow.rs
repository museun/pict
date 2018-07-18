use std::fs;
use std::path::PathBuf;

use winit;

use app::{App, Handler, APP};

pub struct MainWindow {
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

    fn next(&self) {}
    fn previous(&self) {}

    fn toggle_filelist(&self) {
        APP.with(|app| {
            match *app.borrow() {
                Some(ref app) => {
                    if app.filelist.is_visible() {
                        app.filelist.hide()
                    } else {
                        app.filelist.show()
                    }
                }
                _ => {}
            };
        });
    }
    fn align_filelist(&self) {}

    fn previous_frame(&self) {}
    fn next_frame(&self) {}
    fn toggle_playing(&self) {}

    fn on_key_down(&self, key: winit::VirtualKeyCode, _mods: winit::ModifiersState) {
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
        eprintln!("click: {:?}, {:?}", button, mods)
    }

    fn on_mouse_wheel(&self, delta: winit::MouseScrollDelta, mods: winit::ModifiersState) {
        // zoom in and out
        eprintln!("scroll: {:?}, {:?}", delta, mods)
    }

    fn on_resize(&self, size: winit::dpi::LogicalSize) {
        // resize the canvas
        eprintln!("resized: {:?}", size)
    }

    // TODO determine if we actually need to handle errors, instead of silently bailing
    fn on_drop_file(&self, path: &PathBuf) -> Option<()> {
        let dir = if path.is_dir() {
            path
        } else {
            path.parent()?
        };

        let mut list = vec![]; // TODO set the capacity for this.
        for entry in fs::read_dir(&dir).ok()? {
            let path = entry.ok()?.path();
            if !path.is_dir() {
                list.push(path.to_str()?.to_string())
            }
        }

        APP.with(|app| {
            match *app.borrow() {
                Some(ref app) => {
                    app.filelist.populate(dir.to_str().unwrap(), &list);
                }
                _ => {}
            };
        });

        Some(())
    }
}

// could make this an associated type so it doesn't borrow self
impl Handler for MainWindow {
    fn handle(&self, ev: &winit::WindowEvent) {
        match ev {
            winit::WindowEvent::KeyboardInput { input, .. } => {
                if input.state == winit::ElementState::Pressed {
                    if let Some(key) = input.virtual_keycode {
                        self.on_key_down(key, input.modifiers);
                    }
                }
            }
            winit::WindowEvent::DroppedFile(path) => {
                self.on_drop_file(&path);
            }

            winit::WindowEvent::MouseWheel {
                delta, modifiers, ..
            } => {
                self.on_mouse_wheel(*delta, *modifiers);
            }

            winit::WindowEvent::MouseInput {
                state,
                button,
                modifiers,
                ..
            } => {
                if *state == winit::ElementState::Pressed {
                    self.on_mouse_down(*button, *modifiers);
                }
            }
            winit::WindowEvent::Resized(size) => {
                self.on_resize(*size);
            }
            _ => {}
        };
    }

    fn id(&self) -> winit::WindowId {
        self.id
    }
}
