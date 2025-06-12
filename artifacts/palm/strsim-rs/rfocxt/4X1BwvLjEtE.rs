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
#[derive(Clone, Copy, PartialEq, Eq)]
struct RowId {
    val: isize,
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
fn damerau_levenshtein_impl<Iter1, Iter2>(
    s1: Iter1,
    len1: usize,
    s2: Iter2,
    len2: usize,
) -> usize
where
    Iter1: Iterator<Item = char> + Clone,
    Iter2: Iterator<Item = char> + Clone,
{
    let max_val = max(len1, len2) as isize + 1;
    let mut last_row_id = HybridGrowingHashmapChar::<RowId>::default();
    let size = len2 + 2;
    let mut fr = vec![max_val; size];
    let mut r1 = vec![max_val; size];
    let mut r: Vec<isize> = (max_val..max_val + 1)
        .chain(0..(size - 1) as isize)
        .collect();
    for (i, ch1) in s1.enumerate().map(|(i, ch1)| (i + 1, ch1)) {
        mem::swap(&mut r, &mut r1);
        let mut last_col_id: isize = -1;
        let mut last_i2l1 = r[1];
        r[1] = i as isize;
        let mut t = max_val;
        for (j, ch2) in s2.clone().enumerate().map(|(j, ch2)| (j + 1, ch2)) {
            let diag = r1[j] + isize::from(ch1 != ch2);
            let left = r[j] + 1;
            let up = r1[j + 1] + 1;
            let mut temp = min(diag, min(left, up));
            if ch1 == ch2 {
                last_col_id = j as isize;
                fr[j + 1] = r1[j - 1];
                t = last_i2l1;
            } else {
                let k = last_row_id.get(ch2).val;
                let l = last_col_id;
                if j as isize - l == 1 {
                    let transpose = fr[j + 1] + (i as isize - k);
                    temp = min(temp, transpose);
                } else if i as isize - k == 1 {
                    let transpose = t + (j as isize - l);
                    temp = min(temp, transpose);
                }
            }
            last_i2l1 = r[j + 1];
            r[j + 1] = temp;
        }
        last_row_id.get_mut(ch1).val = i as isize;
    }
    r[len2 + 1] as usize
}
