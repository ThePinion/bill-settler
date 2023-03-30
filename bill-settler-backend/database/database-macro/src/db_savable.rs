use proc_macro::TokenStream;
use quote::quote;

pub fn impl_db_savable(ast: syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let fields = match ast.data {
        syn::Data::Struct(data_struct) => data_struct.fields,
        _ => panic!("DbSavable only works with structs"),
    };

    let prop_pairs = fields
        .into_iter()
        .filter_map(|f| f.ident)
        .filter(|n| n != "id" && n != "source_id" && n != "target_id")
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
