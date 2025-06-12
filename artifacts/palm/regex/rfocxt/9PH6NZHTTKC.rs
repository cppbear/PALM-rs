use std::str;
use memchr::memchr;
use re_bytes;
use re_unicode;
pub struct Captures<'t> {
    text: &'t [u8],
    locs: Locations,
    named_groups: Arc<HashMap<String, usize>>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CaptureRef<'a> {
    cap: Ref<'a>,
    end: usize,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Match<'t> {
    text: &'t [u8],
    start: usize,
    end: usize,
}
impl<'t> Captures<'t> {
    pub fn get(&self, i: usize) -> Option<Match<'t>> {
        self.locs.pos(i).map(|(s, e)| Match::new(self.text, s, e))
    }
    pub fn name(&self, name: &str) -> Option<Match<'t>> {
        self.named_groups.get(name).and_then(|&i| self.get(i))
    }
    pub fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 't> {}
    pub fn expand(&self, replacement: &[u8], dst: &mut Vec<u8>) {}
    #[inline]
    pub fn len(&self) -> usize {}
}
pub fn expand_bytes(
    caps: &re_bytes::Captures,
    mut replacement: &[u8],
    dst: &mut Vec<u8>,
) {
    while !replacement.is_empty() {
        match memchr(b'$', replacement) {
            None => break,
            Some(i) => {
                dst.extend(&replacement[..i]);
                replacement = &replacement[i..];
            }
        }
        if replacement.get(1).map_or(false, |&b| b == b'$') {
            dst.push(b'$');
            replacement = &replacement[2..];
            continue;
        }
        debug_assert!(! replacement.is_empty());
        let cap_ref = match find_cap_ref(replacement) {
            Some(cap_ref) => cap_ref,
            None => {
                dst.push(b'$');
                replacement = &replacement[1..];
                continue;
            }
        };
        replacement = &replacement[cap_ref.end..];
        match cap_ref.cap {
            Ref::Number(i) => {
                dst.extend(caps.get(i).map(|m| m.as_bytes()).unwrap_or(b""));
            }
            Ref::Named(name) => {
                dst.extend(caps.name(name).map(|m| m.as_bytes()).unwrap_or(b""));
            }
        }
    }
    dst.extend(replacement);
}
fn find_cap_ref<T: ?Sized + AsRef<[u8]>>(replacement: &T) -> Option<CaptureRef> {
    let mut i = 0;
    let rep: &[u8] = replacement.as_ref();
    if rep.len() <= 1 || rep[0] != b'$' {
        return None;
    }
    let mut brace = false;
    i += 1;
    if rep[i] == b'{' {
        brace = true;
        i += 1;
    }
    let mut cap_end = i;
    while rep.get(cap_end).map_or(false, is_valid_cap_letter) {
        cap_end += 1;
    }
    if cap_end == i {
        return None;
    }
    let cap = str::from_utf8(&rep[i..cap_end]).expect("valid UTF-8 capture name");
    if brace {
        if !rep.get(cap_end).map_or(false, |&b| b == b'}') {
            return None;
        }
        cap_end += 1;
    }
    Some(CaptureRef {
        cap: match cap.parse::<u32>() {
            Ok(i) => Ref::Number(i as usize),
            Err(_) => Ref::Named(cap),
        },
        end: cap_end,
    })
}
