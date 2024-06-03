use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ItemFn};

#[proc_macro_attribute]
pub fn maybe_async(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);

    let quote = if cfg!(feature = "async") {
        quote!(async #item)
    } else {
        quote!(#item)
    };

    quote.into()
}

#[proc_macro]
pub fn maybe_await(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Expr);

    let quote = if cfg!(feature = "async") {
        quote!(#item.await)
    } else {
        quote!(#item)
    };

    quote.into()
}
