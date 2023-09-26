use proc_macro::TokenStream;

mod figa;

#[proc_macro_derive(Figa, attributes(figa))]
pub fn derive_figa(input: TokenStream) -> TokenStream {
    match figa::derive(syn::parse_macro_input!(input as syn::DeriveInput)) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
