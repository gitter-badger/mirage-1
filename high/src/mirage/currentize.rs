use av::Capture;
use current::CurrentGuard;
use gfx_device_gl::Resources;
use piston_window::{Event, PistonWindow, Texture};
use std::mem;

pub fn currentize<F>(
	capture: &mut Capture, 
	texture: &mut Texture<Resources>, 
	window: &mut PistonWindow, 
	func: F) where F: Fn()
{

	let mut event = None;

	let capture_guard: CurrentGuard<Capture> = CurrentGuard::new(capture);
	let window_guard: CurrentGuard<PistonWindow> = CurrentGuard::new(window);
	let texture_guard = CurrentGuard::new(texture);
	let event_guard: CurrentGuard<Option<Event>> = CurrentGuard::new(&mut event);

	func();

	mem::drop(event_guard);
	mem::drop(texture_guard);
	mem::drop(window_guard);
	mem::drop(capture_guard);
}