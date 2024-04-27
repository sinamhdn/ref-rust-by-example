use crate::List::{Cons, Nil};

fn main() {
    // if we use 5 outside of Box it will get stored on the stack
    // but 5 here is stored on the heap
    // when box goes out of scope both Box and data it stores will get dropped (just like owned value)
    let b = Box::new(5);
    println!("b = {}", b);

    //// let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Nil)))));
}

// this wont compile without box because this is a recursive type and doesn't have a fixed size
// rust doesn't know how much memory needed to store this type on the heap
// to calculate how much value is needed for a type to get stored on the heap
// rust looks at the fields of that type to see which fields needs most space
// but in recursive types no matter how much rust checks there is still a type that needs to check infinitely
// to fix this issue instead of directly storing a value
// we store a pointer to the value in the structure and store the value indirectly
// Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to
// instead of storing items inside each other we can store items next to each other with box which is a pointer to the type itself
//// enum List {
////     Cons(i32, List),
////     Nil,
//// }
enum List {
    Cons(i32, Box<List>),
    Nil,
}
