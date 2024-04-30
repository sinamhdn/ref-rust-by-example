use hello_macro::HelloMacro;

struct Pancakes;

// instead of implementing trait for each type we can turn this into a macro and call it before eachtype using #derive
impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
