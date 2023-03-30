use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Lit, MetaNameValue};

pub fn impl_db_label(ast: syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let _ = match ast.data {
        syn::Data::Struct(_) => (),
        _ => panic!("DbLabel only works with structs"),
    };

    let label_name = get_label(&ast, struct_name);

    let gen = quote! {
        impl database::entity::DbLabel for #struct_name {
            fn g_label() -> &'static str {
                #label_name
            }
        }
    };

    gen.into()
}

fn get_label(ast: &syn::DeriveInput, struct_name: &Ident) -> quote::__private::TokenStream {
    let label_name = match ast.attrs.iter().find(|attr| attr.path.is_ident("DbLabel")) {
        Some(attr) => {
            let meta = attr.parse_meta().unwrap();
            match meta {
                syn::Meta::NameValue(MetaNameValue {
                    lit: Lit::Str(str), ..
                }) => quote! {
                    #str
                },
                _ => panic!("DbLabelName attribute must be a string literal"),
            }
        }
        None => {
            quote! {
                stringify!(#struct_name)
            }
        }
    };
    label_name
}
