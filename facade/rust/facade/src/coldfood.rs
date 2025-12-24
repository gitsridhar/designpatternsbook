pub struct ColdFood {}

impl ColdFood {
    pub fn washAndRinse(&self) -> &str {
        "ColdFood Washed And Rinsed"
    }

    pub fn wrap(&self) -> &str {
        "ColdFood Wrapped"
    }

    pub fn freeze(&self) -> &str {
        "ColdFood frozen"
    }
}