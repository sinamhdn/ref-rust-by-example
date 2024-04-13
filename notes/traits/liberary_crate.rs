pub trait Summary {
    fn summarize(&self) -> String;
}

// default implementation
// default implementations can call other not implemented methods on the trait
pub trait SummaryWithDefaultImplementation {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!(String::from("(Read more...)"), self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// use default implementation
impl SummaryWithDefaultImplementation for NewsArticle {}

// we only need to implement not implemented method to use summerize method
impl SummaryWithDefaultImplementation for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// this function accepts any type that implements Summery trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify(item: &impl Summary, item1: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound
// this syntax is better when you have multiple variables
pub fn notify<T: Summary>(item: &T, item1: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Clearer Trait Bounds with where Clauses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}

// return types that implement Summary trait
// we can use this feature to return types that implement Iterator trait to create iterators and closures
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// you can only use impl Trait if you’re returning a single type
// this code wont work because it may return either a newsArticle or a Tweet
// Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions around how the impl Trait syntax is implemented in the compiler
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}

// implement one method for all types and the other for a specific type that implement certain traits
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// we can also implement traits for any type that implement another trait (Blanket Implementation)
// we can use ToString trait functions on any type that implements Display trait
impl<T: Display> ToString for T {
    // --snip--
}