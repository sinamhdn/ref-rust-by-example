use adder; // because this file is a seperate crate we need to import the library crate

mod common;

// we don’t need to annotate any code with #[cfg(test)]
// if on of the unit tests fail rust wont run integration test and doc test
#[test]
fn it_adds_two_integration() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}