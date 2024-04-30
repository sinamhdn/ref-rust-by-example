//! NEW TYPE
//!  If we wrote a function with a parameter of type Millimeters, we couldn’t compile a program that accidentally tried to call that function with a value of type Meters or a plain u32.
//! We can also use the newtype pattern to abstract away some implementation details of a type: the new type can expose a public API that is different from the API of the private inner type.
//! The newtype pattern is a lightweight way to achieve encapsulation to hide implementation details

//! TYPE ALIASES
//! Rust provides the ability to declare a type alias to give an existing type another name.
fn main() {
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);
}
//! Kilometers is not a separate, new type. Values that have the type Kilometers will be treated the same as values of type i32
//! using this method, we don’t get the type checking benefits that we get from the newtype pattern discussed earlier.

//! The main use case for type synonyms is to reduce repetition.
//! WE CAN REPLACE THE LONG TYPE DEFINITION WITH AN ALIAS
fn main() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    //// let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }
    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --snip--
        Box::new(|| ())
    }
}
//! a use case of aliases with Result<T, E>
use std::fmt;
use std::io::Error;
pub trait Write {
    // rust has this type alias by default
    type Result<T> = std::result::Result<T, std::io::Error>;

    // fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    // fn flush(&mut self) -> Result<(), Error>;
    fn flush(&mut self) -> Result<()>;

    // fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    // fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

//! NEVER TYPE
//! Rust has a special type named ! that’s known in type theory lingo as the empty type because it has no values. We prefer to call it the never type because it stands in the place of the return type when a function will never return.
fn bar() -> ! {
    // --snip--
    panic!();
}
//! This code is read as “the function bar returns never.” Functions that return never are called diverging functions. We can’t create values of the type ! so bar can never possibly return.
//! But what use is a type you can never create values for? Recall the code from Listing 2-5, part of the number guessing game; we’ve reproduced a bit of it
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
//! guess is u32 type so how does rust allows continue to return from match and assign it to quess
//! the answer is that continue has a never type and rust knows that continue will never return anything and return the control to the top of the loop and guess will never get assigned
//! match arms must all return the same type. So, for example, the following code doesn’t work
let guess = match guess.trim().parse() {
    Ok(_) => 5,
    Err(_) => "hello",
};
//! panic macro also has never type
//! unwrap needs to return T variable if panic!() didn't have a never type this match expression would be invalid
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
//! loop{} also returns never type
//! but if we use break it will break out of the loop
print!("forever ");
loop {
    print!("and ever ");
}

//! DYNAMIC SIZED TYPES
//! Rust needs to know certain details about its types, such as how much space to allocate for a value of a particular type. This leaves one corner of its type system a little confusing at first: the concept of dynamically sized types. Sometimes referred to as DSTs or unsized types, these types let us write code using values whose size we can know only at runtime.
//! because str maybe of different sizes and is not like numbers that have defined fixed size
//! dynamic sized data have an extra bit of metadata that stores the size of the dynamic information. The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.
//! We can combine str with all kinds of pointers: for example, Box<str> or Rc<str>.
//! Every trait is a dynamically sized type we can refer to by using the name of the trait.
//! to use traits as trait objects, we must put them behind a pointer, such as &dyn Trait or Box<dyn Trait> (Rc<dyn Trait> would work too).
fn main() {
    // this will generate error because rust can't know the size of str at compile time
    let s1: str = "Hello there!";
    let s2: str = "How's it going?";
    // we can use &str instead now this variable is a reference to string in memory (heap, stack and etc) with additional meta data like size of the string
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";
}

//! To work with DSTs, Rust provides the Sized trait to determine whether or not a type’s size is known at compile time. This trait is automatically implemented for everything whose size is known at compile time. In addition, Rust implicitly adds a bound on Sized to every generic function.
fn generic<T>(t: T) {
    // --snip--
}
//!same as
fn generic<T: Sized>(t: T) {
    // --snip--
}
//! By default, generic functions will work only on types that have a known size at compile time. However, you can use the following special syntax to relax this restriction
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}

//! A trait bound on ?Sized means “T may or may not be Sized” and this notation overrides the default that generic types must have a known size at compile time. The ?Trait syntax with this meaning is only available for Sized, not any other traits.

//! Also note that we switched the type of the t parameter from T to &T. Because the type might not be Sized, we need to use it behind some kind of pointer. In this case, we’ve chosen a reference.