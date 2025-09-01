extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input as a struct
    let input = parse_macro_input!(item as ItemStruct);

    let struct_name = &input.ident;

    let expanded = quote! {
        #input

        impl crate::Component for #struct_name {
            pub fn type_name() -> &'static str {
                stringify!(#struct_name)
            }   
        }
    };

    TokenStream::from(expanded)
}