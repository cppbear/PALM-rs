use super::{Entry, HdrName, HeaderMap, HeaderName, MaxSizeReached};
pub trait Sealed {
    fn try_insert<T>(
        self,
        map: &mut HeaderMap<T>,
        val: T,
    ) -> Result<Option<T>, MaxSizeReached>;
    fn try_append<T>(
        self,
        map: &mut HeaderMap<T>,
        val: T,
    ) -> Result<bool, MaxSizeReached>;
    fn try_entry<T>(
        self,
        map: &mut HeaderMap<T>,
    ) -> Result<Entry<'_, T>, MaxSizeReached>;
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
pub struct MaxSizeReached {
    _priv: (),
}
impl Sealed for HeaderName {
    #[inline]
    fn try_insert<T>(
        self,
        map: &mut HeaderMap<T>,
        val: T,
    ) -> Result<Option<T>, MaxSizeReached> {}
    #[inline]
    fn try_append<T>(
        self,
        map: &mut HeaderMap<T>,
        val: T,
    ) -> Result<bool, MaxSizeReached> {
        map.try_append2(self, val)
    }
    #[inline]
    fn try_entry<T>(
        self,
        map: &mut HeaderMap<T>,
    ) -> Result<Entry<'_, T>, MaxSizeReached> {}
}
