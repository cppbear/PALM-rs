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
impl<'a> Sealed for &'a String {
    #[inline]
    fn try_entry<T>(
        self,
        map: &mut HeaderMap<T>,
    ) -> Result<Entry<'_, T>, TryEntryError> {}
    #[inline]
    fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)> {}
    fn as_str(&self) -> &str {
        self
    }
}
impl HeaderName {
    pub fn from_bytes(src: &[u8]) -> Result<HeaderName, InvalidHeaderName> {}
    pub fn from_lowercase(src: &[u8]) -> Result<HeaderName, InvalidHeaderName> {}
    #[allow(unconditional_panic)]
    pub const fn from_static(src: &'static str) -> HeaderName {}
    #[inline]
    pub fn as_str(&self) -> &str {
        match self.inner {
            Repr::Standard(v) => v.as_str(),
            Repr::Custom(ref v) => &v.0,
        }
    }
    pub(super) fn into_bytes(self) -> Bytes {}
}
