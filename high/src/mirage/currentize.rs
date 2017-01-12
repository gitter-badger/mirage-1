use current::CurrentGuard;
use piston_window::{Event, PistonWindow};
use std::mem;

pub fn currentize<F>(window: &mut PistonWindow, fun: F) where F: Fn() {

	let mut event = None;

	let window_guard: CurrentGuard<PistonWindow> = CurrentGuard::new(window);
	let event_guard: CurrentGuard<Option<Event>> = CurrentGuard::new(&mut event);

	fun();

	mem::drop(event_guard);
	mem::drop(window_guard);
}