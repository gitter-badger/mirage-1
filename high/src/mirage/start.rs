use piston_window::{EventLoop, PistonWindow, Window, WindowSettings};

pub fn start<F>(func: F) where F: Fn(&mut PistonWindow) {

	let mut window = {

		let settings = {

			let title = "Mirage - Interactive";
			let size = [1280, 720];

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
				.ups(30)
				.max_fps(30)
	};

	loop {

		func(&mut window);

		if window.should_close() {

			info!("Closing window");

			break
		}

		info!("Reloading script");
	}
}