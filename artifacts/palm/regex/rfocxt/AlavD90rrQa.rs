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
pub struct Captures<'t> {
    text: &'t [u8],
    locs: Locations,
    named_groups: Arc<HashMap<String, usize>>,
}
pub struct Locations(Vec<Slot>);
struct CapturesDebug<'c, 't: 'c>(&'c Captures<'t>);
impl<'t> fmt::Debug for Captures<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Captures").field(&CapturesDebug(self)).finish()
    }
}
