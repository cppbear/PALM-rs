use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::ops::Index;
use std::str::FromStr;
use std::sync::Arc;
use memchr::memchr;
use syntax;
use error::Error;
use exec::{Exec, ExecNoSyncStr};
use expand::expand_str;
use re_builder::unicode::RegexBuilder;
use re_trait::{self, RegularExpression, Locations, SubCapturesPosIter};
pub trait Replacer {
    fn replace_append(&mut self, caps: &Captures, dst: &mut String);
    fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>> {
        None
    }
    fn by_ref<'r>(&'r mut self) -> ReplacerRef<'r, Self> {
        ReplacerRef(self)
    }
}
pub struct NoExpand<'t>(pub &'t str);
impl<'t> Replacer for NoExpand<'t> {
    fn replace_append(&mut self, _: &Captures, dst: &mut String) {}
    fn no_expansion(&mut self) -> Option<Cow<str>> {
        Some(Cow::Borrowed(self.0))
    }
}
