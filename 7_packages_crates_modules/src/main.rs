pub mod garden; // expose to crate

use crate::garden::vegetables::Asparagus;

fn main() {
    let plant = Asparagus{};
    println!("The plant is: {plant:#?}");
}
