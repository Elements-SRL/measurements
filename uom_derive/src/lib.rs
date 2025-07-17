use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use darling::FromDeriveInput;

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(uom), forward_attrs(allow, doc, cfg))]
struct Opts {
    label: Option<syn::Path>,
}

#[proc_macro_derive(Uom, attributes(uom))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;
    let uom = match opts.label {
        Some(path) => quote! {
            fn uom() -> String {
                format!(stringify!(#path))
            }
        },
        None => quote! {
            fn uom() -> String {
                format!(stringify!(#ident))
            }
        },
    };
    let output = quote! {
        impl Uom for #ident {
            #uom
        }
    };
    output.into()
}