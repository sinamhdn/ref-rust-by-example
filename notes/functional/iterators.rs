// In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.

// you can create an iterator on collections by using iter() method on them
// in languages that iterator is not available we iterate over data by incrementing a counter variable from 0 to the lenght of the collection
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // for loop implicitly creates iterator and use it
    // but here we create iterator explicitely
    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1_iter = v1.iter();
    // each call to next will return one item from the iterator
    // we need to mske the v1_iter mutable becssuse next changes the interal state
    // next() consues the iterator
    // for loop automatically take ownership and makes iterator mutable behind the scene
    // next() returns immutable references
    // iter() returns an iterator over immutable reference
    // we can use into_iter to ake ownership
    // we can use iter_mut to take mutable references
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

// all iterators must implement Iterator Trait
pub trait Iterator {
    type Item; // this item type will be returned from the iterator in next
    fn next(&mut self) -> Option<Self::Item>; // structs that implement this trait only require to implement this function
                                              // methods with default implementations elided
}

// some default implemented methods inside Iterator trait calls next() that is why we need to implement next()
// Methods that call next are called consuming adaptors, because calling them uses up the iterator.
// sum() here uses iterator because of that we can't call sum() after it is consumed
#[cfg(test)]
mod tests {
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }
}

// Iterator adaptors are methods defined on the Iterator trait that don’t consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator.
// after using map() we get a new iterator
// to consume that iterator we need to use next() or similar functions
// we use collect() to consume iterator and convert it to collection
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| x + 1);
}

// You can chain multiple calls to iterator adaptors to perform complex actions in a readable way. But because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

// Many iterator adapters take closures as arguments, and commonly the closures we’ll specify as arguments to iterator adapters will be closures that capture their environment.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoes_in_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
