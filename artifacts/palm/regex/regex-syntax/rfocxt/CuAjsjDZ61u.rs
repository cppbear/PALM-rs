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
fn canonical_gencat(normalized_value: &str) -> Option<&'static str> {
    match normalized_value {
        "any" => Some("Any"),
        "assigned" => Some("Assigned"),
        "ascii" => Some("ASCII"),
        _ => {
            let gencats = property_values("General_Category").unwrap();
            canonical_value(gencats, normalized_value)
        }
    }
}
fn canonical_value(
    vals: PropertyValues,
    normalized_value: &str,
) -> Option<&'static str> {
    ucd_util::canonical_property_value(vals, normalized_value)
}
fn property_values(canonical_property_name: &'static str) -> Option<PropertyValues> {
    ucd_util::property_values(PROPERTY_VALUES, canonical_property_name)
}
