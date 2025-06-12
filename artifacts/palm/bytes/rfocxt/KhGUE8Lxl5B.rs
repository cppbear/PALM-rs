use crate::Buf;
#[derive(Debug)]
pub struct IntoIter<T> {
    inner: T,
}
impl<T> IntoIter<T> {
    pub fn new(inner: T) -> IntoIter<T> {
        IntoIter { inner }
    }
    pub fn into_inner(self) -> T {}
    pub fn get_ref(&self) -> &T {}
    pub fn get_mut(&mut self) -> &mut T {}
}
