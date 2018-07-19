extern crate env_logger;
extern crate winit;

extern crate pict;
use pict::*;

fn main() {
    env_logger::init();

    let mut app = App::new();
    app.run();
}
