use std::cmp;
use std::fmt;
use std::iter;
use std::mem;
use std::ops;
use hir::{self, Hir, HirKind};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassBytes {
    set: IntervalSet<ClassBytesRange>,
}
#[derive(Debug)]
pub struct ClassBytesIter<'a>(IntervalSetIter<'a, ClassBytesRange>);
impl ClassBytes {
    pub fn new<I>(ranges: I) -> ClassBytes
    where
        I: IntoIterator<Item = ClassBytesRange>,
    {}
    pub fn empty() -> ClassBytes {}
    pub fn push(&mut self, range: ClassBytesRange) {}
    pub fn iter(&self) -> ClassBytesIter {
        ClassBytesIter(self.set.iter())
    }
    pub fn ranges(&self) -> &[ClassBytesRange] {}
    pub fn case_fold_simple(&mut self) {}
    pub fn negate(&mut self) {}
    pub fn union(&mut self, other: &ClassBytes) {}
    pub fn intersect(&mut self, other: &ClassBytes) {}
    pub fn difference(&mut self, other: &ClassBytes) {}
    pub fn symmetric_difference(&mut self, other: &ClassBytes) {}
    pub fn is_all_ascii(&self) -> bool {}
}
fn cls_byte_count(cls: &hir::ClassBytes) -> usize {
    cls.iter().map(|&r| 1 + (r.end as u32) - (r.start as u32)).sum::<u32>() as usize
}
