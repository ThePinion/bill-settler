use db_edge::impl_db_edge;
use db_label::impl_db_label;
use db_savable::impl_db_savable;
use db_vertex::impl_db_vertex;
use proc_macro::TokenStream;

mod db_edge;
mod db_label;
mod db_savable;
mod db_vertex;

#[proc_macro_derive(DbSavable)]
pub fn db_savable_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_db_savable(ast)
}

#[proc_macro_derive(DbLabel, attributes(DbLabel))]
pub fn db_label_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_db_label(ast)
}

#[proc_macro_derive(DbVertex)]
pub fn db_vertex_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_db_vertex(ast)
}

#[proc_macro_derive(DbEdge)]
pub fn db_edge_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_db_edge(ast)
}
