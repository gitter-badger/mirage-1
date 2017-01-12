extern crate high;

use high::av::Capture;
use high::mirage;
use high::reexport::{PistonWindow, Resources, Texture};

#[no_mangle]
pub fn app(capture: &mut Capture, texture: &mut Texture<Resources>, window: &mut PistonWindow) {

    mirage::currentize(capture, texture, window, || include!("../.script"));
}