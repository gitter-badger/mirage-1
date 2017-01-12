extern crate high;

use high::mirage;
use high::reexport::PistonWindow;

#[no_mangle]
pub unsafe extern fn app(window: &mut PistonWindow) {

    mirage::currentize(window, || include!("../.script"));
}