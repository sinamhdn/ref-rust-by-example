pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // this makes all the enum variats public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // call function in parent module of this module
        super::deliver_order();
    }

    fn cook_order() {}
}

// we need to mark function as pub to use it here
// inner modules can access outer module scope
// but outer code can only access sibiling modules or inner codes that are marked with pub
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

// this will no longer work because function in moved to another inner scope
// use crate::front_of_house::serving;
/*
fn deliver_order() {
    // we can shorten the path with the 'use' keyword
    // 'use' only creates a shortcut on that scope
    serving::take_order();
}
*/
mod server{
    // we bring parent module instead of the function itself so we always know that this function comes from another module
    use crate::front_of_house::serving;
    fn deliver_order() {
        // we can shorten the path with the 'use' keyword
        // 'use' only creates a shortcut on that scope
        serving::take_order();
    }
}

// because both enums are Result we import the outer module so that rust can distinguish between them
use std::fmt;
use std::io;
fn function1() -> fmt::Result {
    // --snip--
}
fn function2() -> io::Result<()> {
    // --snip--
}

// another way to import similar names into the scope is with "as" keyword
use std::fmt::Result as FMTResult;
use std::io::Result as IoResult;
fn function3() -> FMTResult {
    // --snip--
}
fn function4() -> IoResult<()> {
    // --snip--
}

// REEXPORTING
// we can bring a module to the scope and then reexport it for the others to import it from our module
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
// now users can import this code using restaurant::hosting::add_to_waitlist() instead of restaurant::front_of_house::hosting::add_to_waitlist()

// std is a crate that is available y default and we dont need to import it
use std::collections::HashMap;

use std::cmp::Ordering;
use std::io;
// we can summerize use statement above like this
use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;
// replaced with
use std::io::{self, Write};

// use glob operator to bring all public items defined in std::collection
use std::collections::*;

// moving modules to seperate file
// only load module to the module tree here
// other files in the package need to import this module with "use"
// in rus if we move modules to a file we need this nod declaration do that compiler know where to look for modules
mod courier;