use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

pub fn impl_db_edge(ast: syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let fields = match ast.data {
        syn::Data::Struct(data_struct) => data_struct.fields,
        _ => panic!("TryFromEdgeMap only works with structs"),
    };

    let fields = fields
        .into_iter()
        .filter_map(|f| match f.ident {
            Some(n) => Some((n, f.ty)),
            None => None,
        })
        .filter(|f| f.0 != "source_id" && f.0 != "target_id")
        .collect::<Vec<_>>();

    let its = fields
        .iter()
        .map(|f| {
            let nm = &f.0;
            let tp = &f.1;
            quote! {#nm: #tp}
        })
        .collect::<Vec<_>>();

    let prps = fields
        .iter()
        .map(|f| {
            let nm = &f.0;
            quote! {#nm: inter.#nm}
        })
        .collect::<Vec<_>>();

    let intermediate = Ident::new(
        &format!("__Macro{}Intermediate", struct_name),
        Span::call_site(),
    );

    let gen = quote! {
        #[derive(gremlin_client::derive::FromGMap)]
        struct #intermediate{
            #(#its),*
        }
        impl TryFrom<database::edge::DbEdgeMap> for #struct_name {
            type Error = gremlin_client::GremlinError;

            fn try_from(value: database::edge::DbEdgeMap) -> Result<Self, Self::Error> {
                let inter: #intermediate = value.properties.try_into()?;
                Ok(#struct_name {
                    source_id: value.source_id,
                    target_id: value.target_id,
                    #(#prps),*
                })
            }
        }

        impl database::edge::DbSavableE for # struct_name {}

        impl database::edge::DbEdge for # struct_name {
            fn source_id(&self) -> i64{
                self.source_id
            }
            fn target_id(&self) -> i64{
                self.target_id
            }
        }
    };

    gen.into()
}
