use proc_macro::TokenStream;
use quote::quote;

pub fn impl_db_vertex(ast: syn::DeriveInput) -> TokenStream {
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
