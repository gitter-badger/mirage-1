use high::{av, mirage, piston};

//av::conn();

while piston::open() {

	if piston::render() {
		//let image = av::read();
		
		//let _ = piston::draw_image(&image);
	}

	if let Some(key) = piston::pressed_key() {

		if key <= 255 {

			match key as u8 as char {
				'r' => {

					// reload if the window is still open
					break
				},

				_ => {

				}
			}
		}
	}
}