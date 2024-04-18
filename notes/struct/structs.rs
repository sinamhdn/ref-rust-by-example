// define struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// we can create a function that returns an instance of the struct and initialize values  with it
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// we can use this shorthand to write "email" instead of "email:email"
fn build_user_alt(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// tuple structs are good to make each tuple a different type than other tuple
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit types structs are same as "()" and has no field
struct AlwaysEqual;

// this wont compile because we don't use lifetime on struct definition
struct User2 {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    // create an instance of struct (order doesn't matter)
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // instance must be mutable so that we can change a value in it
    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user2.email = String::from("anotheremail@example.com");

    // we can create a new instance based on another and copy some values from it
    // we can do this in two method
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // here we can not longer use user1 because we moved it's username string to this instance
    // if we copied active and then sign_in_count and then used new values for username nad email we could still use user1 because these values implemnt copy trait
    let user3_alt = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // each of these variables have different type
    // you can use all of tuple features with these
    // but you can pass Color tuple in function that accepts Point
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // we use unit type structs when we want to implemnt a trait on a type but we have no data
    let subject = AlwaysEqual;

    // this wont compile because we don't use lifetime on struct definition
    let user4 = User2 {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}