use std::{env, process};

fn main() {

	let out = format!("{}/.main.rs", env::var("OUT_DIR").unwrap());

	let output = process::
		Command::new("mv")
				.arg(".main.rs")
				.arg(&out)
				.current_dir("../")
				.output()
				.expect("failed to execute process");

	if !output.status.success() {

		// println!("{}", output.status);
		// println!("{}", String::from_utf8_lossy(&output.stdout));
		println!("{}", String::from_utf8_lossy(&output.stderr));

		panic!()
	}
}