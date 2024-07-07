use crate::garden::vegetables::Asparagus;

pub mod garden;
pub mod vegetables;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
