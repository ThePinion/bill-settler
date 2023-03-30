use crate::prop::PropPair;

pub trait DbSavable {
    fn g_props(&self) -> Vec<PropPair>;
}

pub trait DbLabel {
    fn g_label() -> &'static str;
}
