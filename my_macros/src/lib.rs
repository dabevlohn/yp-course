use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// Наш процедурный макрос: принимает строку и печатает её в коде
#[proc_macro]
pub fn say_hello(input: TokenStream) -> TokenStream {
    let msg = parse_macro_input!(input as syn::LitStr); // ожидаем строковый литерал
    let expanded = quote! {
        println!("{}", #msg);
    };
    expanded.into()
}
