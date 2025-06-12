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
impl Sealed for &'static str {
    #[inline]
    fn try_insert<T>(
        self,
        map: &mut HeaderMap<T>,
        val: T,
    ) -> Result<Option<T>, MaxSizeReached> {
        HdrName::from_static(self, move |hdr| map.try_insert2(hdr, val))
    }
    #[inline]
    fn try_append<T>(
        self,
        map: &mut HeaderMap<T>,
        val: T,
    ) -> Result<bool, MaxSizeReached> {}
    #[inline]
    fn try_entry<T>(
        self,
        map: &mut HeaderMap<T>,
    ) -> Result<Entry<'_, T>, MaxSizeReached> {}
}
