fn main() {
    // rust variables are immutable by default means ONCE you have assigned a value to them you can't reassign
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // this code has error because we tried to change the value of a variable
    // cannot assign twice to immutable variable `x`

    // you can make a variable mutable by adding mut
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constants are always immutable
    // you must define type when defining a constant
    // The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing
    // you can shadow a variable by repeating he same variable name twice
    // now compiler will see the second variable until the scope ends or variable shadows
    // with shadowing we can reassign to a immutable variable
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    // we can even change the type of variable with shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    // we get error if we assign a different type to a mutable variable
    let mut spaces = "   ";
    spaces = spaces.len();
}