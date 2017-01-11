use libloading::Library;
use std::path::Path;
use std::process::Command;

use super::conf;

pub fn dylib() {

	info!("[Mirage] Rebuilding library.");

	let output = {

		Command::new("cargo")
				.arg("build")
				.current_dir(conf::DIRECTORY)
				.output()
				.expect("failed to execute process")
	};

	if output.status.success() {

		info!("[Mirage] Reloading library.");

		let path = {

			Path::new(conf::DIRECTORY)
				 .join("target/debug")
				 .join(format!("lib{}.dylib", conf::NAME))
		};

		let library = Library::new(&path).expect("failed to load dynamic library");

		unsafe {

			// Pointer to function
			let function = library.get::<fn()>(b"load").expect("failed to get point to function");

			function()
		}

	} else {

		error!("{}", output.status);
		error!("{}", String::from_utf8_lossy(&output.stdout));
		error!("{}", String::from_utf8_lossy(&output.stderr));
	}
}