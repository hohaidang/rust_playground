// extern crate proc_macro;
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, DeriveInput};

// #[proc_macro_derive(PrintOnCreate)]
// pub fn derive_print_on_create(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = &input.ident;

//     let expanded = quote! {
//         impl #name {
//             pub fn new() -> Self {
//                 println!("Struct defined: {}", stringify!(#name));
//                 Self {
//                     // Assuming the struct has a Default implementation
//                     ..Default::default()
//                 }
//             }
//         }
//     };

//     TokenStream::from(expanded)
// }


extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PrintOnCreate)]
pub fn derive_print_on_create(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn new() -> Self {
                println!("Struct defined: {}", stringify!(#name));
                println!("****{}", stringify!(#name));
                Self {
                    ..Default::default()
                }
            }
        }
    };

    TokenStream::from(expanded)
}
