//! rust has declarative macros with macro_rules! and three kinds of procedural macros:
//! ---- Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
//! ---- Attribute-like macros that define custom attributes usable on any item
//! ---- Function-like macros that look like function calls but operate on the tokens specified as their argument

//! Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming.
//! All of these macros expand to produce more code than the code you’ve written manually.
//! Metaprogramming is useful for reducing the amount of code you have to write and maintain, which is also one of the roles of functions. However, macros have some additional powers that functions don’t.

//! A function signature must declare the number and type of parameters the function has. Macros, on the other hand, can take a variable number of parameters: we can call println!("hello") with one argument or println!("hello {}", name) with two arguments. Also, macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a trait on a given type. A function can’t, because it gets called at runtime and a trait needs to be implemented at compile time.

//! The downside to implementing a macro instead of a function is that macro definitions are more complex than function definitions because you’re writing Rust code that writes Rust code. Due to this indirection, macro definitions are generally more difficult to read, understand, and maintain than function definitions.

//! Another important difference between macros and functions is that you must define macros or bring them into scope before you call them in a file, as opposed to functions you can define anywhere and call anywhere.

//! Macros like match expressions compare a value to patterns that are associated with particular code: in this situation, the value is the literal Rust source code passed to the macro; the patterns are compared with the structure of that source code; and the code associated with each pattern, when matched, replaces the code passed to the macro. This all happens during compilation.

//! We could also use the vec! macro to make a vector of two integers or a vector of five string slices. We wouldn’t be able to use a function to do the same because we wouldn’t know the number or type of values up front.
#![allow(unused)]
fn main() {
    let v: Vec<u32> = vec![1, 2, 3];
}
//! this is a slightly simplified definition of the vec! macro
//! We then start the macro definition with macro_rules! and the name of the macro we’re defining without the exclamation mark. The name, in this case vec, is followed by curly brackets denoting the body of the macro definition.
//! The structure in the vec! body is similar to the structure of a match expression. Here we have one arm with the pattern ( $( $x:expr ),* ), followed by => and the block of code associated with this pattern. If the pattern matches, the associated block of code will be emitted. Given that this is the only pattern in this macro, there is only one valid way to match; any other pattern will result in an error. More complex macros will have more than one arm.
//! macro patterns are matched against Rust code structure rather than values
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
//! The #[macro_export] annotation indicates that this macro should be made available whenever the crate in which the macro is defined is brought into scope. Without this annotation, the macro can’t be brought into scope.

//! procedural macro acts more like a function (and is a type of procedure). Procedural macros accept some code as an input, operate on that code, and produce some code as an output rather than matching against patterns and replacing the code with other code as declarative macros do. The three kinds of procedural macros are custom derive, attribute-like, and function-like, and all work in a similar fashion

//! When creating procedural macros, the definitions must reside in their own crate with a special crate type.

use proc_macro;
#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}

//! The function that defines a procedural macro takes a TokenStream as an input and produces a TokenStream as an output. The TokenStream type is defined by the proc_macro crate that is included with Rust and represents a sequence of tokens. This is the core of the macro: the source code that the macro is operating on makes up the input TokenStream, and the code the macro produces is the output TokenStream. The function also has an attribute attached to it that specifies which kind of procedural macro we’re creating. We can have multiple kinds of procedural macros in the same crate.

//! Let’s create a crate named hello_macro that defines a trait named HelloMacro with one associated function named hello_macro. Rather than making our users implement the HelloMacro trait for each of their types, we’ll provide a procedural macro so users can annotate their type with #[derive(HelloMacro)] to get a default implementation of the hello_macro function. The default implementation will print Hello, Macro! My name is TypeName! where TypeName is the name of the type on which this trait has been defined.

//! in short when a user adds our macro before a type it will implement the trait inside for that type
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
#[derive(HelloMacro)]
struct Pancakes;
fn main() {
    Pancakes::hello_macro();
}
//! first we create a new --lib crate "hello_macro" that defines a trait named "HelloMacro"
pub trait HelloMacro {
    fn hello_macro();
}
//! user normally can implement the trait like this
use hello_macro::HelloMacro;
struct Pancakes;
impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}
fn main() {
    Pancakes::hello_macro();
}
//! but with this user need to implement for every type tht needs this trait
//! also rust doesn't have reflective qualities so it can't access the name of type that implements the trait
//! next we create a new procedural macro crate inside out library crate (do this because two crates are tightly coupled)
//! we still need to import both crates in the user side
// $ cargo new hello_macro_derive --lib
//! if we change the trait code we also need to change the macro code
//! we could create macro crate outside the library crate and add it as a dependency but we want the user to be able to use the library even if they don't need to use the macro
//! add this to the Cargo.toml of the macro crate
// [lib]
// proc-macro = true
// [dependencies]
// syn = "1.0"
// quote = "1.0"
//! this is the code of macro crate
use proc_macro::TokenStream; // this is the compiler’s API that allows us to read and manipulate Rust code from our code
use quote::quote; // The quote crate turns syn data structures back into Rust code
use syn; // The syn crate parses Rust code from a string into a data structure that we can perform operations on
// this function will be called when a user of our library specifies #[derive(HelloMacro)] on a type
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    // the only different part of derive crates is here
    // our code goes here
    impl_hello_macro(&ast)
}
//! The parse function in syn takes a TokenStream and returns a DeriveInput struct representing the parsed Rust code.
// DeriveInput {
//     // --snip--
//     ident: Ident {
//         ident: "Pancakes",
//         span: #0 bytes(95..103)
//     },
//     data: Struct(
//         DataStruct {
//             struct_token: Struct,
//             fields: Unit,
//             semi_token: Some(
//                 Semi
//             )
//         }
//     )
// }
//! this data structure contains all the data about the struct or enum we use it for
//! ie ident shows that type name is "Pancakes"
//! with this macro we can add more functionality to the code our users write
//! we use unwrap to make macro panic on error because we want to always return Tokenstrem and not Result
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
//! this is the main logic of our code
//! with ident we get the name of  the type
//! quote! is the code we want to return
//! #name if filled with name of the type
//! stringify! is to convert expression to string
//! now if you run cargo buid it will build both crates
//! both crates will be regular dependencies if you publish them if not you can add the locally
// hello_macro = { path = "../hello_macro" }
// hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }

//! attribute like macros are ame as derive macros but instead of generating code for derive attribute they allow you create new attributes
//! derive only works for enum and structs but aattribute like macros work for functions as well
#[route(GET, "/")]
fn index() {}
//! here we annotate this function with route attribute
//! route attribute is defined as a procedural macro
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
//! attr is for contents of attribute ===> Get, /
//! item is for body of the item the attribute is attached to ===> fn index() {}
//! for this macro just like derive macro we create a new crate

//! function like macros are like declarative macros but can do more complex tasks and accept TokenStream meaning we can pass them a rust code and they return us a rust code
//! a exampel of function like macro is sql!() which takes in an sql command and checks if it is valid or not
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {}