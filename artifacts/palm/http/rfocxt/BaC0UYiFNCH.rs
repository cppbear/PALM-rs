use super::{Entry, HdrName, HeaderMap, HeaderName, InvalidHeaderName, MaxSizeReached};
pub trait Sealed {
    fn try_entry<T>(self, map: &mut HeaderMap<T>) -> Result<Entry<'_, T>, TryEntryError>;
    fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)>;
    fn as_str(&self) -> &str;
}
#[derive(Clone)]
pub struct HeaderMap<T = HeaderValue> {
    mask: Size,
    indices: Box<[Pos]>,
    entries: Vec<Bucket<T>>,
    extra_values: Vec<ExtraValue<T>>,
    danger: Danger,
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct HeaderName {
    inner: Repr<Custom>,
}
impl Sealed for HeaderName {
    #[inline]
    fn try_entry<T>(
        self,
        map: &mut HeaderMap<T>,
    ) -> Result<Entry<'_, T>, TryEntryError> {}
    #[inline]
    fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)> {
        map.find(self)
    }
    fn as_str(&self) -> &str {}
}
