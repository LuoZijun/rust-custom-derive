extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;



#[proc_macro_derive(HelloWorld)]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    
    println!("{:?}", ast);
    
    // Build the output, possibly using quasi-quotation
    let name = &ast.ident;
    
    let expanded = quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, World! My name is {}", stringify!(#name));
            }
        }
    };
    
    // Hand the output tokens back to the compiler
    expanded.into()
}
