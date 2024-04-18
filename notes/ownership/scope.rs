// A scope is the range within a program for which an item is valid.
// The variable s refers to a string literal, where the value of the string is hardcoded into the text of our program. The variable is valid from the point at which it’s declared until the end of the current scope
// When s comes into scope, it is valid.
// It remains valid until it goes out of scope.
#![allow(unused)]
fn main() {         // s is not valid here, it’s not yet declared
    let s = "hello";// s is valid from this point forward
                    // do stuff with s
}                   // this scope is now over, and s is no longer valid