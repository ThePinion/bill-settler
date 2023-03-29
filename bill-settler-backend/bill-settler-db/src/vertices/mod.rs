pub mod user;

pub trait DbVertex: TryFrom<gremlin_client::GValue> + TryFrom<gremlin_client::Map> {
    fn g_label() -> &'static str;
    fn id(&self) -> i64;
}
