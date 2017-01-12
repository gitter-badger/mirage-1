pub use self::capture::Capture;
mod capture;

use current::Current;
use lychee::image::RgbaImage;

pub fn conn() {
	let capture = unsafe { &mut *Current::<Capture>::new() };
	capture.conn()
}

pub fn read() -> RgbaImage {
	let capture = unsafe { &*Current::<Capture>::new() };
	capture.frame()
}