//! in some data structures like graph a node is owned by multiple edges
//! and we can't cleanup node as long as there is edge attached to it
//! we use Rc (reference counting) to determine if there is any references to it
//! and the value is still in use
//! if the references are zero we can cleanup the data
//! We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last. If we knew which part would finish last, we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect.
//! we use Rc<T> only in single threaded situations

//! here we want to create a list that two seperate lists point to the start of it
//!
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // this will gives error because a value needs to get two owners
    // because we moved a to Cons in b we can't move it in c
    // we can change Cons to hold references
    // but is that case we need to specify lifetime parameters
    // but the problem with that is that every element needs to live as long as the entire list
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
    // instead of Box we wrap data inside Rc and create new references to initial data by cloning Rc<T> each Rc clone creates a new instance
    // rust wont cleanup value until all references go out of scope (here we have three references of 'a')
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    // we use Rc clone instead of cloning the main data because
    // cloning on the value creates a deep copy but Rc only counts the reference count and doesn't create a deep copy and is faster than cloning

    // here you can see how reference count changes when the value goes out of the scope
    // is Rc we have both strong_count and weak_count functions
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    //! Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only
    //! If Rc<T> allowed you to have multiple mutable references too, you might violate one of the borrowing rules
}
