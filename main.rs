use high::{mirage, piston};

while piston::open() {


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