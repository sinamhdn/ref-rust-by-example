//! Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules. To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing. Unsafe code indicates to the compiler that we’re checking the rules manually instead of relying on the compiler to check them for us
//! We can use types that use the interior mutability pattern only when we can ensure that the borrowing rules will be followed at runtime, even though the compiler can’t guarantee that. The unsafe code involved is then wrapped in a safe API, and the outer type is still immutable.
//! RefCell is similiar to Box in case that RefCell<T> type represents single ownership over the data it holds (unlike Rc)
//! difference between RefCell<T> and Box<T> is that Box<T> is enforcing borrowing rules at compile time but RefCell<T> is enforcing the at runtime
//! if you break borrowing rules with RefCell<T> program panics and closes
//! The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are then allowed, where they would’ve been disallowed by the compile-time checks.
//! there are some cases where we need to postpone borrowing rules to runetime because comple is too strict
//!
//! The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.
//!
//! Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.
//!
//! Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
//!
//!
//! Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
//!
//! Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
//!
//! Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

fn main() {
    let x = 5;
    // we get error here because we borrow mutabliy an immutable value
    let y = &mut x;
    // there are cases that we want a value to be mutable in it's methods but immutable outside
    // in this cases we use RefCell and postpone borrowing rule check to the runtime
}

//!Sometimes during testing a programmer will use a type in place of another type, in order to observe particular behavior and assert it’s implemented correctly. This placeholder type is called a test double. Think of it in the sense of a “stunt double” in filmmaking, where a person steps in and substitutes for an actor to do a particular tricky scene. Test doubles stand in for other types when we’re running tests. Mock objects are specific types of test doubles that record what happens during a test so you can assert that the correct actions took place.
//! Rust doesn’t have objects in the same sense as other languages have objects, and Rust doesn’t have mock object functionality built into the standard library as some other languages do. However, you can definitely create a struct that will serve the same purposes as a mock object.

// here the user of the library must define the messenger trait and implement how send() works (email, sms and etc.)
pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

//! we want to test the behavior of the set_value method on the LimitTracker. We can change what we pass in for the value parameter, but set_value doesn’t return anything for us to make assertions on. We want to be able to say that if we create a LimitTracker with something that implements the Messenger trait and a particular value for max, when we pass different numbers for value, the messenger is told to send the appropriate messages.
//!
//! We need a mock object that, instead of sending an email or text message when we call send, will only keep track of the messages it’s told to send. We can create a new instance of the mock object, create a LimitTracker that uses the mock object, call the set_value method on LimitTracker, and then check that the mock object has the messages we expect.
//!
#[cfg(test)]
mod tests {
    use super::*;
    struct MockMessenger {
        sent_messages: Vec<String>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                //// sent_messages: vec![],
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // for this code to work we need to get a mutable reference to self
            // but we can't change self to be mutable because send must have same signature as Trait
            // to fix ths wrap sent_message in RefCell then borrow the field variable as mutable to change it's value
            //// self.sent_messages.push(String::from(message));
            self.sent_messages.borrow_mut().push(String::from(message));

            // test will fail because we take two mutable references
            // be with RefCell code will panic at run time instead of error at compile time
            // one trade off is that error detention at run-time make the code slower
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}

//! When creating immutable and mutable references, we use the & and &mut syntax, respectively. With RefCell<T>, we use the borrow and borrow_mut methods, which are part of the safe API that belongs to RefCell<T>. The borrow method returns the smart pointer type Ref<T>, and borrow_mut returns the smart pointer type RefMut<T>. Both types implement Deref, so we can treat them like regular references.
//! 
//! The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active. Every time we call borrow, the RefCell<T> increases its count of how many immutable borrows are active. When a Ref<T> value goes out of scope, the count of immutable borrows goes down by one. Just like the compile-time borrowing rules, RefCell<T> lets us have many immutable borrows or one mutable borrow at any point in time.

//! by combining Rc<T> with RefCell<T> we can have multiple mutable references to a value
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}