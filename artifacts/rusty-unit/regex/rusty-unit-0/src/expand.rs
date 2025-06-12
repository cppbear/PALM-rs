use std::str;

use crate::find_byte::find_byte;

use crate::re_bytes;
use crate::re_unicode;

pub fn expand_str(
    caps: &re_unicode::Captures<'_>,
    mut replacement: &str,
    dst: &mut String,
) {
    while !replacement.is_empty() {
        match find_byte(b'$', replacement.as_bytes()) {
            None => break,
            Some(i) => {
                dst.push_str(&replacement[..i]);
                replacement = &replacement[i..];
            }
        }
        if replacement.as_bytes().get(1).map_or(false, |&b| b == b'$') {
            dst.push_str("$");
            replacement = &replacement[2..];
            continue;
        }
        debug_assert!(!replacement.is_empty());
        let cap_ref = match find_cap_ref(replacement.as_bytes()) {
            Some(cap_ref) => cap_ref,
            None => {
                dst.push_str("$");
                replacement = &replacement[1..];
                continue;
            }
        };
        replacement = &replacement[cap_ref.end..];
        match cap_ref.cap {
            Ref::Number(i) => {
                dst.push_str(caps.get(i).map(|m| m.as_str()).unwrap_or(""));
            }
            Ref::Named(name) => {
                dst.push_str(
                    caps.name(name).map(|m| m.as_str()).unwrap_or(""),
                );
            }
        }
    }
    dst.push_str(replacement);
}

pub fn expand_bytes(
    caps: &re_bytes::Captures<'_>,
    mut replacement: &[u8],
    dst: &mut Vec<u8>,
) {
    while !replacement.is_empty() {
        match find_byte(b'$', replacement) {
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
        debug_assert!(!replacement.is_empty());
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
                dst.extend(
                    caps.name(name).map(|m| m.as_bytes()).unwrap_or(b""),
                );
            }
        }
    }
    dst.extend(replacement);
}

/// `CaptureRef` represents a reference to a capture group inside some text.
/// The reference is either a capture group name or a number.
///
/// It is also tagged with the position in the text following the
/// capture reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CaptureRef<'a> {
    cap: Ref<'a>,
    end: usize,
}

/// A reference to a capture group in some text.
///
/// e.g., `$2`, `$foo`, `${foo}`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Ref<'a> {
    Named(&'a str),
    Number(usize),
}

impl<'a> From<&'a str> for Ref<'a> {
    fn from(x: &'a str) -> Ref<'a> {
        Ref::Named(x)
    }
}

impl From<usize> for Ref<'static> {
    fn from(x: usize) -> Ref<'static> {
        Ref::Number(x)
    }
}

/// Parses a possible reference to a capture group name in the given text,
/// starting at the beginning of `replacement`.
///
/// If no such valid reference could be found, None is returned.
fn find_cap_ref(replacement: &[u8]) -> Option<CaptureRef<'_>> {
    let mut i = 0;
    let rep: &[u8] = replacement;
    if rep.len() <= 1 || rep[0] != b'$' {
        return None;
    }
    i += 1;
    if rep[i] == b'{' {
        return find_cap_ref_braced(rep, i + 1);
    }
    let mut cap_end = i;
    while rep.get(cap_end).copied().map_or(false, is_valid_cap_letter) {
        cap_end += 1;
    }
    if cap_end == i {
        return None;
    }
    // We just verified that the range 0..cap_end is valid ASCII, so it must
    // therefore be valid UTF-8. If we really cared, we could avoid this UTF-8
    // check via an unchecked conversion or by parsing the number straight from
    // &[u8].
    let cap =
        str::from_utf8(&rep[i..cap_end]).expect("valid UTF-8 capture name");
    Some(CaptureRef {
        cap: match cap.parse::<u32>() {
            Ok(i) => Ref::Number(i as usize),
            Err(_) => Ref::Named(cap),
        },
        end: cap_end,
    })
}

