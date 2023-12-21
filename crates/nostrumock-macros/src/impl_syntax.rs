use syn::{
    parse::{Parse, ParseStream, Parser},
    spanned::Spanned,
    Token,
};

pub(crate) struct ImplMock {
    pub trait_: syn::Path,
    pub for_token: Token![for],
    /// The Self type of the impl.
    pub item_impl: syn::ItemImpl,
}

impl Parse for ImplMock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item_impl: syn::ItemImpl = input.parse()?;
        let Some((_, trait_, for_token)) = item_impl.trait_.clone() else {
            return Err(syn::Error::new(item_impl.self_ty.span(), "expected trait"));
        };
        Ok(Self {
            trait_,
            for_token,
            item_impl,
        })
    }
}

impl ImplMock {
    pub(crate) fn trait_name(&self) -> &syn::Ident {
        &self
            .trait_
            .segments
            .last()
            .expect("not empty trait path")
            .ident
    }
    pub(crate) fn methods(&self) -> impl Iterator<Item = &syn::ImplItemFn> {
        self.item_impl.items.iter().filter_map(|item| match item {
            syn::ImplItem::Fn(method) => Some(method),
            _ => None,
        })
    }
    pub(crate) fn target(&self) -> &syn::Type {
        self.item_impl.self_ty.as_ref()
    }
}
