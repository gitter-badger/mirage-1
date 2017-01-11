extern crate mirage_high as high;

use std::{thread, time};

#[no_mangle]
pub fn load() -> bool {
	println!("hello");

	let ten_millis = time::Duration::from_millis(10);

	thread::sleep(ten_millis);

	true
}