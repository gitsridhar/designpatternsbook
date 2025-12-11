use crate::pan::Pan;

pub struct CopperPan {
}

impl Pan for CopperPan {
	fn cook(&self) {
		println!("Pan : CopperPan : Cook")
	}
}
