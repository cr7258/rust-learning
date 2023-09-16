use crate::garden::vegetables::Asparagus;
// use garden::vegetables::Asparagus; // 也可以

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}