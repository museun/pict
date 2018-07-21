use common::*;

#[derive(Debug, PartialEq)]
pub struct Event {
    pub event: EventType,
    pub hwnd: ::window::HWND,
}

#[derive(Debug, PartialEq)]
pub enum EventType {
    CloseRequest,
    Quit,
    MouseMove { x: i32, y: i32 },
    MouseDown { button: MouseButton, x: i32, y: i32 },
    MouseWheel { delta: i32, x: f32, y: f32 },
    KeyDown { key: Key },
    Moved { x: i32, y: i32 },
    Moving { x: i32, y: i32 },
    Resizing { width: i32, height: i32 },
    Resize { width: i32, height: i32 },
    Notify { lp: isize }, // actually an LPARAM
}

#[derive(Debug, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    // X1, X2,
}

impl From<u32> for MouseButton {
    fn from(button: u32) -> MouseButton {
        match button {
            winuser::WM_LBUTTONDOWN => MouseButton::Left,
            winuser::WM_MBUTTONDOWN => MouseButton::Middle,
            winuser::WM_RBUTTONDOWN => MouseButton::Right,
            _ => unreachable!(),
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
            _ => Key::Other(key),
        }
    }
}
