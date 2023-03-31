use proc_macro::TokenStream;
use quote::quote;

pub fn impl_enum_g_value(ast: syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let _ = match ast.data {
        syn::Data::Enum(_) => (),
        _ => panic!("EnumGValue only works with enum"),
    };

    let gen = quote! {
        impl TryFrom<gremlin_client::GValue> for #struct_name  {
            type Error = gremlin_client::GremlinError;

            fn try_from(value: gremlin_client::GValue) -> Result<Self, Self::Error> {
                match value {
                    gremlin_client::GValue::String(s) => Ok(serde_json::from_str(&s)?),
                    _ => Err(gremlin_client::GremlinError::WrongType(value)),
                }
            }
        }

        impl Into<gremlin_client::GValue> for #struct_name {
            fn into(self) -> gremlin_client::GValue {
                gremlin_client::GValue::String(serde_json::to_string(&self).unwrap())
            }
        }
    };

    gen.into()
}
