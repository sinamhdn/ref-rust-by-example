//! Associated types connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures. The implementor of a trait will specify the concrete type to be used instead of the placeholder type for the particular implementation. That way, we can define a trait that uses some types without needing to know exactly what those types are until the trait is implemented.
//! The difference is that when using generics, as in Listing 19-13, we must annotate the types in each implementation; because we can also implement Iterator<String> for Counter or any other type, we could have multiple implementations of Iterator for Counter. In other words, when a trait has a generic parameter, it can be implemented for a type multiple times, changing the concrete types of the generic type parameters each time. When we use the next method on Counter, we would have to provide type annotations to indicate which implementation of Iterator we want to use.
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

//! When we use generic type parameters, we can specify a default concrete type for the generic type. This eliminates the need for implementors of the trait to specify a concrete type if the default type works. You specify a default type when declaring a generic type with the <PlaceholderType=ConcreteType> syntax.
//! Rust doesn’t allow you to create your own operators or overload arbitrary operators. But you can overload the operations and corresponding traits listed in std::ops by implementing the traits associated with the operator. For example, in Listing 19-14 we overload the + operator to add two Point instances together. We do this by implementing the Add trait on a Point struct
use std::ops::Add;
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
//! add trait looks like this with default generic
trait Add<Rhs=Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}
//! We have two structs, Millimeters and Meters, holding values in different units. This thin wrapping of an existing type in another struct is known as the newtype pattern, which we describe in more detail in the “Using the Newtype Pattern to Implement External Traits on External Types” section. We want to add values in millimeters to values in meters and have the implementation of Add do the conversion correctly. We can implement Add for Millimeters with Meters as the Rhs here we overwrite default generic
use std::ops::Add;
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
//! You’ll use default type parameters in two main ways:
//!! To extend a type without breaking existing code
//!! To allow customization in specific cases most users won’t need

//! if you want to have multiple methods with the same name by default rust uses the method that implemented on the type
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
fn main() {
    let person = Human;

    // to use other fly methods from traits use this syntax
    Pilot::fly(&person);
    Wizard::fly(&person);

    person.fly();
    Human::fly(&person); // we can also call this function like this
}

//! associated functions that are not methods don’t have a self parameter. When there are multiple types or traits that define non-method functions with the same function name, Rust doesn't always know which type you mean unless you use fully qualified syntax.
//! trait Animal {
fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());

    // here if we want to call method from trait we can't do this
    // compiler thinks that we are trying to call baby_name on trait
    //// println!("A baby dog is called a {}", Animal::baby_name());
    // instead we use fully qualified syntax
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

//! Sometimes, you might write a trait definition that depends on another trait: for a type to implement the first trait, you want to require that type to also implement the second trait. You would do this so that your trait definition can make use of the associated items of the second trait. The trait your trait definition is relying on is called a supertrait of your trait.
use std::fmt;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
fn main() {}
//! if we try ti implement OutlinePrint on a type that doesn't implement Display trait we get an error
struct Point {
    x: i32,
    y: i32,
}
impl OutlinePrint for Point {}
use std::fmt;
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

//! we mentioned the orphan rule that states we’re only allowed to implement a trait on a type if either the trait or the type are local to our crate.
//! It’s possible to get around this restriction using the newtype pattern, which involves creating a new type in a tuple struct.
//! Because the wrapper type is local to our crate, and we can implement the trait on the wrapper.
//! we can implement Display for Vec which both are in exteral code for each other like this
//! one downside is that we would have to implement every method of the Vec in the wrapper if we want to use them
//! we can call parent methods in side this struct using self.0.method()
use std::fmt;
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}