use current::CurrentGuard;
use lychee::image::ImageBuffer;
use piston_window::{Event, PistonWindow, Texture, TextureSettings};
use std::mem;

pub fn currentize<F>(window: &mut PistonWindow, fun: F) where F: Fn() {

	let mut texture = {
		let image_buffer = ImageBuffer::new(1280, 720);
		let settings = TextureSettings::new();

		let closure = |error| panic!("{}", error);

		Texture::from_image(&mut window.factory, &image_buffer, &settings).unwrap_or_else(closure)
	};

	let mut event = None;

	let window_guard: CurrentGuard<PistonWindow> = CurrentGuard::new(window);
	let texture_guard = CurrentGuard::new(&mut texture);
	let event_guard: CurrentGuard<Option<Event>> = CurrentGuard::new(&mut event);

	fun();

	mem::drop(event_guard);
	mem::drop(texture_guard);
	mem::drop(window_guard);
}