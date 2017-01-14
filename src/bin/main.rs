#[macro_use]
extern crate log;

extern crate libloading;
extern crate mirage;

use libloading::{Library, Symbol};
use std::{env, mem, process, str, thread};
use std::path::PathBuf;
use std::time::Duration;

#[cfg(target_os = "macos")]
const DYNAMIC_LIBRARY_EXTENSION: &'static str = "dylib";
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly"))]
const DYNAMIC_LIBRARY_EXTENSION: &'static str = "so";
#[cfg(target_os = "windows")]
const DYNAMIC_LIBRARY_EXTENSION: &'static str = "dll";

const DYNAMIC_LIBRARY_NAME: &'static str = concat!("lib", env!("CARGO_PKG_NAME"));
const SYMBOL: &'static [u8] = b"dyn_func";

fn main() {
	mirage::logger::init().expect("failed to initialize logger");

	let path_buf = {
		let exe = env::current_exe().unwrap();
		let directory = exe.parent().unwrap();
		directory.join(DYNAMIC_LIBRARY_NAME).with_extension(DYNAMIC_LIBRARY_EXTENSION)
	};

	loop {

		load_then_drop(&path_buf);

		let (secs, nanos) = (2, 0);

		thread::sleep(
			Duration::new(secs, nanos)
		);
	}
}

pub fn load_then_drop(path_buf: &PathBuf) {

	let mut arguments = vec!["build"];

	if !cfg!(debug_assertions) { arguments.push("--release") }

	info!("Building project with `cargo build`");

	let output = process::
		Command::new("cargo")
				.args(&arguments)
				.output()
				.expect("failed to execute process");

	if output.status.success() {

		info!("Calling `fn`: {}", str::from_utf8(SYMBOL).unwrap());

		let library = Library::new(path_buf).expect("failed to load library");

		unsafe {

			let func: Symbol<fn() -> Result<(), String>> = 
				library.get(SYMBOL).expect("failed to get `fn`");

			if let Err(message) = func() {

				error!("{}", message);
			}
		}

		mem::drop(library);

	} else {

		//error!("{}", output.status);
		error!("{}", String::from_utf8_lossy(&output.stdout));
		//error!("{}", String::from_utf8_lossy(&output.stderr));
	}
}