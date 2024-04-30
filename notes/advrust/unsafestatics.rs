//! In this book, we’ve not yet talked about global variables, which Rust does support but can be problematic with Rust’s ownership rules. If two threads are accessing the same mutable global variable, it can cause a data race.
//! In Rust, global variables are called static variables.

//! Static variables can only store references with the 'static lifetime, which means the Rust compiler can figure out the lifetime and we aren’t required to annotate it explicitly. Accessing an immutable static variable is safe.

//! A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory. Using the value will always access the same data. Constants, on the other hand, are allowed to duplicate their data whenever they’re used. Another difference is that static variables can be mutable. Accessing and modifying mutable static variables is unsafe.

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!("name is: {}", HELLO_WORLD);
    add_to_count(3);

    // to mutate a static variable we need to use unsafe block
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

//! it’s preferable to use the concurrency techniques and thread-safe smart pointers so the compiler checks that data accessed from different threads is done safely.
