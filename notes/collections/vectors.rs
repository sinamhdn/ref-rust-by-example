fn main() {
    // vector is dynamic unlike arrays and is stored on the heap instead of stack
    let mut v: Vec<i32> = Vec::new();

    // rust can infer the type so we dont need to define generic type
    // i32 is the default integer value
    let v2 = vec![1, 2, 3];

    // add to vector (to make changes make the variable mutable)
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // two methods of accessing vector variables
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // if we try to access out of bound variable first one will panic and second will return None
    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    // this code panics because cant have mutable and immutable references in the same scope
    // because there is a possibility that adding new value require our program to copy whole vector to a new location we cant add to the vector after borrowing it to get its first value
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("The first element is: {first}");

    // use a for loop to get immutable references to each element in a vector of i32 values and print them
    let v3 = vec![100, 32, 57];
    for i in &v3 {
        println!("{i}");
    }

    // We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements
    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
    }

    // to store values of different types in vector we can use enums
    // with enums we need to define exactly what values of whats types gets stored inside enum if we want to add undefined value type to the vector we can use Traits
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
