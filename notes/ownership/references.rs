fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    change(&s);
    change_mut(&mut s);

    // this code fails because we try to have two mutable reference to a variable
    // Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);

    // we can create two mutable reference by creating a new scope
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;

    // rust doesn't allow us to create a mutable reference after an immutable reference
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    println!("{}, {}, and {}", r1, r2, r3);

    // this code works because we consume variables in print and they are not valid after that because print! take ownership
    // after print we can create a mutable variable again
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);

    // this code panics because we are trying to create a dangling reference
    let reference_to_nothing = dangle();
}

// here we pass a reference to String type instead of taking ownership
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
//   We call the action of creating a reference borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.

// this code panics because we are trying to edit an immutable reference to make it mutable add mut after &
// referenced are immutable by default just like variables
fn change(some_string: &String) {
    some_string.push_str(", world");
}

// we can now do modifications to the parameter
// Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}

// this code panics because we are trying to create a dangling reference
fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
// to fix the function above we can return the value itself instead of a reference to it
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}