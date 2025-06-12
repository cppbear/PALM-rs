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
#[derive(Clone)]
pub struct HeaderMap<T = HeaderValue> {
    mask: Size,
    indices: Box<[Pos]>,
    entries: Vec<Bucket<T>>,
    extra_values: Vec<ExtraValue<T>>,
    danger: Danger,
}
pub struct MaxSizeReached {
    _priv: (),
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct HeaderName {
    inner: Repr<Custom>,
}
#[allow(missing_debug_implementations)]
pub enum TryEntryError {
    InvalidHeaderName(InvalidHeaderName),
    MaxSizeReached(MaxSizeReached),
}
#[derive(Debug)]
pub enum Entry<'a, T: 'a> {
    /// An occupied entry
    Occupied(OccupiedEntry<'a, T>),
    /// A vacant entry
    Vacant(VacantEntry<'a, T>),
}
impl<'a> Sealed for &'a String {
    #[inline]
    fn try_entry<T>(
        self,
        map: &mut HeaderMap<T>,
    ) -> Result<Entry<'_, T>, TryEntryError> {
        self.as_str().try_entry(map)
    }
    #[inline]
    fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)> {}
    fn as_str(&self) -> &str {}
}
