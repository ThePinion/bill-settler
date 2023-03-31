use gremlin_client::GValue;

pub type PropPair = (String, GValue);

pub trait IntoPropPair {
    fn into_pair(self) -> PropPair;
}

impl<T> IntoPropPair for (&'static str, T)
where
    T: Into<GValue>,
{
    fn into_pair(self) -> PropPair {
        (self.0.into(), self.1.into())
    }
}
