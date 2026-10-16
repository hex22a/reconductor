use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

#[proc_macro_derive(Newtype)]
pub fn newtype_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    impl_newtype(&ast)
}

fn impl_newtype(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let inner_ty = match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Unnamed(f) if f.unnamed.len() == 1 => &f.unnamed[0].ty,
            _ => {
                return syn::Error::new_spanned(
                    name,
                    "must be a tuple struct with exactly one field",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(name, "must be a struct")
                .to_compile_error()
                .into();
        }
    };
    quote! {
        impl #name {
            pub fn new(value: impl Into<#inner_ty>) -> Self {
                Self(value.into())
            }

            pub fn into_inner(self) -> #inner_ty {
                self.0
            }
        }

        impl From<#inner_ty> for #name {
            fn from(value: #inner_ty) -> Self {
                Self(value)
            }
        }

        impl AsRef<#inner_ty> for #name {
            fn as_ref(&self) -> &#inner_ty {
                &self.0
            }
        }
    }
    .into()
}
