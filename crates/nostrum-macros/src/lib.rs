use proc_macro::TokenStream;

mod object;
mod use_obj;

/// Turns a impl struct to a trait object.
///`#[nostrum::object]`
#[proc_macro_attribute]
pub fn object(_attr: TokenStream, item: TokenStream) -> TokenStream {
    quote::quote! {}.into()
}

/// Use obj instead of real struct.
/// `#[nostrum::use_obj]`
#[proc_macro_attribute]
pub fn use_obj(attr: TokenStream, item: TokenStream) -> TokenStream {
    quote::quote! {}.into()
}
