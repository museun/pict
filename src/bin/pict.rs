extern crate env_logger;

extern crate pict;
use pict::*;

fn main() {
    env_logger::init();

    let app = App::new();
    app.run();
}