fn find_cap_ref_braced(rep: &[u8], mut i: usize) -> Option<CaptureRef<'_>> {
    let start = i;
    while rep.get(i).map_or(false, |&b| b != b'}') {
        i += 1;
    }
    if !rep.get(i).map_or(false, |&b| b == b'}') {
        return None;
    }
    // When looking at braced names, we don't put any restrictions on the name,
    // so it's possible it could be invalid UTF-8. But a capture group name
    // can never be invalid UTF-8, so if we have invalid UTF-8, then we can
    // safely return None.
    let cap = match str::from_utf8(&rep[start..i]) {
        Err(_) => return None,
        Ok(cap) => cap,
    };
    Some(CaptureRef {
        cap: match cap.parse::<u32>() {
            Ok(i) => Ref::Number(i as usize),
            Err(_) => Ref::Named(cap),
        },
        end: i + 1,
    })
}

/// Returns true if and only if the given byte is allowed in a capture name.
fn is_valid_cap_letter(b: u8) -> bool {
    match b {
        b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'_' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{find_cap_ref, CaptureRef};

    macro_rules! find {
        ($name:ident, $text:expr) => {
            #[test]
            fn $name() {
                assert_eq!(None, find_cap_ref($text.as_bytes()));
            }
        };
        ($name:ident, $text:expr, $capref:expr) => {
            #[test]
            fn $name() {
                assert_eq!(Some($capref), find_cap_ref($text.as_bytes()));
            }
        };
    }

    macro_rules! c {
        ($name_or_number:expr, $pos:expr) => {
            CaptureRef { cap: $name_or_number.into(), end: $pos }
        };
    }

    find!(find_cap_ref1, "$foo", c!("foo", 4));
    find!(find_cap_ref2, "${foo}", c!("foo", 6));
    find!(find_cap_ref3, "$0", c!(0, 2));
    find!(find_cap_ref4, "$5", c!(5, 2));
    find!(find_cap_ref5, "$10", c!(10, 3));
    // See https://github.com/rust-lang/regex/pull/585
    // for more on characters following numbers
    find!(find_cap_ref6, "$42a", c!("42a", 4));
    find!(find_cap_ref7, "${42}a", c!(42, 5));
    find!(find_cap_ref8, "${42");
    find!(find_cap_ref9, "${42 ");
    find!(find_cap_ref10, " $0 ");
    find!(find_cap_ref11, "$");
    find!(find_cap_ref12, " ");
    find!(find_cap_ref13, "");
    find!(find_cap_ref14, "$1-$2", c!(1, 2));
    find!(find_cap_ref15, "$1_$2", c!("1_", 3));
    find!(find_cap_ref16, "$x-$y", c!("x", 2));
    find!(find_cap_ref17, "$x_$y", c!("x_", 3));
    find!(find_cap_ref18, "${#}", c!("#", 4));
    find!(find_cap_ref19, "${Z[}", c!("Z[", 5));
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::iter::IntoIterator;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::str::FromStr;
	use std::convert::From;
	use std::cmp::Eq;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_14() {
    rusty_monitor::set_test_id(14);
    let mut usize_0: usize = 1700usize;
    let mut result_0: dfa::Result<i32> = crate::dfa::Result::NoMatch(usize_0);
    let mut result_0_ref_0: &dfa::Result<i32> = &mut result_0;
    let mut usize_1: usize = 7455usize;
    let mut str_0: &str = "68Pvy4HNeK";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut regexset_0: crate::re_set::unicode::RegexSet = crate::re_set::unicode::RegexSet::empty();
    let mut regexset_0_ref_0: &crate::re_set::unicode::RegexSet = &mut regexset_0;
    let mut regexset_1: crate::re_set::bytes::RegexSet = crate::re_set::bytes::RegexSet::empty();
    let mut regexset_1_ref_0: &crate::re_set::bytes::RegexSet = &mut regexset_1;
    let mut regexset_2: crate::re_set::bytes::RegexSet = crate::re_set::bytes::RegexSet::empty();
    let mut regexset_2_ref_0: &crate::re_set::bytes::RegexSet = &mut regexset_2;
    let mut usize_2: usize = 4365usize;
    let mut bool_0: bool = true;
    let mut regexoptions_0: crate::re_builder::RegexOptions = crate::re_builder::RegexOptions::default();
    let mut execbuilder_0: crate::exec::ExecBuilder = crate::exec::ExecBuilder::new_options(regexoptions_0);
    let mut bool_1: bool = false;
    let mut str_1: &str = "QOGriFp";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_2: bool = true;
    let mut usize_3: usize = 1475usize;
    let mut str_2: &str = "O";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_4: usize = 7491usize;
    let mut usize_5: usize = 4781usize;
    let mut str_3: &str = "V0YnqveZy3";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut ref_0: expand::Ref = crate::expand::Ref::Named(str_3_ref_0);
    let mut captureref_0: crate::expand::CaptureRef = crate::expand::CaptureRef {cap: ref_0, end: usize_5};
    let mut captureref_0_ref_0: &crate::expand::CaptureRef = &mut captureref_0;
    let mut execbuilder_1: crate::exec::ExecBuilder = crate::exec::ExecBuilder::automatic(execbuilder_0);
    let mut result_1: dfa::Result<i32> = crate::dfa::Result::clone(result_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_25() {
    rusty_monitor::set_test_id(25);
    let mut char_0: char = '(';
    let mut char_1: crate::input::Char = crate::input::Char::from(char_0);
    let mut char_1_ref_0: &crate::input::Char = &mut char_1;
    let mut usize_0: usize = 6036usize;
    let mut usize_1: usize = 6357usize;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut char_2: char = 'v';
    let mut char_3: crate::input::Char = crate::input::Char::from(char_2);
    let mut char_3_ref_0: &crate::input::Char = &mut char_3;
    let mut usize_2: usize = 6639usize;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut usize_3: usize = 9675usize;
    let mut usize_4: usize = 9158usize;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut usize_5: usize = 1501usize;
    let mut usize_6: usize = 7184usize;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut usize_7: usize = 6127usize;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut usize_8: usize = 7219usize;
    let mut usize_9: usize = 1180usize;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut str_0: &str = "Dpvrb";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut regexset_0: crate::re_set::unicode::RegexSet = crate::re_set::unicode::RegexSet::empty();
    let mut regexset_0_ref_0: &crate::re_set::unicode::RegexSet = &mut regexset_0;
    let mut setmatches_0: crate::re_set::unicode::SetMatches = crate::re_set::unicode::RegexSet::matches(regexset_0_ref_0, str_0_ref_0);
    let mut setmatches_0_ref_0: &crate::re_set::unicode::SetMatches = &mut setmatches_0;
    let mut u8_0: u8 = 56u8;
    let mut usize_10: usize = 3287usize;
    let mut usize_11: usize = 5810usize;
    let mut bool_12: bool = crate::input::Char::eq(char_3_ref_0, char_1_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_38() {
//     rusty_monitor::set_test_id(38);
//     let mut suffixcachekey_0: crate::compile::SuffixCacheKey = crate::compile::SuffixCacheKey::default();
//     let mut suffixcachekey_0_ref_0: &crate::compile::SuffixCacheKey = &mut suffixcachekey_0;
//     let mut suffixcachekey_1: crate::compile::SuffixCacheKey = crate::compile::SuffixCacheKey::default();
//     let mut suffixcachekey_1_ref_0: &crate::compile::SuffixCacheKey = &mut suffixcachekey_1;
//     let mut str_0: &str = "w8IuI00";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut string_0: std::string::String = crate::re_unicode::escape(str_0_ref_0);
//     let mut error_0: error::Error = crate::error::Error::Syntax(string_0);
//     let mut error_0_ref_0: &error::Error = &mut error_0;
//     let mut usize_0: usize = 5564usize;
//     let mut program_0: crate::prog::Program = crate::prog::Program::new();
//     let mut program_0_ref_0: &crate::prog::Program = &mut program_0;
//     let mut str_1: &str = "EG8vrmQ7N";
//     let mut str_1_ref_0: &str = &mut str_1;
//     let mut regexset_0: crate::re_set::unicode::RegexSet = crate::re_set::unicode::RegexSet::empty();
//     let mut regexset_0_ref_0: &crate::re_set::unicode::RegexSet = &mut regexset_0;
//     let mut setmatches_0: crate::re_set::unicode::SetMatches = crate::re_set::unicode::RegexSet::matches(regexset_0_ref_0, str_1_ref_0);
//     let mut setmatches_0_ref_0: &crate::re_set::unicode::SetMatches = &mut setmatches_0;
//     let mut usize_1: usize = 5604usize;
//     let mut usize_2: usize = 2231usize;
//     let mut ref_0: expand::Ref = crate::expand::Ref::Number(usize_2);
//     let mut captureref_0: crate::expand::CaptureRef = crate::expand::CaptureRef {cap: ref_0, end: usize_1};
//     let mut captureref_0_ref_0: &crate::expand::CaptureRef = &mut captureref_0;
//     let mut program_1: crate::prog::Program = crate::prog::Program::new();
//     let mut program_1_ref_0: &crate::prog::Program = &mut program_1;
//     let mut bool_0: bool = crate::dfa::can_exec(program_1_ref_0);
//     let mut tuple_0: () = crate::expand::CaptureRef::assert_receiver_is_total_eq(captureref_0_ref_0);
//     let mut usize_3: usize = crate::prog::Program::skip(program_0_ref_0, usize_0);
//     let mut setmatchesiter_0: crate::re_set::unicode::SetMatchesIter = crate::re_set::unicode::SetMatches::into_iter(setmatches_0_ref_0);
//     let mut setmatchesiter_0_ref_0: &crate::re_set::unicode::SetMatchesIter = &mut setmatchesiter_0;
//     let mut setmatchesiter_1: crate::re_set::unicode::SetMatchesIter = crate::re_set::unicode::SetMatchesIter::clone(setmatchesiter_0_ref_0);
//     let mut error_1: error::Error = crate::error::Error::clone(error_0_ref_0);
//     let mut bool_1: bool = crate::compile::SuffixCacheKey::eq(suffixcachekey_1_ref_0, suffixcachekey_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_47() {
//     rusty_monitor::set_test_id(47);
//     let mut regexset_0: crate::re_set::bytes::RegexSet = crate::re_set::bytes::RegexSet::empty();
//     let mut regexset_0_ref_0: &crate::re_set::bytes::RegexSet = &mut regexset_0;
//     let mut usize_0: usize = 3927usize;
//     let mut str_0: &str = "hbWnWpX";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut usize_1: usize = 1704usize;
//     let mut usize_2: usize = 623usize;
//     let mut bool_0: bool = false;
//     let mut usize_3: usize = 8533usize;
//     let mut suffixcachekey_0: crate::compile::SuffixCacheKey = crate::compile::SuffixCacheKey::default();
//     let mut suffixcachekey_0_ref_0: &crate::compile::SuffixCacheKey = &mut suffixcachekey_0;
//     let mut suffixcachekey_1: crate::compile::SuffixCacheKey = crate::compile::SuffixCacheKey::default();
//     let mut suffixcachekey_1_ref_0: &crate::compile::SuffixCacheKey = &mut suffixcachekey_1;
//     let mut str_1: &str = "xx";
//     let mut str_1_ref_0: &str = &mut str_1;
//     let mut str_2: &str = "GQX72";
//     let mut str_2_ref_0: &str = &mut str_2;
//     let mut str_3: &str = "QF7syBkkw";
//     let mut str_3_ref_0: &str = &mut str_3;
//     let mut regexset_1: crate::re_set::bytes::RegexSet = crate::re_set::bytes::RegexSet::empty();
//     let mut regexset_1_ref_0: &crate::re_set::bytes::RegexSet = &mut regexset_1;
//     let mut usize_4: usize = 8043usize;
//     let mut str_4: &str = "brpDHYzt3T4NZgAD09";
//     let mut str_4_ref_0: &str = &mut str_4;
//     let mut ref_0: expand::Ref = crate::expand::Ref::from(str_4_ref_0);
//     let mut captureref_0: crate::expand::CaptureRef = crate::expand::CaptureRef {cap: ref_0, end: usize_4};
//     let mut captureref_0_ref_0: &crate::expand::CaptureRef = &mut captureref_0;
//     let mut execbuilder_0: crate::exec::ExecBuilder = crate::exec::ExecBuilder::new(str_1_ref_0);
//     let mut bool_1: bool = crate::compile::SuffixCacheKey::eq(suffixcachekey_1_ref_0, suffixcachekey_0_ref_0);
//     let mut inst_0: prog::Inst = crate::prog::Inst::Match(usize_1);
//     let mut result_0: std::result::Result<crate::re_bytes::Regex, error::Error> = crate::re_bytes::Regex::from_str(str_0_ref_0);
//     panic!("From RustyUnit with love");
// }
}