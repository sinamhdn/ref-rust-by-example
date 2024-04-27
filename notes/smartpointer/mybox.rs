//! Note: there’s one big difference between the MyBox<T> type we’re about to build and the real Box<T>: our version will not store its data on the heap.

// Box is defined this way a tuple struct with one generic parameter
struct MyBox<T>(T);

// this implementation make * act just like references
impl<T> Deref for MyBox<T> {
    type Target = T; // this is an association type

    fn deref(&self) -> &Self::Target {
        // we return reference here because we don't want the deref operator to take ownership of inner value
        &self.0 // return first item in a tuple
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    // in rust unlike c we can store references in regular variable and don't need a raw reference
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let t = MyBox::new(x);

    // to access reference value we need to dereference variable
    // if we don't use * we get error
    // Deref works same on references(&) and Box
    // The main difference between references(&) and Box is that in Box we set z to be an instance of a Box<T> pointing to a copied value of x rather than a reference pointing to the value of x
    // "*" operator for MyBox wont work because we haven't implement it"
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    // behind the scene --> *(y.deref()) <-- rust will call the function that returns a reference than calls normal dereference operator on the reference
    assert_eq!(5, *t);

    // we want to pass a reference to MyBox<String> to the hello function that accepts a string slice reference
    // with coercion we can write this code
    let m = MyBox::new(String::from("Rust"));
    // coersion will first look at deref function in MyBox than in String so it automatically converts MyBox to &String than to &str
    hello(&m);
    // without coercion we need to write code like this
    // dereference MyBox than get a reference to String than convert it to slice
    hello(&(*m)[..]);
}
