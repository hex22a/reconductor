use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Newtype)]
pub fn newtype_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_newtype(&ast)
}

fn impl_newtype(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated = quote! {
        impl #name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl From<String> for #name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl AsRef<str> for #name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    };
    generated.into()
}
