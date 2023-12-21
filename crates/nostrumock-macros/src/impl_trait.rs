use proc_macro2::TokenStream;
use quote::quote;

use crate::{attr_syntax::LetMock, impl_syntax::ImplMock};

pub(crate) fn generate(attr: &LetMock, mock: &ImplMock) -> TokenStream {
    let trait_ = &mock.trait_;
    let struct_name = mock.struct_name(attr);
    let lifetime = quote!('__narrative_state);
    let generics = mock.methods().map(|method| {
        let method_ident = &method.sig.ident;
        let closure_type = crate::closure_type::generate(mock.target(), method);
        quote! {
            #method_ident: #closure_type,
        }
    });
    let struct_generics = mock.methods().map(|method| {
        let method_ident = &method.sig.ident;
        quote! {
            #method_ident
        }
    });
    let methods = mock.methods().map(|method| {
        let method_ident = &method.sig.ident;
        quote! {
            fn #method_ident(&self) -> String {
                self.#method_ident.lock().unwrap()(self.__narrative_state)
            }
        }
    });
    quote! {
        impl <
            #lifetime,
            #(#generics)*
        > #trait_ for #struct_name<#lifetime #(,#struct_generics)*> {
            #(#methods)*
        }
    }
}

#[cfg(test)]
mod tests {
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
            impl <
                '__narrative_state,
            > Something for my_mock__Something<'__narrative_state> {
            }
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn with_method() {
        let attr = parse_quote! {
            let my_mock = Cat
        };
        let input = parse_quote! {
            impl Something for Cat {
                fn meow(&self) -> String {
                    "meow".to_string()
                }
            }
        };
        let actual = generate(&attr, &input);
        let expected = quote! {
            impl <
                '__narrative_state,
                meow: FnMut(&Cat) -> String,
            > Something for my_mock__Something<'__narrative_state, meow> {
                fn meow(&self) -> String {
                    self.meow.lock().unwrap()(self.__narrative_state)
                }
            }
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }
}
