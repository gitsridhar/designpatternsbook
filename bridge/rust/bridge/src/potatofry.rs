use crate::pan::{Pan};
use crate::food::{Food};

pub struct PotatoFry {
	pub pan: Box<dyn Pan>,
}

impl Food for PotatoFry {
	fn eat(&self) {
		println!("Cooking before eating.");
		self.pan.cook()
	}
}
