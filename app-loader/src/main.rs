#[macro_use] extern crate log;
extern crate high;
extern crate libloading as lib;

fn main() {

use lib::Library;
use high::mirage;
use high::av::Capture;
use high::reexport::{PistonWindow, Resources, Texture};
use log::{Log, LogLevel, LogLevelFilter, LogMetadata, LogRecord};
use std::{fs, io, process, thread, time};
use std::path::Path;
use std::io::Write;

const LIB_DIRECTORY: &'static str = "../app";
const LIB_PATH: &'static str = "../app/target/debug/libapp";
const SCRIPT_PATH: &'static str = "../main.rs";
const SCRIPT_OUTPUT_PATH: &'static str = "../app/.script";

#[cfg(target_os = "macos")]
const LIB_EXT: &'static str = "dylib";
#[cfg(target_os = "linux")]
const LIB_EXT: &'static str = "so";
#[cfg(target_os = "windows")]
const LIB_EXT: &'static str = "dll";

struct Logger;

impl Log for Logger {
	fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("[{}] - {}", record.level().to_string().to_lowercase(), record.args());
        }
    }
}

log::set_logger(|f| {f.set(LogLevelFilter::Info); Box::new(Logger)}).expect("failed to set logger");

struct Application { lib: Library }

impl Application {

	fn new() -> lib::Result<Application> {
		info!("Building library.");

		let path = Path::new(LIB_PATH).with_extension(LIB_EXT);
		let lib = Library::new(path)?;

		Ok(Application { lib: lib })
	}

	fn app(&self, capture: &mut Capture, texture: &mut Texture<Resources>, window: &mut PistonWindow) 
		-> lib::Result<()>
	{

		type Fn = unsafe extern fn(&mut Capture, &mut Texture<Resources>, &mut PistonWindow);

		unsafe {

			let func: lib::Symbol<Fn> = self.lib.get(b"app")?;

			Ok(func(capture, texture, window))
		}
	}
}

fn wait_for_changes() {
	let last_modified = fs::metadata(SCRIPT_PATH).unwrap().modified().unwrap();
	let dur = time::Duration::from_secs(2);

	loop {
		thread::sleep(dur);

		if let Ok(Ok(modified)) = fs::metadata(SCRIPT_PATH).map(|m| m.modified()) {

	        if modified > last_modified {
				break
	        }
	    }
	}
}

fn app(capture: &mut Capture, texture: &mut Texture<Resources>, window: &mut PistonWindow) {
	// open script
	let mut source = fs::File::open(SCRIPT_PATH).expect("failed to open script");

	let destination_path = Path::new(SCRIPT_OUTPUT_PATH);
	// delete old generated script
	let _ = fs::remove_file(&destination_path);
	// create a blank file
	let mut destination = fs::File::create(&destination_path).expect("failed to move script");

	let _ = write!(destination, "{{");
	let _ = io::copy(&mut source, &mut destination).expect("failed to copy content from `source`");
	let _ = write!(destination, "}}");

	let output = process::
		Command::new("cargo")
				.arg("build")
				.current_dir(LIB_DIRECTORY)
				.output()
				.expect("failed to execute process");

	if output.status.success() {

		info!("Loading library.");

		let application = Application::new().expect("failed to load library");

		info!("Library successfully loaded.");

		application.app(capture, texture, window).expect("failed to call `fn`"); 

	} else {

		error!("{}", output.status);
		error!("{}", String::from_utf8_lossy(&output.stdout));
		error!("{}", String::from_utf8_lossy(&output.stderr));

		info!("waiting for changes");
		wait_for_changes();
		info!("Recompiling");

		app(capture, texture, window)
	}
}

mirage::start(app);


	// loop {

	// 	let dur = ::std::time::Duration::from_secs(2);
	// 	::std::thread::sleep(dur);

	// 	let application = app();

	// 	application.app().expect("failed to call `fn`");

	// 	// if window.should_close() {

	// 	// 	info!("Closing window");

	// 	// 	break
	// 	// }

	// 	::std::mem::drop(application);

	// 	info!("Reloading script");
	// }





}
