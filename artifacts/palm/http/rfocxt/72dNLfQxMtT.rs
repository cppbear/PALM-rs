use super::{Entry, HdrName, HeaderMap, HeaderName, InvalidHeaderName, MaxSizeReached};
pub trait Sealed {
    fn try_entry<T>(self, map: &mut HeaderMap<T>) -> Result<Entry<'_, T>, TryEntryError>;
    fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)>;
    fn as_str(&self) -> &str;
}
pub struct MaxSizeReached {
    _priv: (),
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
#[derive(Debug)]
pub enum Entry<'a, T: 'a> {
    /// An occupied entry
    Occupied(OccupiedEntry<'a, T>),
    /// A vacant entry
    Vacant(VacantEntry<'a, T>),
}
#[allow(missing_debug_implementations)]
pub enum TryEntryError {
    InvalidHeaderName(InvalidHeaderName),
    MaxSizeReached(MaxSizeReached),
}
impl Sealed for HeaderName {
    #[inline]
    fn try_entry<T>(
        self,
        map: &mut HeaderMap<T>,
    ) -> Result<Entry<'_, T>, TryEntryError> {
        Ok(map.try_entry2(self)?)
    }
    #[inline]
    fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)> {}
    fn as_str(&self) -> &str {}
}
