use av::Capture;
use lychee::image::ImageBuffer;
use piston_window::{EventLoop, PistonWindow, Texture, TextureSettings, Window, WindowSettings};
use gfx_device_gl::Resources;

pub fn start<F>(func: F) where F: Fn(&mut Capture, &mut Texture<Resources>, &mut PistonWindow) {

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

	let mut texture: Texture<Resources> = {

		let buf = ImageBuffer::new(1280, 720);
		let settings = TextureSettings::new();
		let ref mut factory = window.factory;

		Texture::from_image(factory, &buf, &settings).expect("failed to create texture")
	};

	let mut capture = Capture::init();

	loop {

		let dur = ::std::time::Duration::from_secs(2);
		::std::thread::sleep(dur);

		func(&mut capture, &mut texture, &mut window);

		if window.should_close() {

			//info!("Closing window");

			break
		}

		//info!("Reloading script");
	}
}