fn main() {
    // if else tatement
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // IN RUST CONDITION MUST ALWAYS BE BOOL
    // this code results in error
    let number = 3;
    if number {
        println!("number was three");
    }

    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // we can assign variable values with if else
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // we can't use if else with different data types because we cant't assign different data types to a variable
    let condition = true;
    let number = if condition { 5 } else { "six" };
    println!("The value of number is: {number}");

    // you have to stop loop explicitly
    // you can stop with break; or continue; or keyboard interrupt
    loop {
        println!("again!");
    }

    // you can return value out of loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // you can use loop labels to specify which loop you want to break or continue
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // loop based on condition
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    // you can loop in a collection with while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    // or you could use for to iterate a collection
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    // you can use for loop like this to reap the loop a certain time
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}