use std::fmt::{self, Display};
use syn::{Ident, Path};

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

pub const ALIAS: Symbol = Symbol("alias");
pub const BORROW: Symbol = Symbol("borrow");
pub const BOUND: Symbol = Symbol("bound");
pub const CONTENT: Symbol = Symbol("content");
pub const CRATE: Symbol = Symbol("crate");
pub const DEFAULT: Symbol = Symbol("default");
pub const DENY_UNKNOWN_FIELDS: Symbol = Symbol("deny_unknown_fields");
pub const DESERIALIZE: Symbol = Symbol("deserialize");
pub const DESERIALIZE_WITH: Symbol = Symbol("deserialize_with");
pub const EXPECTING: Symbol = Symbol("expecting");
pub const FIELD_IDENTIFIER: Symbol = Symbol("field_identifier");
pub const FLATTEN: Symbol = Symbol("flatten");
pub const FROM: Symbol = Symbol("from");
pub const GETTER: Symbol = Symbol("getter");
pub const INTO: Symbol = Symbol("into");
pub const NON_EXHAUSTIVE: Symbol = Symbol("non_exhaustive");
pub const OTHER: Symbol = Symbol("other");
pub const REMOTE: Symbol = Symbol("remote");
pub const RENAME: Symbol = Symbol("rename");
pub const RENAME_ALL: Symbol = Symbol("rename_all");
pub const RENAME_ALL_FIELDS: Symbol = Symbol("rename_all_fields");
pub const REPR: Symbol = Symbol("repr");
pub const SERDE: Symbol = Symbol("serde");
pub const SERIALIZE: Symbol = Symbol("serialize");
pub const SERIALIZE_WITH: Symbol = Symbol("serialize_with");
pub const SKIP: Symbol = Symbol("skip");
pub const SKIP_DESERIALIZING: Symbol = Symbol("skip_deserializing");
pub const SKIP_SERIALIZING: Symbol = Symbol("skip_serializing");
pub const SKIP_SERIALIZING_IF: Symbol = Symbol("skip_serializing_if");
pub const TAG: Symbol = Symbol("tag");
pub const TRANSPARENT: Symbol = Symbol("transparent");
pub const TRY_FROM: Symbol = Symbol("try_from");
pub const UNTAGGED: Symbol = Symbol("untagged");
pub const VARIANT_IDENTIFIER: Symbol = Symbol("variant_identifier");
pub const WITH: Symbol = Symbol("with");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl PartialEq<Symbol> for &Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl PartialEq<Symbol> for &Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

#[cfg(test)]
mod tests_llm_16_154 {
    use super::*;

use crate::*;
    use crate::internals::symbol::Symbol;
    use proc_macro2::Ident;

    #[test]
    fn test_eq() {
        let ident = Ident::new("test_ident", proc_macro2::Span::call_site());
        let symbol = Symbol("test_ident");

        assert!(ident.eq(&symbol));

        let different_symbol = Symbol("different_ident");
        assert!(!ident.eq(&different_symbol));
    }
}

#[cfg(test)]
mod tests_llm_16_156 {
    use super::*;

use crate::*;
    use proc_macro2::Ident;

    #[test]
    fn test_eq_ident_equals_symbol() {
        let ident = Ident::new("example", proc_macro2::Span::call_site());
        let symbol = Symbol("example");

        assert!(ident.eq(&symbol));
    }

    #[test]
    fn test_eq_ident_not_equals_symbol() {
        let ident = Ident::new("test", proc_macro2::Span::call_site());
        let symbol = Symbol("example");

        assert!(!ident.eq(&symbol));
    }

    #[test]
    fn test_eq_ident_equals_different_symbol() {
        let ident = Ident::new("test", proc_macro2::Span::call_site());
        let symbol = Symbol("test");

        assert!(ident.eq(&symbol));
    }

    #[test]
    fn test_eq_ident_empty_equals_symbol() {
        let ident = Ident::new("", proc_macro2::Span::call_site());
        let symbol = Symbol("");

        assert!(ident.eq(&symbol));
    }

    #[test]
    fn test_eq_ident_empty_not_equals_symbol() {
        let ident = Ident::new("", proc_macro2::Span::call_site());
        let symbol = Symbol("non_empty");

        assert!(!ident.eq(&symbol));
    }
}
