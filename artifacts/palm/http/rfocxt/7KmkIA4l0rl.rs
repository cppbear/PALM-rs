type AnyMap = HashMap<
    TypeId,
    Box<dyn AnyClone + Send + Sync>,
    BuildHasherDefault<IdHasher>,
>;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::hash::{BuildHasherDefault, Hasher};
#[derive(Default)]
struct IdHasher(u64);
impl Hasher for IdHasher {
    fn write(&mut self, _: &[u8]) {
        unreachable!("TypeId calls write_u64");
    }
    #[inline]
    fn write_u64(&mut self, id: u64) {}
    #[inline]
    fn finish(&self) -> u64 {}
}
