pub trait Resolve {
    type Output;
    fn resolve(&self) -> Self::Output;
}
