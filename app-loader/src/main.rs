// `cargo run --release --manifest-path app-loader/Cargo.toml`
//
// `cd app-loader && cargo run --release`

#[macro_use] extern crate log;
extern crate high;
extern crate libloading;

use libloading::{Library, Symbol};
use high::mirage;
use log::{Log, LogLevel, LogLevelFilter, LogMetadata, LogRecord, MaxLogLevelFilter};
use std::{fs, io, mem, process, thread, time};
use std::path::Path;
use std::io::Write;

const LIB_DIRECTORY: &'static str = "../app";
//const LIB_PATH: &'static str = "../app/target/debug/libapp";

#[cfg(debug_assertions)]
const LIB_PATH: &'static str = "../app/target/debug/libapp";

#[cfg(not(debug_assertions))]
const LIB_PATH: &'static str = "../app/target/release/libapp";

const SCRIPT_PATH: &'static str = "../main.rs";
const SCRIPT_OUTPUT_PATH: &'static str = "../.main.rs";

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

fn wait_for_changes() {
	info!("waiting for changes");

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

	info!("Recompiling");

	application()
}

fn call_dynamic<F>(closure: F) where F: Fn(Result<(), String>) {

	info!("Building library.");

	let path = Path::new(LIB_PATH).with_extension(LIB_EXT);
	let lib = Library::new(path).expect("failed to load library");

	info!("Library successfully loaded.");

	let result = {
		let func: Symbol<fn() -> Result<(), String>> = unsafe {

			lib.get(b"app").expect("failed to get `fn`") 
		};

		func()
	};

	mem::drop(lib);

	closure(result);
}

fn application() {
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

	let mut args = vec!["build"];

	if !cfg!(debug_assertions) { args.push("--release") }

	let output = process::
		Command::new("cargo")
				.args(&args)
				.current_dir(LIB_DIRECTORY)
				.output()
				.expect("failed to execute process");

	if output.status.success() {

		let closure = |result| if let Err(st) = result {
			error!("{}", st);
			wait_for_changes();
		};

		call_dynamic(closure);

	} else {

		error!("{}", output.status);
		error!("{}", String::from_utf8_lossy(&output.stdout));
		error!("{}", String::from_utf8_lossy(&output.stderr));

		wait_for_changes();
	}
}

fn main() {
	let make_logger = |filter: MaxLogLevelFilter| -> Box<Log> {
		filter.set(LogLevelFilter::Info);
		Box::new(Logger)
	};

	log::set_logger(make_logger).expect("failed to set logger");

	mirage::currentize(application);
}
