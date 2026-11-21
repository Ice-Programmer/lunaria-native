mod repository;
mod service;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn service(args: TokenStream, item: TokenStream) -> TokenStream {
    service::expand(args, item).unwrap_or_else(|err| err.into_compile_error().into())
}

#[proc_macro_attribute]
pub fn repository(args: TokenStream, item: TokenStream) -> TokenStream {
    repository::expand(args, item).unwrap_or_else(|err| err.into_compile_error().into())
}
