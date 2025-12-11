mod pan;
mod food;
mod steelpan;
mod potatofry;

use food::{Food};
use pan::{Pan};
use steelpan::{SteelPan};
use potatofry::{PotatoFry};

fn main() {
	let steelpan = Box::new(SteelPan{});
	steelpan.cook();

	let potatofry = Box::new(PotatoFry{pan: steelpan});
	potatofry.eat();
}
