#[derive(Clone)]
pub struct GValueHolder(GValue);

//This is stupid but it suposedly has to be that way
impl ToGValue for GValueHolder {
    fn to_gvalue(&self) -> GValue {
        self.0.clone()
    }
}

impl<T> From<T> for GValueHolder
where
    GValue: From<T>,
{
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl Debug for GValueHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.0.fmt(f);
    }
}
