use proc_macro::TokenStream;
use quote::quote;

/// Turns a impl struct to a trait object.
///`#[nostrum::object]`
#[proc_macro_attribute]
pub fn nostrum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::ItemImpl);
    process(item).into()
}

fn process(item: syn::ItemImpl) -> proc_macro2::TokenStream {
    let trait_ = generate_trait(&item);
    let impl_ = generate_impl(&item);
    quote! {
        #trait_
        #impl_
    }
}

fn generate_trait(item: &syn::ItemImpl) -> proc_macro2::TokenStream {
    quote! {}
}

fn generate_impl(item: &syn::ItemImpl) -> proc_macro2::TokenStream {
    quote! {}
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_trait() {
        let input = parse_quote! {
            impl Cat {
                fn new(name: String) -> Cat {
                    Cat { name }
                }
                fn meow(&self) -> String {
                    format!("{}: meow", self.name)
                }
                fn name(&self) -> &str {
                    &self.name
                }
                fn change_name(&mut self, name: String) {
                    self.name = name;
                }
            }
        };
        let actual = process(input);
        let expected = quote! {
            trait CatObj {
                fn new(name: String) -> Cat;
                fn meow(&self) -> String;
                fn name(&self) -> &str;
                fn change_name(&mut self, name: String);
            }
        };
    }
}
