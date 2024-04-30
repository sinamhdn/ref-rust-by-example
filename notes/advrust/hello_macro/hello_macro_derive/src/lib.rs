//! procedural macros must be defined at their own crate
//! we create this derive crate inside the crate that use this
//! we we still need to publish these crates seperately
//! and import them while using
extern crate proc_macro; // allows  use to manipulate rust code
use proc_macro::TokenStream;
use quote::quote; // turn syntax tree data structure to rust code
use syn; // take a stream of rust code and turn it into a syntax tree code structure

// this function is only going to be called at compile time
#[proc_macro_derive(HelloMacro)] // indicate this is a custom derive macro
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident; // extact the name of the type
    let gen = quote! { // output rust code
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into() // convert to TokenStream
}
