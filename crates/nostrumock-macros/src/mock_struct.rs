use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};

use crate::{attr_syntax::LetMock, impl_syntax::ImplMock};

pub(crate) fn generate(attr: &LetMock, input: &ImplMock) -> TokenStream {
    let ident = format_ident!(
        "{}__{}",
        attr.pat_ident.ident,
        input
            .trait_
            .segments
            .last()
            .expect("not empty trait path")
            .ident
    );
    quote! {
        #[allow(non_camel_case_types)]
        #[allow(clippy::extra_unused_lifetimes)]
        struct #ident<'a> {}
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn empty() {
        let attr = parse_quote! {
            let my_mock = Cat
        };
        let input = parse_quote! {
            impl Something for Cat {}
        };
        let actual = generate(&attr, &input);
        let expected = quote! {
            #[allow(non_camel_case_types)]
            #[allow(clippy::extra_unused_lifetimes)]
            struct my_mock__Something<'a> {}
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }
}
