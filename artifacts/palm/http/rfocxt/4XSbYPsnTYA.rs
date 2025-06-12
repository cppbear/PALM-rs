use super::{Entry, HdrName, HeaderMap, HeaderName, InvalidHeaderName, MaxSizeReached};
pub trait Sealed {
    fn try_entry<T>(self, map: &mut HeaderMap<T>) -> Result<Entry<'_, T>, TryEntryError>;
    fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)>;
    fn as_str(&self) -> &str;
}
trait AnyClone: Any {
    fn clone_box(&self) -> Box<dyn AnyClone + Send + Sync>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct HeaderName {
    inner: Repr<Custom>,
}
#[derive(Clone)]
pub struct HeaderMap<T = HeaderValue> {
    mask: Size,
    indices: Box<[Pos]>,
    entries: Vec<Bucket<T>>,
    extra_values: Vec<ExtraValue<T>>,
    danger: Danger,
}
impl<'a> Sealed for &'a String {
    #[inline]
    fn try_entry<T>(
        self,
        map: &mut HeaderMap<T>,
    ) -> Result<Entry<'_, T>, TryEntryError> {}
    #[inline]
    fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)> {
        Sealed::find(*self, map)
    }
    fn as_str(&self) -> &str {}
}
