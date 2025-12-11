use crate::pan::Pan;

pub struct SteelPan {
}

impl Pan for SteelPan {
	fn cook(&self) {
		println!("Pan : SteelPan : Cook")
	}
}
