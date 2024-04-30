//! we mentioned that the compiler ensures references are always valid. Unsafe Rust has two new types called raw pointers that are similar to references. As with references, raw pointers can be immutable or mutable and are written as *const T and *mut T, respectively. The asterisk isn’t the dereference operator; it’s part of the type name. In the context of raw pointers, immutable means that the pointer can’t be directly assigned to after being dereferenced.
//!
//!
//! Different from references and smart pointers, raw pointers:
//! Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
//! Aren’t guaranteed to point to valid memory
//! Are allowed to be null
//! Don’t implement any automatic cleanup
//!
//! We can create raw pointers in safe code; we just can’t dereference raw pointers outside an unsafe block, as you’ll see in a bit
fn main() {
    let mut num = 5;

    // here because we convert a valid reference to raw pointer we will have  no risk
    // we can create a mut and unmut pointer to the same location but we can't create references like so
    // with immutable referenceswe may change the data and create a data race
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // we can also create pointer to a random location in memory at doing it like this is risky
    let address = 0x012345usize;
    let r = address as *const i32;

    // we can create unsafe pointers in unsafe code but we can't dereference them and use them in unsafe code
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // we can also create unsafe functions and methods but we need to becarefull because compile wont check for functions errors as strict as default
    // every code inside unsafe function can also be unsafe
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }

    // we can abstract unsafe code by wrapping them in a safe function you don't need to mark whole function with unsafe
    // split_at_mut is a standard function that uses unsafe code inside (this function splits the slice a the index)
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    use std::slice;

    // we don't need to mark this function ans unsafe because code inside it uses unsafe code in a safe way ans access valid memory locations
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        assert!(mid <= len);

        /// implementing the code with safe code results in an error
        // here rust thinks that we are borrowing from one slice twice when we return the tuple
        (&mut values[..mid], &mut values[mid..])

        // we can fix this problem by using unsafe code and pointers
        // we use as_mut_ptr to get access to raw pointer of the slice (slices are a pointer to some data and the length of the slice)
        let ptr = values.as_mut_ptr(); // (*mut i32) type 
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    // but this code most likely failt because it creates a long slice in an arbitrary locations
    let address = 0x01234usize;
    let r = address as *mut i32;
    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
}
