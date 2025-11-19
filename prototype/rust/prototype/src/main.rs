use crate::prototype::PrototypeFactory;
pub mod prototype;
fn main() {

    let pf = PrototypeFactory{};
    pf.create_fruit_prototype("fruit".to_string());

    pf.create_vegetable_prototype("vegetable".to_string());

}
