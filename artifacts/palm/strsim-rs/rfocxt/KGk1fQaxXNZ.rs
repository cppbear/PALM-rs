pub type HammingResult = Result<usize, StrSimError>;
use std::char;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;
use std::mem;
use std::str::Chars;
struct GrowingHashmapChar<ValueType> {
    used: i32,
    fill: i32,
    mask: i32,
    map: Option<Vec<GrowingHashmapMapElemChar<ValueType>>>,
}
#[derive(Default, Clone)]
struct GrowingHashmapMapElemChar<ValueType> {
    key: u32,
    value: ValueType,
}
impl<ValueType> GrowingHashmapChar<ValueType>
where
    ValueType: Default + Clone + Eq + Copy,
{
    fn get(&self, key: u32) -> ValueType {}
    fn get_mut(&mut self, key: u32) -> &mut ValueType {}
    fn allocate(&mut self) {}
    fn lookup(&self, key: u32) -> usize {
        let hash = key;
        let mut i = hash as usize & self.mask as usize;
        let map = self.map.as_ref().expect("callers have to ensure map is allocated");
        if map[i].value == Default::default() || map[i].key == key {
            return i;
        }
        let mut perturb = key;
        loop {
            i = (i * 5 + perturb as usize + 1) & self.mask as usize;
            if map[i].value == Default::default() || map[i].key == key {
                return i;
            }
            perturb >>= 5;
        }
    }
    fn grow(&mut self, min_used: i32) {}
}
