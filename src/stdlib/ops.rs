pub trait Pow<Rhs = Self> {
    type Output;
    fn pow(self, rhs: Rhs) -> Self::Output;
}

pub trait TypeOf {
    fn r#typeof(self) -> String;
}
