use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::{mem, ptr};

use winapi::um::winuser;

pub trait EventHandler {
    fn handle(&mut self, event: Event) -> ControlFlow;
}

impl<F> EventHandler for F
where
    F: FnMut(Event) -> ControlFlow,
{
    fn handle(&mut self, event: Event) -> ControlFlow {
        self(event)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ControlFlow {
    /// Continue looping and waiting for events.
    Continue,
    /// Break from the event loop.
    Break,
}

pub struct EventQueue(pub Rc<RefCell<VecDeque<Event>>>);

impl EventQueue {
    pub fn new() -> Self {
        EventQueue(Rc::new(RefCell::new(VecDeque::new())))
    }

    pub fn send(&self, event: Event) {
        self.0.borrow_mut().push_back(event);
    }

    pub fn run(self, mut handler: impl 'static + EventHandler) -> ! {
        unsafe {
            winuser::IsGUIThread(1);

            let mut msg = mem::uninitialized();
            'out: loop {
                if winuser::GetMessageW(&mut msg, ptr::null_mut(), 0, 0) == 0 {
                    error!("no message to get!");
                    break 'out;
                }

                winuser::TranslateMessage(&msg);
                winuser::DispatchMessageW(&msg);

                while let Some(event) = self.0.borrow_mut().pop_front() {
                    if ControlFlow::Break == handler.handle(event) {
                        break 'out;
                    }
                }
            }
        }

        drop(handler);
        ::std::process::exit(0);
    }
}

#[derive(Debug, PartialEq)]
pub struct Event {
    pub event: EventType,
    pub hwnd: ::window::HWND,
}

#[derive(Debug, PartialEq)]
pub enum EventType {
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
