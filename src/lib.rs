extern crate winapi;

#[macro_use]
extern crate log;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate toml;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate typed_builder;

mod error;
mod util;

pub mod config;
pub use config::Config;

mod class;
//mod control;
mod window;

// mod listview;

// mod filelist;
mod mainwindow;

mod app;
mod context;

pub use app::App;

mod event; // not sure if I'll wrap all of the messages. just here to make mainwindow happy
