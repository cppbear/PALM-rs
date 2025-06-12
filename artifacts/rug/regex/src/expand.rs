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

/// Returns true if and only if the given byte is allowed in a capture name
/// written in non-brace form.
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
    find!(find_cap_ref20, "${¾}", c!("¾", 5));
    find!(find_cap_ref21, "${¾a}", c!("¾a", 6));
    find!(find_cap_ref22, "${a¾}", c!("a¾", 6));
    find!(find_cap_ref23, "${☃}", c!("☃", 6));
    find!(find_cap_ref24, "${a☃}", c!("a☃", 7));
    find!(find_cap_ref25, "${☃a}", c!("☃a", 7));
    find!(find_cap_ref26, "${名字}", c!("名字", 9));
}

#[cfg(test)]
mod tests_llm_16_97 {
    use super::*; // Ensure the module where `from` is defined is accessible

use crate::*;
    use crate::expand::Ref; // Adjust the import path as necessary

    #[test]
    fn test_from_str() {
        let input = "foo";
        let result = Ref::from(input);
        
        match result {
            Ref::Named(s) => assert_eq!(s, input),
            _ => panic!("Expected Ref::Named variant"),
        }
    }

    #[test]
    fn test_from_empty_str() {
        let input = "";
        let result = Ref::from(input);
        
        match result {
            Ref::Named(s) => assert_eq!(s, input),
            _ => panic!("Expected Ref::Named variant"),
        }
    }
}

#[cfg(test)]
mod tests_llm_16_98 {
    use super::*;

use crate::*;
    use crate::expand::Ref;

    #[test]
    fn test_from_usize() {
        let num: usize = 5;
        let result: Ref<'static> = Ref::from(num);
        match result {
            Ref::Number(value) => assert_eq!(value, 5),
            _ => panic!("Expected a Ref::Number variant"),
        }
    }
}

#[cfg(test)]
mod tests_llm_16_400 {
    use super::*;

use crate::*;
    use re_unicode::Captures;
    use std::collections::HashMap;

    #[test]
    fn test_expand_str_with_named_capture() {
        let text = "Hello, my name is Alice.";
        let re = Regex::new(r"(Hello), my name is (?P<name>\w+).").unwrap();
        let caps = re.captures(text).unwrap();

        let mut result = String::new();
        expand_str(&caps, "greetings: $name!", &mut result);
        assert_eq!(result, "greetings: Alice!");
    }

    #[test]
    fn test_expand_str_with_indexed_capture() {
        let text = "The quick brown fox.";
        let re = Regex::new(r"(quick) (brown)").unwrap();
        let caps = re.captures(text).unwrap();

        let mut result = String::new();
        expand_str(&caps, "1: $1, 2: $2.", &mut result);
        assert_eq!(result, "1: quick, 2: brown.");
    }

    #[test]
    fn test_expand_str_with_no_capture() {
        let text = "The rain in Spain.";
        let re = Regex::new(r"(rain)").unwrap();
        let caps = re.captures(text).unwrap();

        let mut result = String::new();
        expand_str(&caps, "Nothing matched here: $2.", &mut result);
        assert_eq!(result, "Nothing matched here: .");
    }

    #[test]
    fn test_expand_str_with_escaped_dollars() {
        let text = "The cost is $100.";
        let re = Regex::new(r"(cost) is \$(\d+).").unwrap();
        let caps = re.captures(text).unwrap();

        let mut result = String::new();
        expand_str(&caps, "Price: $$ $2!", &mut result);
        assert_eq!(result, "Price: $ 100!");
    }

    #[test]
    fn test_expand_str_with_empty_replacement() {
        let text = "No captures here.";
        let re = Regex::new(r"(captures)").unwrap();
        let caps = re.captures(text).unwrap();

        let mut result = String::new();
        expand_str(&caps, "", &mut result);
        assert_eq!(result, "");
    }
}

