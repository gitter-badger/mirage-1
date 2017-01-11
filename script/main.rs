use high::{mirage, piston};

let mut reload = true;

while piston::open() {


	if piston::esc() {

		reload = false;
		break;
	}
}

reload