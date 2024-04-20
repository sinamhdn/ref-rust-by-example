pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    // we need to bring the code under test in the outer module into the scope of the inner module. We use a glob here so anything we define in the outer module is available to this tests module.

    #[test] // marks the function as test function
    fn add_2_w_2() {
        // running "cargo test" will print function name
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    // The assert! macro, is useful when you want to ensure that some condition in a test evaluates to true. We give the assert! macro an argument that evaluates to a Boolean. If the value is true, nothing happens and the test passes. If the value is false, the assert! macro calls panic! to cause the test to fail.
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // because can_hold() returns false we negate it's value so that test succeeds
        assert!(!smaller.can_hold(&larger));
    }

    // A common way to verify functionality is to test for equality between the result of the code under test and the value you expect the code to return. You could do this using the assert! macro and passing it an expression using the == operator. However, this is such a common test that the standard library provides a pair of macros—assert_eq! and assert_ne!—to perform this test more conveniently.
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // You can also add a custom message to be printed with the failure message as optional arguments to the assert!, assert_eq!, and assert_ne! macros. Any arguments specified after the required arguments are passed along to the format! macro
    // so you can pass a format string that contains {} placeholders and values to go in those placeholders.
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // here we use contains instead of equality because this name may change
        // we can add custom message to assert
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // The test passes if the code inside the function panics; the test fails if the code inside the function doesn’t panic.
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    // Tests that use should_panic can be imprecise. A should_panic test would pass even if the test panics for a different reason from the one we were expecting. To make should_panic tests more precise, we can add an optional expected parameter to the should_panic attribute.
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100_precise() {
        Guess::new(200);
    }
    // This test will pass because the value we put in the should_panic attribute’s expected parameter is a substring of the message that the Guess::new function panics with.

    // WE CAN ALSO HANDLE EXCEPTIONS WITH TESTS INSTEAD OF PANIC
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    // Writing tests so they return a Result<T, E> enables you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.
    // You can’t use the #[should_panic] annotation on tests that use Result<T, E>. To assert that an operation returns an Err variant, don’t use the question mark operator on the Result<T, E> value. Instead, use assert!(value.is_err()).

    // by default test liberary will consume print statements and only displays pass or fail to the output
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    // you can ignore time consuming tests like this
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

    // in rust you can also test private functions
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
