extern crate mirage_high as high;

use high::types::PistonWindow;
use std::{thread, time};

#[no_mangle]
pub fn start(window: &mut PistonWindow) -> bool {

	high::mirage::currentize::<_, bool>(window, || {

		

		true
	})
}