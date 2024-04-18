// Rust code uses snake case as the conventional style for function and variable names
// Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.
// In function signatures, you must declare the type of each parameter.
// Function bodies are made up of a series of statements optionally ending in an expression
// in rust it is crucial to know difference between expression and statement
// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value. Let’s look at some examples.
fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_parameter(5);
    print_labeled_measurement(5, 'h');

    let y = 6; // statement
    let x = (let y = 6); // ERROR because statements don't return value

    // a new scope created is an expression
    // Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. //
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

/* all function definitions are statements */
/* Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression */

// Expressions can be part of statements

fn another_function() {
    println!("Another function.");
}

fn another_function_with_parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1 // if we put semicolon here we get an error
    // because if we put semicolorn we turn this into a statement and in that case function returns () but we want the function to return an integer
}