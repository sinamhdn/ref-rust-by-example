#![allow(unused)]
fn main() {
    // s is valid from this point forward
    let s = String::from("hello"); // String data type is a complex data type that gets stored on the heap and is mutable
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
    // this scope is now over, and s is no longer valid

    // when we do this we first bind 5 to x
    // then we copy the value of x and bind it to y
    // because these values have know sive they automatically get stored on the stack
    // these values don't need dropping because popping off the stack automatically drops them
    let x = 5;
    let y = x;

    // but this is different for String
    // string is a pointer to the heap with meta data that gets stored on the stack
    // actual string content is stored on the heap
    // here we just copy string pointer and not the actual data on the heap
    // so both variables point to same value
    // if rust copied every heap data every time it will be very time consuming
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1); // this code panics because rust moves the ownership of s1 to s2

    // we can create a deep copy using clone
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);
}
