type AnyMap = HashMap<
    TypeId,
    Box<dyn AnyClone + Send + Sync>,
    BuildHasherDefault<IdHasher>,
>;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::hash::{BuildHasherDefault, Hasher};
#[derive(Clone, Default)]
pub struct Extensions {
    map: Option<Box<AnyMap>>,
}
#[derive(Default)]
struct IdHasher(u64);
impl Extensions {
    #[inline]
    pub fn new() -> Extensions {}
    pub fn insert<T: Clone + Send + Sync + 'static>(&mut self, val: T) -> Option<T> {}
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {}
    pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.map
            .as_mut()
            .and_then(|map| map.get_mut(&TypeId::of::<T>()))
            .and_then(|boxed| (**boxed).as_any_mut().downcast_mut())
    }
    pub fn get_or_insert<T: Clone + Send + Sync + 'static>(
        &mut self,
        value: T,
    ) -> &mut T {}
    pub fn get_or_insert_with<T: Clone + Send + Sync + 'static, F: FnOnce() -> T>(
        &mut self,
        f: F,
    ) -> &mut T {}
    pub fn get_or_insert_default<T: Default + Clone + Send + Sync + 'static>(
        &mut self,
    ) -> &mut T {}
    pub fn remove<T: Send + Sync + 'static>(&mut self) -> Option<T> {}
    #[inline]
    pub fn clear(&mut self) {}
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn len(&self) -> usize {}
    pub fn extend(&mut self, other: Self) {}
}
