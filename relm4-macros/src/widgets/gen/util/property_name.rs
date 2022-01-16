use proc_macro2::{Span as Span2, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Generics, Ident};

use crate::widgets::gen::PropertyName;

impl PropertyName {
    pub fn assign_fn_stream(&self, p_generics: &Option<Generics>, w_name: &Ident) -> TokenStream2 {
        let mut tokens = match self {
            PropertyName::Ident(ident) => {
                quote! { #w_name.#ident }
            }
            PropertyName::Path(path) => quote! { #path },
        };

        if let Some(generics) = p_generics {
            tokens.extend(quote! { :: #generics });
        }

        tokens
    }

    pub fn assign_args_stream(&self, w_name: &Ident) -> Option<TokenStream2> {
        match self {
            PropertyName::Ident(_) => None,
            PropertyName::Path(_) => Some(quote_spanned! { w_name.span() => & #w_name, }),
        }
    }

    pub fn self_assign_fn_stream(
        &self,
        p_generics: &Option<Generics>,
        w_name: &Ident,
        widgets_as_self: bool,
    ) -> TokenStream2 {
        let self_token = if widgets_as_self {
            quote! { widgets }
        } else {
            quote! { self }
        };

        let mut tokens = match self {
            PropertyName::Ident(ident) => {
                quote! { #self_token.#w_name.#ident }
            }
            PropertyName::Path(path) => quote! { #path },
        };

        if let Some(generics) = p_generics {
            tokens.extend(quote! { :: #generics });
        }

        tokens
    }

    pub fn self_assign_args_stream(
        &self,
        w_name: &Ident,
        widgets_as_self: bool,
    ) -> Option<TokenStream2> {
        let self_token = if widgets_as_self {
            quote! { widgets }
        } else {
            quote! { self }
        };

        match self {
            PropertyName::Ident(_) => None,
            PropertyName::Path(_) => {
                Some(quote_spanned! { w_name.span() =>  & #self_token.#w_name, })
            }
        }
    }
}

impl Spanned for PropertyName {
    fn span(&self) -> Span2 {
        match self {
            PropertyName::Ident(ident) => ident.span(),
            PropertyName::Path(path) => path.span(),
        }
    }
}
