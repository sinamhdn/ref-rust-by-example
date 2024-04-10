// by default cargo will see this file as a binary crate named "rustpackage"
// A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.
use crate::garden::vegetables::Asparagus;

fn main() {
    let plant = Asparagus {};
    println!("Hello, world!");
}
