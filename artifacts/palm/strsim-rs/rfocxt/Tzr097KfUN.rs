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
    fn get(&self, key: char) -> ValueType {}
    fn get_mut(&mut self, key: char) -> &mut ValueType {
        let value = key as u32;
        if value <= 255 {
            let val_u8 = u8::try_from(value).expect("we check the bounds above");
            &mut self.extended_ascii[usize::from(val_u8)]
        } else {
            self.map.get_mut(value)
        }
    }
}
impl<ValueType> GrowingHashmapChar<ValueType>
where
    ValueType: Default + Clone + Eq + Copy,
{
    fn get(&self, key: u32) -> ValueType {}
    fn get_mut(&mut self, key: u32) -> &mut ValueType {
        if self.map.is_none() {
            self.allocate();
        }
        let mut i = self.lookup(key);
        if self.map.as_ref().expect("map should have been created above")[i].value
            == Default::default()
        {
            self.fill += 1;
            if self.fill * 3 >= (self.mask + 1) * 2 {
                self.grow((self.used + 1) * 2);
                i = self.lookup(key);
            }
            self.used += 1;
        }
        let elem = &mut self
            .map
            .as_mut()
            .expect("map should have been created above")[i];
        elem.key = key;
        &mut elem.value
    }
    fn allocate(&mut self) {}
    fn lookup(&self, key: u32) -> usize {}
    fn grow(&mut self, min_used: i32) {}
}
