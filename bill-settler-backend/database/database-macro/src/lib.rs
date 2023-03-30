use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Lit, MetaNameValue};

#[proc_macro_derive(DbSavable)]
pub fn db_savable_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_db_savable(ast)
}

fn impl_db_savable(ast: syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let fields = match ast.data {
        syn::Data::Struct(data_struct) => data_struct.fields,
        _ => panic!("DbSavable only works with structs"),
    };

    let prop_pairs = fields
        .into_iter()
        .filter_map(|f| f.ident)
        .filter(|n| n != "id")
        .map(|field_name| {
            let label = field_name.to_string();
            quote! {
                database::prop::IntoPropPair::into_pair((#label, self.#field_name.clone()))
            }
        })
        .collect::<Vec<_>>();

    let gen = quote! {
        impl database::entity::DbSavable for #struct_name {
            fn g_props(&self) -> Vec<database::prop::PropPair> {
                vec![
                    #(#prop_pairs),*
                ]
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(DbLabel, attributes(DbLabel))]
pub fn db_label_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_db_label(ast)
}

fn impl_db_label(ast: syn::DeriveInput) -> TokenStream {
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

#[proc_macro_derive(DbVertex)]
pub fn db_vertex_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_db_vertex(ast)
}

fn impl_db_vertex(ast: syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let _ = match ast.data {
        syn::Data::Struct(data_struct) => data_struct.fields,
        _ => panic!("DbVertex only works with structs"),
    };

    let gen = quote! {
        impl database::vertex::DbVertex for #struct_name {
            fn id(&self) -> i64 {
                self.id
            }
        }
    };

    gen.into()
}
