use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Greeting)]
pub fn fn_macro_derive(input: TokenStream) -> TokenStream {
    let st = syn::parse(input).unwrap(); // DeriveInput

    impl_macro_derive(&st)
}

fn impl_macro_derive(st: &syn::DeriveInput) -> TokenStream {
    let name = &st.ident; // Ident
    let gen = quote! {
        impl Greeting for #name {
            fn speak() {
                println!("Hello, Macro! Your identity is: {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}