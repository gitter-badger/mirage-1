extern crate high;

use high::mirage;

#[no_mangle]
pub fn app() -> Result<(), String> {
	
	include!("../../.main.rs");

	Ok(())
}