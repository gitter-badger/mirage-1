#[macro_use] extern crate log;

extern crate libloading;
extern crate mirage;
extern crate piston_window;

fn main() {
	use libloading::Library;
	use piston_window::PistonWindow;
	use std::path::Path;
	use std::process::Command;

	type Fn = (fn(&mut PistonWindow) -> bool);

	fn dylib(directory: &str, package_name: &str, window: &mut PistonWindow) -> bool {

		info!("[Mirage] Rebuilding library.");

		let output = {

			Command::new("cargo")
					.arg("build")
					.current_dir(directory)
					.output()
					.expect("failed to execute process")
		};

		if output.status.success() {

			info!("[Mirage] Reloading library.");

			let path = {

				Path::new(directory)
					 .join("target/debug")
					 .join(format!("lib{}.dylib", package_name))
			};

			let library = Library::new(&path).expect("failed to load dynamic library");

			// Pointer to function
			let function = unsafe {

				library.get::<Fn>(b"load")
					   .expect("failed to get point to function")
			};

			function(window)

		} else {

			error!("{}", output.status);
			error!("{}", String::from_utf8_lossy(&output.stdout));
			error!("{}", String::from_utf8_lossy(&output.stderr));

			unimplemented!()
		}
	}

	mirage::start(|mut window| while dylib("./.dylib", "start", &mut window) {

	});
}