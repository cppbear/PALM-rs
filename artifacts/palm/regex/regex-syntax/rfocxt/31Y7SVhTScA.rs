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
pub enum ClassQuery<'a> {
    /// Return a class corresponding to a Unicode binary property, named by
    /// a single letter.
    OneLetter(char),
    /// Return a class corresponding to a Unicode binary property.
    ///
    /// Note that, by special exception (see UTS#18, Section 1.2), both
    /// general category values and script values are permitted here as if
    /// they were a binary property.
    Binary(&'a str),
    /// Return a class corresponding to all codepoints whose property
    /// (identified by `property_name`) corresponds to the given value
    /// (identified by `property_value`).
    ByValue {
        /// A property name.
        property_name: &'a str,
        /// A property value.
        property_value: &'a str,
    },
}
#[derive(Debug)]
pub enum Error {
    PropertyNotFound,
    PropertyValueNotFound,
}
#[derive(Debug, Eq, PartialEq)]
enum CanonicalClassQuery {
    /// The canonical binary property name.
    Binary(&'static str),
    /// The canonical general category name.
    GeneralCategory(&'static str),
    /// The canonical script name.
    Script(&'static str),
    /// An arbitrary association between property and value, both of which
    /// have been canonicalized.
    ///
    /// Note that by construction, the property name of ByValue will never
    /// be General_Category or Script. Those two cases are subsumed by the
    /// eponymous variants.
    ByValue {
        /// The canonical property name.
        property_name: &'static str,
        /// The canonical property value.
        property_value: &'static str,
    },
}
impl<'a> ClassQuery<'a> {
    fn canonicalize(&self) -> Result<CanonicalClassQuery> {
        match *self {
            ClassQuery::OneLetter(c) => self.canonical_binary(&c.to_string()),
            ClassQuery::Binary(name) => self.canonical_binary(name),
            ClassQuery::ByValue { property_name, property_value } => {
                let property_name = normalize(property_name);
                let property_value = normalize(property_value);
                let canon_name = match canonical_prop(&property_name) {
                    None => return Err(Error::PropertyNotFound),
                    Some(canon_name) => canon_name,
                };
                Ok(
                    match canon_name {
                        "General_Category" => {
                            let canon = match canonical_gencat(&property_value) {
                                None => return Err(Error::PropertyValueNotFound),
                                Some(canon) => canon,
                            };
                            CanonicalClassQuery::GeneralCategory(canon)
                        }
                        "Script" => {
                            let canon = match canonical_script(&property_value) {
                                None => return Err(Error::PropertyValueNotFound),
                                Some(canon) => canon,
                            };
                            CanonicalClassQuery::Script(canon)
                        }
                        _ => {
                            let vals = match property_values(canon_name) {
                                None => return Err(Error::PropertyValueNotFound),
                                Some(vals) => vals,
                            };
                            let canon_val = match canonical_value(
                                vals,
                                &property_value,
                            ) {
                                None => return Err(Error::PropertyValueNotFound),
                                Some(canon_val) => canon_val,
                            };
                            CanonicalClassQuery::ByValue {
                                property_name: canon_name,
                                property_value: canon_val,
                            }
                        }
                    },
                )
            }
        }
    }
    fn canonical_binary(&self, name: &str) -> Result<CanonicalClassQuery> {
        let norm = normalize(name);
        if let Some(canon) = canonical_prop(&norm) {
            return Ok(CanonicalClassQuery::Binary(canon));
        }
        if let Some(canon) = canonical_gencat(&norm) {
            return Ok(CanonicalClassQuery::GeneralCategory(canon));
        }
        if let Some(canon) = canonical_script(&norm) {
            return Ok(CanonicalClassQuery::Script(canon));
        }
        Err(Error::PropertyNotFound)
    }
}
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
fn canonical_prop(normalized_name: &str) -> Option<&'static str> {
    ucd_util::canonical_property_name(PROPERTY_NAMES, normalized_name)
}
fn canonical_script(normalized_value: &str) -> Option<&'static str> {
    let scripts = property_values("Script").unwrap();
    canonical_value(scripts, normalized_value)
}
fn canonical_value(
    vals: PropertyValues,
    normalized_value: &str,
) -> Option<&'static str> {
    ucd_util::canonical_property_value(vals, normalized_value)
}
fn normalize(x: &str) -> String {
    let mut x = x.to_string();
    ucd_util::symbolic_name_normalize(&mut x);
    x
}
fn property_values(canonical_property_name: &'static str) -> Option<PropertyValues> {
    ucd_util::property_values(PROPERTY_VALUES, canonical_property_name)
}
