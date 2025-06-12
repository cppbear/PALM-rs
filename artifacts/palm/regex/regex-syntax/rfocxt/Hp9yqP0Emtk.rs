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
#[derive(Debug)]
struct AgeIter {
    ages: &'static [(&'static str, &'static [(char, char)])],
}
#[derive(Debug)]
pub enum Error {
    PropertyNotFound,
    PropertyValueNotFound,
}
fn ages(canonical_age: &str) -> Result<AgeIter> {
    const AGES: &'static [(&'static str, &'static [(char, char)])] = &[
        ("V1_1", age::V1_1),
        ("V2_0", age::V2_0),
        ("V2_1", age::V2_1),
        ("V3_0", age::V3_0),
        ("V3_1", age::V3_1),
        ("V3_2", age::V3_2),
        ("V4_0", age::V4_0),
        ("V4_1", age::V4_1),
        ("V5_0", age::V5_0),
        ("V5_1", age::V5_1),
        ("V5_2", age::V5_2),
        ("V6_0", age::V6_0),
        ("V6_1", age::V6_1),
        ("V6_2", age::V6_2),
        ("V6_3", age::V6_3),
        ("V7_0", age::V7_0),
        ("V8_0", age::V8_0),
        ("V9_0", age::V9_0),
        ("V10_0", age::V10_0),
    ];
    assert_eq!(AGES.len(), age::BY_NAME.len(), "ages are out of sync");
    let pos = AGES.iter().position(|&(age, _)| canonical_age == age);
    match pos {
        None => Err(Error::PropertyValueNotFound),
        Some(i) => Ok(AgeIter { ages: &AGES[..i + 1] }),
    }
}
