use proc_macro::TokenStream;

#[proc_macro]
pub fn function_like(_input: TokenStream) -> TokenStream {
    "5".parse().unwrap()
}