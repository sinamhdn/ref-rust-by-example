// lifetimes means how long a variable is valid
// mostly rust determines lifetimes based on the scopes and predefined rules
// we only need to implicitly define lifetimes when lifetimes of variables are some what related
// Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

// this code wont work because we reference x in r but when we use r x is out of scope and invalid
// if we try to use a variable whitout giving it value compiler will throw error
// Rust doesnt allow null values
// if borrow checker findout that borrowee has shorter lifetime than borrower it throws error
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}

// because we dont want to take ownership of the parameter we use straing slices instead of strings
// this code wont compile because we will return either x or y and we need a generic lifetime variable for return type
// here we rust needs to know the relationship between parameters and return type
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// this means returned reference is valid as long as both parameters are valid
// this means that lifetime of the returned reference is as long as the smallest of two parameters
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// print statement will print outer string because it is valid till after the print
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// this will panic because result variable has the lifetime of string2 because it is smaller and goes out of scope
// because computer cant find lifetime with looking at the code shape we need to clarify it we lifetime generics
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

// becaue this function always returns x we dont need to efine lifetime for y
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// this code errors because returned values lifetime is different from the actual value that gets returned
// this code creates a dangling reference and rust wont allow it
// return an owned type instead of reference here
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

// if we use references in structs we need to define lifetime variables
// this means struct reference can not outlive the referenced it have
struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// This code compile without lifetime anottation for historical reasons
// In early versions (pre-1.0) of Rust, this code wouldn’t have compiled because every reference needed an explicit lifetime.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// At that time, the function signature would have been written like this:
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// common lifetime pattrens are coded inside compiler code so that programming in rust could be eaisier
// as time goes more lifetime elision rules are added to the compiler

// Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.
// In method signatures inside the impl block, references might be tied to the lifetime of references in the struct’s fields, or they might be independent.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
// The lifetime parameter declaration after impl and its use after the type name are required, but we’re not required to annotate the lifetime of the reference to self because of the first elision rule.

// There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes. Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// because string literals are stored inside programs code their lifetime is always available (static)
// static literals will never get removed
let s: &'static str = "I have a static lifetime.";

// pack generics, lifetimes and trait bounds together
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}