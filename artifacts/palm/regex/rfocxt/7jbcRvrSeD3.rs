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
struct CapturesDebug<'c, 't: 'c>(&'c Captures<'t>);
pub struct Locations(Vec<Slot>);
pub struct SubCapturesPosIter<'c> {
    idx: usize,
    locs: &'c Locations,
}
pub struct Captures<'t> {
    text: &'t str,
    locs: Locations,
    named_groups: Arc<HashMap<String, usize>>,
}
pub struct Captures<'t> {
    text: &'t [u8],
    locs: Locations,
    named_groups: Arc<HashMap<String, usize>>,
}
impl<'c, 't> fmt::Debug for CapturesDebug<'c, 't> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn escape_bytes(bytes: &[u8]) -> String {
            let mut s = String::new();
            for &b in bytes {
                s.push_str(&escape_byte(b));
            }
            s
        }
        fn escape_byte(byte: u8) -> String {
            use std::ascii::escape_default;
            let escaped: Vec<u8> = escape_default(byte).collect();
            String::from_utf8_lossy(&escaped).into_owned()
        }
        let slot_to_name: HashMap<&usize, &String> = self
            .0
            .named_groups
            .iter()
            .map(|(a, b)| (b, a))
            .collect();
        let mut map = f.debug_map();
        for (slot, m) in self.0.locs.iter().enumerate() {
            let m = m.map(|(s, e)| escape_bytes(&self.0.text[s..e]));
            if let Some(name) = slot_to_name.get(&slot) {
                map.entry(&name, &m);
            } else {
                map.entry(&slot, &m);
            }
        }
        map.finish()
    }
}
impl Locations {
    pub fn pos(&self, i: usize) -> Option<(usize, usize)> {}
    pub fn iter(&self) -> SubCapturesPosIter {
        SubCapturesPosIter {
            idx: 0,
            locs: self,
        }
    }
    pub fn len(&self) -> usize {}
}
