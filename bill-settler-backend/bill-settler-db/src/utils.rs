#[macro_export]
macro_rules! derive_label {
    ($strct:ident, $label:ident) => {
        impl crate::vertices::DbLabel for $strct {
            fn g_label() -> &'static str {
                stringify!($label)
            }
        }
    };
    ($label:ident) => {
        crate::derive_label!($label, $label);
    };
}

#[macro_export]
macro_rules! derive_vertex {
    ($vertex:ident) => {
        crate::derive_label!($vertex);
        impl crate::vertices::DbVertex for $vertex {
            fn id(&self) -> i64 {
                self.id
            }
        }
    };
}
#[macro_export]
macro_rules! unique_fields {
    ($traversal:expr, $strct:ident; $($x:expr),*) => {
        use crate::vertices::DbVertex;
        $(match $traversal
            .v(())
            .has_label($strct::g_label())
            .has($x)
            .count()
            .next()?
        {
            None | Some(0) => (),
            Some(_) => return Err(DbError::NotUnique(($x.0, $x.1.into()))),
        })*
    };
}
