type Result<T> = result::Result<T, Error>;
use std::cmp::Ordering;
use std::result;
use ucd_util::{self, PropertyValues};
use hir;
use unicode_tables::age;
use unicode_tables::case_folding_simple::CASE_FOLDING_SIMPLE;
use unicode_tables::general_category;
use unicode_tables::property_bool;
use unicode_tables::property_names::PROPERTY_NAMES;
use unicode_tables::property_values::PROPERTY_VALUES;
use unicode_tables::script;
use unicode_tables::script_extension;
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ClassUnicodeRange {
    start: char,
    end: char,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
}
pub fn hir_class(ranges: &[(char, char)]) -> hir::ClassUnicode {
    let hir_ranges: Vec<hir::ClassUnicodeRange> = ranges
        .iter()
        .map(|&(s, e)| hir::ClassUnicodeRange::new(s, e))
        .collect();
    hir::ClassUnicode::new(hir_ranges)
}
