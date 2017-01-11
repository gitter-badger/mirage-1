#![feature(main)]

#[macro_use] extern crate log;

extern crate libloading;

//#[main] ?
pub use app::main;

mod app;
mod conf;
mod load;