#[cfg(test)]
mod tests_llm_16_401 {
    use super::*;

use crate::*;
    use crate::expand::find_cap_ref;
    use crate::expand::CaptureRef; // Adjust based on actual module path

    #[test]
    fn test_find_cap_ref_valid_number() {
        let replacement = b"$1";
        let result = find_cap_ref(replacement);
        assert!(result.is_some());
        if let Some(CaptureRef { cap, end }) = result {
            assert_eq!(cap, Ref::Number(1));
            assert_eq!(end, 2);
        }
    }

    #[test]
    fn test_find_cap_ref_valid_named() {
        let replacement = b"${name}";
        let result = find_cap_ref(replacement);
        assert!(result.is_some());
        if let Some(CaptureRef { cap, end }) = result {
            assert_eq!(cap, Ref::Named("name"));
            assert_eq!(end, 6);
        }
    }

    #[test]
    fn test_find_cap_ref_invalid() {
        let replacement = b"$";
        let result = find_cap_ref(replacement);
        assert!(result.is_none());
    }

    #[test]
    fn test_find_cap_ref_invalid_number() {
        let replacement = b"$a";
        let result = find_cap_ref(replacement);
        assert!(result.is_none());
    }

    #[test]
    fn test_find_cap_ref_empty() {
        let replacement = b"";
        let result = find_cap_ref(replacement);
        assert!(result.is_none());
    }

    #[test]
    fn test_find_cap_ref_braced() {
        let replacement = b"${var}";
        let result = find_cap_ref(replacement);
        assert!(result.is_some());
        if let Some(CaptureRef { cap, end }) = result {
            assert_eq!(cap, Ref::Named("var"));
            assert_eq!(end, 6);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_403 {
    use crate::expand::is_valid_cap_letter;

    #[test]
    fn test_valid_cap_letter() {
        // Testing valid lowercase letters
        assert!(is_valid_cap_letter(b'a'));
        assert!(is_valid_cap_letter(b'z'));
        
        // Testing valid uppercase letters
        assert!(is_valid_cap_letter(b'A'));
        assert!(is_valid_cap_letter(b'Z'));

        // Testing valid digits
        assert!(is_valid_cap_letter(b'0'));
        assert!(is_valid_cap_letter(b'9'));

        // Testing valid underscore
        assert!(is_valid_cap_letter(b'_'));
    }

    #[test]
    fn test_invalid_cap_letter() {
        // Testing invalid characters
        assert!(!is_valid_cap_letter(b'!'));
        assert!(!is_valid_cap_letter(b'@'));
        assert!(!is_valid_cap_letter(b'#'));
        assert!(!is_valid_cap_letter(b'$'));
        assert!(!is_valid_cap_letter(b'%'));
        assert!(!is_valid_cap_letter(b'^'));
        assert!(!is_valid_cap_letter(b'&'));
        assert!(!is_valid_cap_letter(b'*'));
        assert!(!is_valid_cap_letter(b'('));
        assert!(!is_valid_cap_letter(b')'));
        assert!(!is_valid_cap_letter(b'-'));
        assert!(!is_valid_cap_letter(b'+'));
        assert!(!is_valid_cap_letter(b'{'));
        assert!(!is_valid_cap_letter(b'}'));
        assert!(!is_valid_cap_letter(b'['));
        assert!(!is_valid_cap_letter(b']'));
        assert!(!is_valid_cap_letter(b'|'));
        assert!(!is_valid_cap_letter(b':'));
        assert!(!is_valid_cap_letter(b';'));
        assert!(!is_valid_cap_letter(b'"'));
        assert!(!is_valid_cap_letter(b'\''));
        assert!(!is_valid_cap_letter(b'<'));
        assert!(!is_valid_cap_letter(b'>'));
        assert!(!is_valid_cap_letter(b','));
        assert!(!is_valid_cap_letter(b'.'));
        assert!(!is_valid_cap_letter(b'?'));
        assert!(!is_valid_cap_letter(b'/'));
    }
}
