mod dish;
mod saltandpepper;
mod serving;
mod soup;
mod fruitsalad;
mod maindish;

use saltandpepper::SaltAndPepper;
use serving::Serving;
use soup::Soup;
use fruitsalad::FruitSalad;
use maindish::MainDish;
use dish::Eatable;

fn main() {
    println!("Hello, world!");

    let saltandpepper = SaltAndPepper{};

    let dinner = Serving{};
    let appetizer = Serving{};

    let soup = Soup{};
    let fruitsalad = FruitSalad{};
    let maindish = MainDish{};

    if appetizer.isComposite() {
        appetizer.addDish(eatable: soup);
        appetizer.addDish(eatable: fruitsalad);
    }

    let maincourse = Serving{};
    if maincourse.isComposite() {
        maincourse.addDish(eatable: maindish);
        maincourse.addDish(eatable: saltandpepper);
    }

    dinner.addDish(eatable: appetizer);
    dinner.addDish(eatable: maincourse);

    let preparations = dinner.prepare();
    println!("'{}'", preparations);
}
