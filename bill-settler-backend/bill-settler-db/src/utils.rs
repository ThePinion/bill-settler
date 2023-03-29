// #[macro_export]
// macro_rules! derive_entity {
//     ($strct:ident) => {
//         use crate::vertices::DbVertex;
//         impl DbVertex for $strct {
//             fn g_label() -> &'static str {
//                 stringify!($strct)
//             }

//             fn id(&self) -> i64 {
//                 self.id
//             }
//         }
//     };
// }
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
