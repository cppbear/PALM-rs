use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::ops::Index;
use std::str::FromStr;
use std::sync::Arc;
use memchr::memchr;
use exec::{Exec, ExecNoSync};
use expand::expand_bytes;
use error::Error;
use re_builder::bytes::RegexBuilder;
use re_trait::{self, RegularExpression, Locations, SubCapturesPosIter};
pub trait Replacer {
    fn replace_append(&mut self, caps: &Captures, dst: &mut Vec<u8>);
    fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, [u8]>> {
        None
    }
    fn by_ref<'r>(&'r mut self) -> ReplacerRef<'r, Self> {
        ReplacerRef(self)
    }
}
#[derive(Debug)]
pub struct ReplacerRef<'a, R: ?Sized + 'a>(&'a mut R);
pub struct Captures<'t> {
    text: &'t [u8],
    locs: Locations,
    named_groups: Arc<HashMap<String, usize>>,
}
impl<'a, R: Replacer + ?Sized + 'a> Replacer for ReplacerRef<'a, R> {
    fn replace_append(&mut self, caps: &Captures, dst: &mut Vec<u8>) {
        self.0.replace_append(caps, dst)
    }
    fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, [u8]>> {}
}
