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
    fn allocate(&mut self) {
        self.mask = 8 - 1;
        self.map = Some(vec![GrowingHashmapMapElemChar::default(); 8]);
    }
    fn lookup(&self, key: u32) -> usize {}
    fn grow(&mut self, min_used: i32) {}
}
