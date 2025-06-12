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
struct HybridGrowingHashmapChar<ValueType> {
    map: GrowingHashmapChar<ValueType>,
    extended_ascii: [ValueType; 256],
}
struct GrowingHashmapChar<ValueType> {
    used: i32,
    fill: i32,
    mask: i32,
    map: Option<Vec<GrowingHashmapMapElemChar<ValueType>>>,
}
impl<ValueType> HybridGrowingHashmapChar<ValueType>
where
    ValueType: Default + Clone + Copy + Eq,
{
    fn get(&self, key: char) -> ValueType {
        let value = key as u32;
        if value <= 255 {
            let val_u8 = u8::try_from(value).expect("we check the bounds above");
            self.extended_ascii[usize::from(val_u8)]
        } else {
            self.map.get(value)
        }
    }
    fn get_mut(&mut self, key: char) -> &mut ValueType {}
}
impl<ValueType> GrowingHashmapChar<ValueType>
where
    ValueType: Default + Clone + Eq + Copy,
{
    fn get(&self, key: u32) -> ValueType {
        self.map
            .as_ref()
            .map_or_else(|| Default::default(), |map| map[self.lookup(key)].value)
    }
    fn get_mut(&mut self, key: u32) -> &mut ValueType {}
    fn allocate(&mut self) {}
    fn lookup(&self, key: u32) -> usize {}
    fn grow(&mut self, min_used: i32) {}
}
