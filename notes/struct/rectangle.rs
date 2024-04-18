#[derive(Debug)] // now we can print struct in Debug mode
// In addition to the Debug trait, Rust has provided a number of traits for us to use with the derive attribute that can add useful behavior to our custom types.
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect1)
    );

    // if we try to print our struct directly we get an error
    // because println! macro with {} only works on types that implement std::fmt::Display trait
    // in primitive types it is clear what to Display but in structs we need to defined how {} display struct fields
    println!("rect1 is {}", rect1);
    // with this we can print struct data in debug mode and see it's values
    println!("rect1 is {:?}", rect1);
    // with this new syntax we print structs in a better format
    println!("rect1 is {:#?}", rect1);

    // we can also use !dbg macro instead of print
    // this macro trakes ownership unlike print! which takes reference prints the file and dbg! line and  returns ownership and resultant value of expression
    // Note: Calling the dbg! macro prints to the standard error console stream (stderr), as opposed to println!, which prints to the standard output console stream (stdout).
    dbg!(&rect1);

    // we can use dbg! on field init because it returns the expression results ownership
    // because dbg take ownership by default we pass a reference of the struct to avoid passing ownership
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
}

// these parameters are related to eachother so if will be better if we bundle them together using structs or tuples
fn area(width: u32, height: u32) -> u32 {
    width * height
}
// refactor code using tuples
// it is better but now parameters are not clear as we have to remember which one is height and which is width
fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// refactor code using structs
// we borrow the struct so tht main function can use struct after we borrowing its value
// here when we access width and height we dont take ownership because we borrowed the struct
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}