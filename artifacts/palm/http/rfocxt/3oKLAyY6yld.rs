type AnyMap = HashMap<
    TypeId,
    Box<dyn AnyClone + Send + Sync>,
    BuildHasherDefault<IdHasher>,
>;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::hash::{BuildHasherDefault, Hasher};
trait AnyClone: Any {
    fn clone_box(&self) -> Box<dyn AnyClone + Send + Sync>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}
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
impl<T: Clone + Send + Sync + 'static> AnyClone for T {
    fn clone_box(&self) -> Box<dyn AnyClone + Send + Sync> {}
    fn as_any(&self) -> &dyn Any {}
    fn as_any_mut(&mut self) -> &mut dyn Any {}
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
