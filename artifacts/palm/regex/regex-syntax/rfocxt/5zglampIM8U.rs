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
fn property_values(canonical_property_name: &'static str) -> Option<PropertyValues> {
    ucd_util::property_values(PROPERTY_VALUES, canonical_property_name)
}
