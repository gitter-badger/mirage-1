use piston_window::{EventLoop, PistonWindow, WindowSettings};
use super::{WINDOW_WIDTH, WINDOW_HEIGHT};

pub fn start<F>(fun: F) where F: Fn(PistonWindow) {

	let window = {

		let settings = {

			let title = "Mirage - Interactive";
			let size = (WINDOW_WIDTH, WINDOW_HEIGHT);

			WindowSettings::new(title, size)
						   .exit_on_esc(true)
						   //.samples(0)
	           			   //.vsync(true)
	           			   .resizable(false)
	           			   .decorated(false)
	    };

	    let closure = |error| panic!("{}", error);

		settings.build::<PistonWindow>()
				.unwrap_or_else(closure)
				.ups(60)
				.max_fps(60)
	};

	fun(window);
}