//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// if we run cargo test rust will run the code in examples section as tests
// if we change the code but don't change the docs running test will fail and inform us to fix the code or doc

//! <-- are comments for file or modules these comments are inside rather than the code following these comments for example we use these comments to explain purpose of the crate inside crate root (i.a. lib.rs)
