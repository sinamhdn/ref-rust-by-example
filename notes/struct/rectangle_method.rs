#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// each struct can have multiple impl blocks if necessary
impl Rectangle {
    // &self is short for self:&Self
    // self means the type this method is implemented on
    // we must have exacly a first parameter name self
    // Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.
    // methods help with organizing the code also we don't need to pass instance everytime to function
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // method can have the same name as fields
    // this is usefull to create getter methods
    fn width(&self) -> bool {
        self.width > 0
    }

    // here we dont take ownership of the self and other and borrow them
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated functions don't use self as first parameter because they don't rely on struct type to work
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}