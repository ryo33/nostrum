mod attr_syntax;
mod closure_expr;
mod closure_type;
mod impl_syntax;
mod impl_trait;
mod init_mock;
mod mock_struct;

use attr_syntax::LetMock;
use impl_syntax::ImplMock;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn mock(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as LetMock);
    let input = syn::parse_macro_input!(item as ImplMock);
    quote::quote! {}.into()
}
