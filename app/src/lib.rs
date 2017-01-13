extern crate high;

use high::mirage;

#[no_mangle]
pub fn app() -> Result<(), String> {
	
	Ok(include!(concat!(env!("OUT_DIR"), "/.main.rs")))
}