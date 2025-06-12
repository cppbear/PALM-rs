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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
}
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
impl Iterator for AgeIter {
    type Item = &'static [(char, char)];
    fn next(&mut self) -> Option<&'static [(char, char)]> {
        if self.ages.is_empty() {
            None
        } else {
            let set = self.ages[0];
            self.ages = &self.ages[1..];
            Some(set.1)
        }
    }
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
    fn canonical_binary(&self, name: &str) -> Result<CanonicalClassQuery> {}
}
impl ClassUnicode {
    pub fn new<I>(ranges: I) -> ClassUnicode
    where
        I: IntoIterator<Item = ClassUnicodeRange>,
    {}
    pub fn empty() -> ClassUnicode {
        ClassUnicode::new(vec![])
    }
    pub fn push(&mut self, range: ClassUnicodeRange) {}
    pub fn iter(&self) -> ClassUnicodeIter {}
    pub fn ranges(&self) -> &[ClassUnicodeRange] {}
    pub fn case_fold_simple(&mut self) {}
    pub fn negate(&mut self) {
        self.set.negate();
    }
    pub fn union(&mut self, other: &ClassUnicode) {
        self.set.union(&other.set);
    }
    pub fn intersect(&mut self, other: &ClassUnicode) {}
    pub fn difference(&mut self, other: &ClassUnicode) {}
    pub fn symmetric_difference(&mut self, other: &ClassUnicode) {}
}
pub fn class<'a>(query: ClassQuery<'a>) -> Result<hir::ClassUnicode> {
    use self::CanonicalClassQuery::*;
    match query.canonicalize()? {
        Binary(name) => {
            property_set(property_bool::BY_NAME, name)
                .map(hir_class)
                .ok_or(Error::PropertyNotFound)
        }
        GeneralCategory("Any") => Ok(hir_class(&[('\0', '\u{10FFFF}')])),
        GeneralCategory("Assigned") => {
            let mut cls = property_set(general_category::BY_NAME, "Unassigned")
                .map(hir_class)
                .ok_or(Error::PropertyNotFound)?;
            cls.negate();
            Ok(cls)
        }
        GeneralCategory("ASCII") => Ok(hir_class(&[('\0', '\x7F')])),
        GeneralCategory(name) => {
            property_set(general_category::BY_NAME, name)
                .map(hir_class)
                .ok_or(Error::PropertyValueNotFound)
        }
        Script(name) => {
            property_set(script::BY_NAME, name)
                .map(hir_class)
                .ok_or(Error::PropertyValueNotFound)
        }
        ByValue { property_name: "Age", property_value } => {
            let mut class = hir::ClassUnicode::empty();
            for set in ages(property_value)? {
                class.union(&hir_class(set));
            }
            Ok(class)
        }
        ByValue { property_name: "Script_Extensions", property_value } => {
            property_set(script_extension::BY_NAME, property_value)
                .map(hir_class)
                .ok_or(Error::PropertyValueNotFound)
        }
        _ => Err(Error::PropertyNotFound),
    }
}
fn property_set(
    name_map: &'static [(&'static str, &'static [(char, char)])],
    canonical: &'static str,
) -> Option<&'static [(char, char)]> {
    name_map.binary_search_by_key(&canonical, |x| x.0).ok().map(|i| name_map[i].1)
}
pub fn hir_class(ranges: &[(char, char)]) -> hir::ClassUnicode {
    let hir_ranges: Vec<hir::ClassUnicodeRange> = ranges
        .iter()
        .map(|&(s, e)| hir::ClassUnicodeRange::new(s, e))
        .collect();
    hir::ClassUnicode::new(hir_ranges)
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
