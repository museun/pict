use winapi::um::winuser::*;

#[derive(Debug)]
pub enum Event {
    CloseRequest,
    Quit,
    MouseMove { x: f32, y: f32 },
    MouseDown { button: MouseButton, x: f32, y: f32 },
    MouseWheel { delta: i32, x: f32, y: f32 },
    KeyDown { key: Key },
    Resizing { width: f32, height: f32 },
    Resize { width: f32, height: f32 },
    Notify { lp: isize }, // actually an LPARAM
}

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    // X1, X2,
}

impl From<u32> for MouseButton {
    fn from(button: u32) -> MouseButton {
        match button {
            WM_LBUTTONDOWN => MouseButton::Left,
            WM_MBUTTONDOWN => MouseButton::Middle,
            WM_RBUTTONDOWN => MouseButton::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
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
            VK_SPACE => Key::Space,
            VK_UP => Key::Up,
            VK_DOWN => Key::Down,
            VK_LEFT => Key::Left,
            VK_RIGHT => Key::Right,
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
