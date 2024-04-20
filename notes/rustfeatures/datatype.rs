#![allow(unused)]
fn main() {
    // we have two data types --> scalar and compounds
    // Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it. In cases when many types are possible, such as when we converted a String to a numeric type using parse , we must add a type annotation
    // Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters

    /* all the integer types */
    // signed integer range --> -(2^n - 1) to 2^(n - 1) - 1
    // signed integer range --> 0 to 2^n - 1
    // isize and usize are 32 or 64 bit depending on the pc
    // Length	    Signed	Unsigned
    // 8-bit	    i8	    u8
    // 16-bit	    i16	    u16
    // 32-bit	    i32	    u32
    // 64-bit	    i64	    u64
    // 128-bit	    i128	u128
    // arch	        isize	usize

    // number literals that can be multiple numeric types allow a type suffix, such as 57u8, to designate the type
    // number literals that can be multiple numeric types allow a type suffix, such as 57u8, to designate the type
    // Number literals      Example
    // Decimal	            98_222
    // Hex	                0xff
    // Octal	            0o77
    // Binary	            0b1111_0000
    // Byte (u8 only)	    b'A'

    // Rust’s defaults are generally good places to start: integer types default to i32. The primary situation in which you’d use isize or usize is when indexing some sort of collection.

    // in case of string overflow compiler panics in debug mode or does two's compliment wrapping in release mode ( i.e. replaces 256 with 0)
    // to handle these explicitly use these methods
    // --- Wrap in all modes with the wrapping_* methods, such as wrapping_add.
    // --- Return the None value if there is overflow with the checked_* methods.
    // --- Return the value and a boolean indicating whether there was overflow with the overflowing_* methods.
    // --- Saturate at the value’s minimum or maximum values with the saturating_* methods.
    let guess: u32 = "42".parse().expect("Not a number!");

    // Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.
    // The f32 type is a single-precision float, and f64 has double precision.
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // rust supports these operations
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;

    // boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // character
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; //destructuring
    println!("The value of y is: {y}");
    // access each element of tuple like this
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    // The tuple without any values has a special name, unit. This value and its corresponding type are both written () and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.

    // Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.
    // arrays are stored on the heap
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // this array will have 5 of 3
    // access array elements like this
    let first = a[0];
    let second = a[1];
    // you can't access a value past the end of array program will exit immediately in this case
}