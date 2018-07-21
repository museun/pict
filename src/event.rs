use common::*;

#[derive(Debug, PartialEq)]
pub struct Event {
    pub event: EventType,
    pub hwnd: ::window::HWND,
}

#[derive(Debug, PartialEq)]
pub enum EventType {
    CloseRequest,                                      // done
    Quit,                                              // done
    MouseMove { x: i32, y: i32 },                      // done
    MouseDown { button: MouseButton, x: i32, y: i32 }, // done
    MouseWheel { delta: i16, x: i32, y: i32 },         // done
    KeyDown { key: Key },                              // done
    Moved { x: i32, y: i32 },                          // done
    Moving { x: i32, y: i32 },                         // done
    Resizing { width: i32, height: i32 },              // ?
    Resize { width: i32, height: i32 },                // ?
    DropFile { file: String },                         // done
    Notify { lp: isize },                              // done | actually an LPARAM
}

#[derive(Debug, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    // X1, X2,
}

impl From<usize> for MouseButton {
    fn from(button: usize) -> MouseButton {
        match button {
            winuser::MK_LBUTTON => MouseButton::Left,
            winuser::MK_MBUTTON => MouseButton::Middle,
            winuser::MK_RBUTTON => MouseButton::Right,
            _ => {
                error!("uh oh: {}", button);
                unreachable!()
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Key {
    Space,
    Up,
    Down,
    Left,
    Right,
    Key1,
    Key2,
    Key3,
    Key4,
    A,
    D,
    K,
    L,
    R,
    Other(i32),
}

impl From<i32> for Key {
    fn from(key: i32) -> Key {
        match key {
            winuser::VK_SPACE => Key::Space,
            winuser::VK_UP => Key::Up,
            winuser::VK_DOWN => Key::Down,
            winuser::VK_LEFT => Key::Left,
            winuser::VK_RIGHT => Key::Right,
            0x31 => Key::Key1,
            0x32 => Key::Key2,
            0x33 => Key::Key3,
            0x34 => Key::Key4,
            0x41 => Key::A,
            0x44 => Key::D,
            0x4B => Key::K,
            0x4C => Key::L,
            0x52 => Key::R,
            _ => Key::Other(key),
        }
    }
}
