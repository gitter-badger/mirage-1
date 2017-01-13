{use high::{capture, mirage, piston};

capture::conn();

while piston::open() {

	if piston::render() {
		
		let image = capture::read();

		piston::draw_image(&image)?;
	}

	if let Some(key) = piston::pressed_key() {

		if key <= 255 {

			match key as u8 as char {
				'r' => {

					// Will reload if the window is still open
					break
				},

				_ => {

				}
			}
		}
	}
}

capture::disconn();}