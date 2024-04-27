//! whenever  code goes out of scope rust runs the drop function inside Drop trait
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // if we want to drop a value before it going out of scope rust doesn't allow us to call drop manually
    // because if rust tries  to call drop at the end of the scope we may have double drop
    //// c.drop();

    // to drop early use std::mem::drop()
    // since this function is in prelude we can just use drop()
    drop(c);

    println!("CustomSmartPointer dropped before the end of main.");
}
