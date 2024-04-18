fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    // even if the string changes later word will be still 5

    // A string slice is a reference to part of a String
    // starting_index is the first position in the slice and ending_index is one more than the last position in the slice
    // Note: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.
    let hello = &s[0..5];
    let world = &s[6..11];

    // both are same
    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();
    // both are the same
    let slice = &s[3..len];
    let slice = &s[3..];
    // both are the same
    let slice = &s[0..len];
    let slice = &s[..];

    // we can also have slices to the arrays
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.
// without slice we can return index to the end of first word before space
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
// the problem with this is that the usize return value does not depend on the string and if string goes away this will be an in valid value

// with this function if we try to use return value after string is cearld we get an error because s is an invalid reference
// Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference. Because clear needs to truncate the String, it needs to get a mutable reference. The println! after the call to clear uses the reference in word, so the immutable reference must still be active at that point. Rust disallows the mutable reference in clear and the immutable reference in word from existing at the same time, and compilation fails.
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// string literals are an immutable reference of str ---> &str
// we can use string slice when we want to pass anything to a function because we can pass slices directly and we can pass a slice of String to the function
// this flexibily comes from deref coercion
// this makes api more flexible
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}