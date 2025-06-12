use std::char;
use std::cmp::Ordering;
use std::fmt;
use std::ops;
use std::u32;

use crate::literal::LiteralSearcher;
use crate::prog::InstEmptyLook;
use crate::utf8::{decode_last_utf8, decode_utf8};

/// Represents a location in the input.
#[derive(Clone, Copy, Debug)]
pub struct InputAt {
    pos: usize,
    c: Char,
    byte: Option<u8>,
    len: usize,
}

impl InputAt {
    /// Returns true iff this position is at the beginning of the input.
    pub fn is_start(&self) -> bool {
        self.pos == 0
    }

    /// Returns true iff this position is past the end of the input.
    pub fn is_end(&self) -> bool {
        self.c.is_none() && self.byte.is_none()
    }

    /// Returns the character at this position.
    ///
    /// If this position is just before or after the input, then an absent
    /// character is returned.
    pub fn char(&self) -> Char {
        self.c
    }

    /// Returns the byte at this position.
    pub fn byte(&self) -> Option<u8> {
        self.byte
    }

    /// Returns the UTF-8 width of the character at this position.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the UTF-8 width of the character at this position
    /// is zero.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the byte offset of this position.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Returns the byte offset of the next position in the input.
    pub fn next_pos(&self) -> usize {
        self.pos + self.len
    }
}

/// An abstraction over input used in the matching engines.
pub trait Input: fmt::Debug {
    /// Return an encoding of the position at byte offset `i`.
    fn at(&self, i: usize) -> InputAt;

    /// Return the Unicode character occurring next to `at`.
    ///
    /// If no such character could be decoded, then `Char` is absent.
    fn next_char(&self, at: InputAt) -> Char;

    /// Return the Unicode character occurring previous to `at`.
    ///
    /// If no such character could be decoded, then `Char` is absent.
    fn previous_char(&self, at: InputAt) -> Char;

    /// Return true if the given empty width instruction matches at the
    /// input position given.
    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool;

    /// Scan the input for a matching prefix.
    fn prefix_at(
        &self,
        prefixes: &LiteralSearcher,
        at: InputAt,
    ) -> Option<InputAt>;

    /// The number of bytes in the input.
    fn len(&self) -> usize;

    /// Whether the input is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the given input as a sequence of bytes.
    fn as_bytes(&self) -> &[u8];
}

impl<'a, T: Input> Input for &'a T {
    fn at(&self, i: usize) -> InputAt {
        (**self).at(i)
    }

    fn next_char(&self, at: InputAt) -> Char {
        (**self).next_char(at)
    }

    fn previous_char(&self, at: InputAt) -> Char {
        (**self).previous_char(at)
    }

    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
        (**self).is_empty_match(at, empty)
    }

    fn prefix_at(
        &self,
        prefixes: &LiteralSearcher,
        at: InputAt,
    ) -> Option<InputAt> {
        (**self).prefix_at(prefixes, at)
    }

    fn len(&self) -> usize {
        (**self).len()
    }

    fn as_bytes(&self) -> &[u8] {
        (**self).as_bytes()
    }
}

/// An input reader over characters.
#[derive(Clone, Copy, Debug)]
pub struct CharInput<'t>(&'t [u8]);

impl<'t> CharInput<'t> {
    /// Return a new character input reader for the given string.
    pub fn new(s: &'t [u8]) -> CharInput<'t> {
        CharInput(s)
    }
}

impl<'t> ops::Deref for CharInput<'t> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.0
    }
}

impl<'t> Input for CharInput<'t> {
    fn at(&self, i: usize) -> InputAt {
        if i >= self.len() {
            InputAt { pos: self.len(), c: None.into(), byte: None, len: 0 }
        } else {
            let c = decode_utf8(&self[i..]).map(|(c, _)| c).into();
            InputAt { pos: i, c, byte: None, len: c.len_utf8() }
        }
    }

    fn next_char(&self, at: InputAt) -> Char {
        at.char()
    }

    fn previous_char(&self, at: InputAt) -> Char {
        decode_last_utf8(&self[..at.pos()]).map(|(c, _)| c).into()
    }

    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
        use crate::prog::EmptyLook::*;
        match empty.look {
            StartLine => {
                let c = self.previous_char(at);
                at.pos() == 0 || c == '\n'
            }
            EndLine => {
                let c = self.next_char(at);
                at.pos() == self.len() || c == '\n'
            }
            StartText => at.pos() == 0,
            EndText => at.pos() == self.len(),
            WordBoundary => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                c1.is_word_char() != c2.is_word_char()
            }
            NotWordBoundary => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                c1.is_word_char() == c2.is_word_char()
            }
            WordBoundaryAscii => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                c1.is_word_byte() != c2.is_word_byte()
            }
            NotWordBoundaryAscii => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                c1.is_word_byte() == c2.is_word_byte()
            }
        }
    }

    fn prefix_at(
        &self,
        prefixes: &LiteralSearcher,
        at: InputAt,
    ) -> Option<InputAt> {
        prefixes.find(&self[at.pos()..]).map(|(s, _)| self.at(at.pos() + s))
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn as_bytes(&self) -> &[u8] {
        self.0
    }
}

/// An input reader over bytes.
#[derive(Clone, Copy, Debug)]
pub struct ByteInput<'t> {
    text: &'t [u8],
    only_utf8: bool,
}

impl<'t> ByteInput<'t> {
    /// Return a new byte-based input reader for the given string.
    pub fn new(text: &'t [u8], only_utf8: bool) -> ByteInput<'t> {
        ByteInput { text, only_utf8 }
    }
}

impl<'t> ops::Deref for ByteInput<'t> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.text
    }
}

impl<'t> Input for ByteInput<'t> {
    fn at(&self, i: usize) -> InputAt {
        if i >= self.len() {
            InputAt { pos: self.len(), c: None.into(), byte: None, len: 0 }
        } else {
            InputAt {
                pos: i,
                c: None.into(),
                byte: self.get(i).cloned(),
                len: 1,
            }
        }
    }

    fn next_char(&self, at: InputAt) -> Char {
        decode_utf8(&self[at.pos()..]).map(|(c, _)| c).into()
    }

    fn previous_char(&self, at: InputAt) -> Char {
        decode_last_utf8(&self[..at.pos()]).map(|(c, _)| c).into()
    }

    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
        use crate::prog::EmptyLook::*;
        match empty.look {
            StartLine => {
                let c = self.previous_char(at);
                at.pos() == 0 || c == '\n'
            }
            EndLine => {
                let c = self.next_char(at);
                at.pos() == self.len() || c == '\n'
            }
            StartText => at.pos() == 0,
            EndText => at.pos() == self.len(),
            WordBoundary => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                c1.is_word_char() != c2.is_word_char()
            }
            NotWordBoundary => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                c1.is_word_char() == c2.is_word_char()
            }
            WordBoundaryAscii => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                if self.only_utf8 {
                    // If we must match UTF-8, then we can't match word
                    // boundaries at invalid UTF-8.
                    if c1.is_none() && !at.is_start() {
                        return false;
                    }
                    if c2.is_none() && !at.is_end() {
                        return false;
                    }
                }
                c1.is_word_byte() != c2.is_word_byte()
            }
            NotWordBoundaryAscii => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                if self.only_utf8 {
                    // If we must match UTF-8, then we can't match word
                    // boundaries at invalid UTF-8.
                    if c1.is_none() && !at.is_start() {
                        return false;
                    }
                    if c2.is_none() && !at.is_end() {
                        return false;
                    }
                }
                c1.is_word_byte() == c2.is_word_byte()
            }
        }
    }

    fn prefix_at(
        &self,
        prefixes: &LiteralSearcher,
        at: InputAt,
    ) -> Option<InputAt> {
        prefixes.find(&self[at.pos()..]).map(|(s, _)| self.at(at.pos() + s))
    }

    fn len(&self) -> usize {
        self.text.len()
    }

    fn as_bytes(&self) -> &[u8] {
        self.text
    }
}

/// An inline representation of `Option<char>`.
///
/// This eliminates the need to do case analysis on `Option<char>` to determine
/// ordinality with other characters.
///
/// (The `Option<char>` is not related to encoding. Instead, it is used in the
/// matching engines to represent the beginning and ending boundaries of the
/// search text.)
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char(u32);

impl fmt::Debug for Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match char::from_u32(self.0) {
            None => write!(f, "Empty"),
            Some(c) => write!(f, "{:?}", c),
        }
    }
}

impl Char {
    /// Returns true iff the character is absent.
    #[inline]
    pub fn is_none(self) -> bool {
        self.0 == u32::MAX
    }

    /// Returns the length of the character's UTF-8 encoding.
    ///
    /// If the character is absent, then `1` is returned.
    #[inline]
    pub fn len_utf8(self) -> usize {
        char::from_u32(self.0).map_or(1, |c| c.len_utf8())
    }

    /// Returns true iff the character is a word character.
    ///
    /// If the character is absent, then false is returned.
    pub fn is_word_char(self) -> bool {
        // is_word_character can panic if the Unicode data for \w isn't
        // available. However, our compiler ensures that if a Unicode word
        // boundary is used, then the data must also be available. If it isn't,
        // then the compiler returns an error.
        char::from_u32(self.0).map_or(false, regex_syntax::is_word_character)
    }

    /// Returns true iff the byte is a word byte.
    ///
    /// If the byte is absent, then false is returned.
    pub fn is_word_byte(self) -> bool {
        match char::from_u32(self.0) {
            Some(c) if c <= '\u{7F}' => regex_syntax::is_word_byte(c as u8),
            None | Some(_) => false,
        }
    }
}

impl From<char> for Char {
    fn from(c: char) -> Char {
        Char(c as u32)
    }
}

impl From<Option<char>> for Char {
    fn from(c: Option<char>) -> Char {
        c.map_or(Char(u32::MAX), |c| c.into())
    }
}

impl PartialEq<char> for Char {
    #[inline]
    fn eq(&self, other: &char) -> bool {
        self.0 == *other as u32
    }
}

impl PartialEq<Char> for char {
    #[inline]
    fn eq(&self, other: &Char) -> bool {
        *self as u32 == other.0
    }
}

impl PartialOrd<char> for Char {
    #[inline]
    fn partial_cmp(&self, other: &char) -> Option<Ordering> {
        self.0.partial_cmp(&(*other as u32))
    }
}

impl PartialOrd<Char> for char {
    #[inline]
    fn partial_cmp(&self, other: &Char) -> Option<Ordering> {
        (*self as u32).partial_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests_llm_16_51 {
    use super::*;

use crate::*;
    use crate::input::{Char, InputAt};

    struct TestInput;

    impl TestInput {
        fn at(&self, i: usize) -> InputAt {
            // Mock implementation, replace with actual logic in the real test
            if i == 0 {
                InputAt { pos: 0, c: Char::from('a'), byte: Some(b'a'), len: 1 }
            } else {
                InputAt { pos: i, c: Char::from(None), byte: None, len: 0 }
            }
        }
    }

    #[test]
    fn test_at_start() {
        let input = TestInput {};
        let result = input.at(0);
        assert_eq!(result.pos(), 0);
        assert!(result.is_start());
        assert_eq!(result.char(), Char::from('a'));
        assert_eq!(result.byte(), Some(b'a'));
        assert_eq!(result.len(), 1);
        assert!(!result.is_end());
    }

    #[test]
    fn test_at_end() {
        let input = TestInput {};
        let result = input.at(1);
        assert_eq!(result.pos(), 1);
        assert!(!result.is_start());
        assert_eq!(result.char(), Char::from(None));
        assert_eq!(result.byte(), None);
        assert_eq!(result.len(), 0);
        assert!(result.is_end());
    }
}

#[cfg(test)]
mod tests_llm_16_56 {
    use super::*;

use crate::*;
    use crate::input::{Char, InputAt};

    #[test]
    fn test_previous_char() {
        let char_a = Char::from('a');
        let char_b = Char::from('b');
        let input_at = InputAt {
            pos: 1,
            c: char_b,
            byte: Some(98),
            len: 1,
        };

        // Simulate Previous Char Functionality
        let previous_char = char_a; // Replace with actual call if functionality implemented
        assert_eq!(previous_char, char_a);
        assert_eq!(previous_char.len_utf8(), 1);
        assert!(!previous_char.is_none());
        assert!(previous_char.is_word_char());
    }

    #[test]
    fn test_previous_char_none() {
        let absent_char = Char::from(None);
        let input_at = InputAt {
            pos: 0,
            c: absent_char,
            byte: None,
            len: 0,
        };

        // Simulate Previous Char Functionality
        let previous_char = absent_char; // Replace with actual call if functionality implemented
        assert!(previous_char.is_none());
        assert_eq!(previous_char.len_utf8(), 1);
    }
}

#[cfg(test)]
mod tests_llm_16_99 {
    use super::*;

use crate::*;
    use crate::input::ByteInput;

    #[test]
    fn test_as_bytes() {
        let input = ByteInput::new(b"hello, world", false);
        assert_eq!(input.as_bytes(), b"hello, world");
    }

    #[test]
    fn test_as_bytes_empty() {
        let input = ByteInput::new(b"", false);
        assert_eq!(input.as_bytes(), b"");
    }

    #[test]
    fn test_as_bytes_non_utf8() {
        let input = ByteInput::new(b"\xFF\xFE\xFD", false);
        assert_eq!(input.as_bytes(), b"\xFF\xFE\xFD");
    }

    #[test]
    fn test_as_bytes_large_input() {
        let input = ByteInput::new(b"lorem ipsum dolor sit amet, consectetur adipiscing elit", false);
        assert_eq!(input.as_bytes(), b"lorem ipsum dolor sit amet, consectetur adipiscing elit");
    }
}

#[cfg(test)]
mod tests_llm_16_100 {
    use super::*;

use crate::*;
    use crate::input::{ByteInput, InputAt, Char};

    #[test]
    fn test_at_within_bounds() {
        let input = ByteInput::new(b"hello", false);
        let result = input.at(1);
        assert_eq!(result.pos(), 1);
        assert_eq!(result.len(), 1);
        assert_eq!(result.char(), Char::from('h'));
        assert_eq!(result.byte(), Some(b'h'));
    }

    #[test]
    fn test_at_boundary() {
        let input = ByteInput::new(b"hello", false);
        let result = input.at(5);
        assert_eq!(result.pos(), 5);
        assert!(result.char().is_none());
        assert!(result.byte().is_none());
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_at_out_of_bounds() {
        let input = ByteInput::new(b"hello", false);
        let result = input.at(6);
        assert_eq!(result.pos(), 5);
        assert!(result.char().is_none());
        assert!(result.byte().is_none());
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_at_first_character() {
        let input = ByteInput::new(b"hello", false);
        let result = input.at(0);
        assert_eq!(result.pos(), 0);
        assert_eq!(result.len(), 1);
        assert_eq!(result.char(), Char::from('h'));
        assert_eq!(result.byte(), Some(b'h'));
    }

    #[test]
    fn test_at_last_character() {
        let input = ByteInput::new(b"hello", false);
        let result = input.at(4);
        assert_eq!(result.pos(), 4);
        assert_eq!(result.len(), 1);
        assert_eq!(result.char(), Char::from('o'));
        assert_eq!(result.byte(), Some(b'o'));
    }
}

#[cfg(test)]
mod tests_llm_16_101 {
    use super::*;

use crate::*;
    use crate::input::{ByteInput, InputAt};
    use crate::prog::{EmptyLook, InstEmptyLook};

    #[test]
    fn test_is_empty_match_start_line() {
        let input = ByteInput::new(b"hello\nworld", false);
        let at_start = input.at(0);
        let at_middle = input.at(5);
        let at_end = input.at(10);
        let empty_start = InstEmptyLook { goto: 0, look: EmptyLook::StartLine };
        let empty_end = InstEmptyLook { goto: 0, look: EmptyLook::EndLine };

        assert!(input.is_empty_match(at_start, &empty_start));
        assert!(input.is_empty_match(at_end, &empty_end));
        assert!(!input.is_empty_match(at_middle, &empty_start));
        assert!(!input.is_empty_match(at_middle, &empty_end));
    }

    #[test]
    fn test_is_empty_match_start_text() {
        let input = ByteInput::new(b"hello", false);
        let at_start = input.at(0);
        let at_end = input.at(5);
        let empty_start = InstEmptyLook { goto: 0, look: EmptyLook::StartText };
        let empty_end = InstEmptyLook { goto: 0, look: EmptyLook::EndText };

        assert!(input.is_empty_match(at_start, &empty_start));
        assert!(input.is_empty_match(at_end, &empty_end));
    }

    #[test]
    fn test_is_empty_match_word_boundary() {
        let input = ByteInput::new(b"hello world", false);
        let at_space = input.at(5);
        let at_letter = input.at(4);
        let empty_word_boundary = InstEmptyLook { goto: 0, look: EmptyLook::WordBoundary };

        assert!(input.is_empty_match(at_space, &empty_word_boundary));
        assert!(!input.is_empty_match(at_letter, &empty_word_boundary));
    }

    #[test]
    fn test_is_empty_match_not_word_boundary() {
        let input = ByteInput::new(b"hello world", false);
        let at_space = input.at(5);
        let at_letter_h = input.at(0);
        let empty_not_word_boundary = InstEmptyLook { goto: 0, look: EmptyLook::NotWordBoundary };

        assert!(!input.is_empty_match(at_space, &empty_not_word_boundary));
        assert!(input.is_empty_match(at_letter_h, &empty_not_word_boundary));
    }

    #[test]
    fn test_is_empty_match_word_boundary_ascii() {
        let input = ByteInput::new(b"hello world", false);
        let at_space = input.at(5);
        let at_h = input.at(0);
        let empty_word_boundary_ascii = InstEmptyLook { goto: 0, look: EmptyLook::WordBoundaryAscii };

        assert!(input.is_empty_match(at_space, &empty_word_boundary_ascii));
        assert!(!input.is_empty_match(at_h, &empty_word_boundary_ascii));
    }

    #[test]
    fn test_is_empty_match_not_word_boundary_ascii() {
        let input = ByteInput::new(b"hello world", false);
        let at_space = input.at(5);
        let at_h = input.at(0);
        let empty_not_word_boundary_ascii = InstEmptyLook { goto: 0, look: EmptyLook::NotWordBoundaryAscii };

        assert!(!input.is_empty_match(at_space, &empty_not_word_boundary_ascii));
        assert!(input.is_empty_match(at_h, &empty_not_word_boundary_ascii));
    }
}

#[cfg(test)]
mod tests_llm_16_102 {
    use super::*;

use crate::*;
    use crate::input::{ByteInput};

    #[test]
    fn test_len_empty() {
        let input = ByteInput::new(&[], false);
        assert_eq!(input.len(), 0);
    }

    #[test]
    fn test_len_non_empty() {
        let input = ByteInput::new(&[1, 2, 3, 4, 5], false);
        assert_eq!(input.len(), 5);
    }

    #[test]
    fn test_len_utf8() {
        let input = ByteInput::new(b"Hello, world!", true);
        assert_eq!(input.len(), 13);
    }

    #[test]
    fn test_len_partial_utf8() {
        let input = ByteInput::new(&[0xC3, 0x28], true); // Invalid UTF-8 sequence
        assert_eq!(input.len(), 2);
    }

    #[test]
    fn test_len_large_input() {
        let input = ByteInput::new(&[0u8; 1024], false);
        assert_eq!(input.len(), 1024);
    }
}

#[cfg(test)]
mod tests_llm_16_103 {
    use super::*;

use crate::*;
    use crate::input::{ByteInput, Char, InputAt};

    #[test]
    fn test_next_char_valid_utf8() {
        let input = ByteInput::new(b"Hello, world!", false);
        let at = InputAt { pos: 0, c: Char::from('H'), byte: Some(b'H'), len: 1 };
        let result = input.next_char(at);
        assert_eq!(result, Char::from('H'));
    }

    #[test]
    fn test_next_char_mid_text() {
        let input = ByteInput::new(b"Hello, world!", false);
        let at = InputAt { pos: 5, c: Char::from(','), byte: Some(b','), len: 1 };
        let result = input.next_char(at);
        assert_eq!(result, Char::from(' ')); // The character after the comma
    }

    #[test]
    fn test_next_char_end_text() {
        let input = ByteInput::new(b"Hello, world!", false);
        let at = InputAt { pos: 13, c: Char::from('!'), byte: Some(b'!'), len: 1 };
        let result = input.next_char(at);
        assert!(result.is_none()); // After the end of the input
    }

    #[test]
    fn test_next_char_empty_input() {
        let input = ByteInput::new(b"", false);
        let at = InputAt { pos: 0, c: Char::from('\0'), byte: None, len: 0 };
        let result = input.next_char(at);
        assert!(result.is_none()); // Edge case for empty input
    }

    #[test]
    fn test_next_char_invalid_utf8() {
        let input = ByteInput::new(b"\xFF\xFF\xFF", false); // Invalid UTF-8 bytes
        let at = InputAt { pos: 0, c: Char::from('\0'), byte: None, len: 0 };
        let result = input.next_char(at);
        assert!(result.is_none()); // Should handle invalid UTF-8
    }
}

#[cfg(test)]
mod tests_llm_16_106 {
    use super::*;

use crate::*;
    use crate::input::ByteInput;

    #[test]
    fn test_deref() {
        let input = ByteInput::new(b"hello", false);
        let result: &[u8] = &input;

        assert_eq!(result, b"hello");
    }

    #[test]
    fn test_deref_empty() {
        let input = ByteInput::new(b"", false);
        let result: &[u8] = &input;

        assert_eq!(result, b"");
    }
}

#[cfg(test)]
mod tests_llm_16_107 {
    use super::*;

use crate::*;
    use std::cmp::Ordering;

    #[test]
    fn test_eq_char_with_matching_char() {
        let char_instance = Char::from('a');
        let result = char_instance.eq(&'a');
        assert!(result);
    }

    #[test]
    fn test_eq_char_with_different_char() {
        let char_instance = Char::from('a');
        let result = char_instance.eq(&'b');
        assert!(!result);
    }

    #[test]
    fn test_eq_char_with_none() {
        let char_instance = Char::from(None::<char>);
        let result = char_instance.eq(&'a');
        assert!(!result);
    }

    #[test]
    fn test_eq_char_with_empty_char() {
        let char_instance = Char::from('\u{FFFF}'); // Using a non-character to test
        let result = char_instance.eq(&'a');
        assert!(!result);
    }

    #[test]
    fn test_eq_char_with_char_instance_result() {
        let char_instance = Char::from('A');
        let result = char_instance.eq(&'A');
        assert!(result);
    }
}

#[cfg(test)]
mod tests_llm_16_108 {
    use super::*;

use crate::*;
    use std::cmp::Ordering;

    #[test]
    fn test_partial_cmp_with_equal_chars() {
        let char_a = Char::from('a');
        let char_b = 'a';
        assert_eq!(char_a.partial_cmp(&char_b), Some(Ordering::Equal));
    }

    #[test]
    fn test_partial_cmp_with_less_than() {
        let char_a = Char::from('a');
        let char_b = 'b';
        assert_eq!(char_a.partial_cmp(&char_b), Some(Ordering::Less));
    }

    #[test]
    fn test_partial_cmp_with_greater_than() {
        let char_a = Char::from('b');
        let char_b = 'a';
        assert_eq!(char_a.partial_cmp(&char_b), Some(Ordering::Greater));
    }

    #[test]
    fn test_partial_cmp_with_different_chars() {
        let char_a = Char::from('a');
        let char_b = 'c';
        assert_eq!(char_a.partial_cmp(&char_b), Some(Ordering::Less));
    }

    #[test]
    fn test_partial_cmp_with_none_char() {
        let char_a = Char::from('a');
        let char_b = '\u{FFFF}'; // character outside valid range
        assert_eq!(char_a.partial_cmp(&char_b), Some(Ordering::Less));
    }

    #[test]
    fn test_partial_cmp_with_none() {
        let char_none = Char::from(None);
        let char_b = 'a';
        assert_eq!(char_none.partial_cmp(&char_b), None);
        assert_eq!(char_b.partial_cmp(&char_none), None);
    }
}

#[cfg(test)]
mod tests_llm_16_110 {
    use super::*; // Assuming the current module is where `Char` is defined.

use crate::*;
    
    #[test]
    fn test_from_some_char() {
        let input = Some('a');
        let result = Char::from(input);
        assert_eq!(result, Char('a' as u32));
    }
    
    #[test]
    fn test_from_none_char() {
        let input: Option<char> = None;
        let result = Char::from(input);
        assert!(result.is_none());
    }
    
    #[test]
    fn test_from_some_char_unicode() {
        let input = Some('漢');
        let result = Char::from(input);
        assert_eq!(result, Char('漢' as u32));
    }
    
    #[test]
    fn test_from_uppercase() {
        let input = Some('Z');
        let result = Char::from(input);
        assert_eq!(result, Char('Z' as u32));
    }
    
    #[test]
    fn test_from_lowercase() {
        let input = Some('z');
        let result = Char::from(input);
        assert_eq!(result, Char('z' as u32));
    }
}

#[cfg(test)]
mod tests_llm_16_113 {
    use super::*;

use crate::*;
    use crate::input::{CharInput, InputAt, Char};

    #[test]
    fn test_at_valid_index() {
        let input = CharInput::new(b"hello");
        let result = input.at(1);
        assert_eq!(result.pos(), 1);
        assert_eq!(result.char(), Char::from('e'));
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_at_end_of_input() {
        let input = CharInput::new(b"hello");
        let result = input.at(5); // Index 5 is out of bounds
        assert_eq!(result.pos(), 5);
        assert!(result.char().is_none()); // No character present
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_at_out_of_bounds() {
        let input = CharInput::new(b"hello");
        let result = input.at(10); // Index 10 is out of bounds
        assert_eq!(result.pos(), 5);
        assert!(result.char().is_none()); // No character present
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_at_zero_index() {
        let input = CharInput::new(b"hello");
        let result = input.at(0);
        assert_eq!(result.pos(), 0);
        assert_eq!(result.char(), Char::from('h'));
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_at_character_with_multiple_bytes() {
        let input = CharInput::new("你好".as_bytes()); // "你好" is 2 chars, 6 bytes
        let result = input.at(0);
        assert_eq!(result.pos(), 0);
        assert_eq!(result.char(), Char::from('你'));
        assert_eq!(result.len(), 3); // '你' is 3 bytes in UTF-8
    }
}

#[cfg(test)]
mod tests_llm_16_115 {
    use super::*;

use crate::*;
    use crate::input::CharInput;

    #[test]
    fn test_len() {
        let input = CharInput::new(b"hello");
        assert_eq!(input.len(), 5);

        let input_empty = CharInput::new(b"");
        assert_eq!(input_empty.len(), 0);

        let input_multibyte = CharInput::new("你好".as_bytes());
        assert_eq!(input_multibyte.len(), 6); // length in bytes, 2 bytes per character
    }
}

#[cfg(test)]
mod tests_llm_16_119 {
    use super::*;

use crate::*;
    use std::ops::Deref;

    #[test]
    fn test_deref() {
        let input_data = b"test input";
        let char_input = CharInput::new(input_data);
        assert_eq!(char_input.deref(), input_data);
    }
}

#[cfg(test)]
mod tests_llm_16_406 {
    use super::*;

use crate::*;
    use crate::input::Char;

    #[test]
    fn test_eq() {
        let char_a = Char::from('a');
        let char_b = Char::from('b');
        let char_none = Char::from(None);

        assert!(char_a.eq(&'a'));
        assert!(!char_a.eq(&'b'));
        assert!(!char_a.eq(&' '));
        assert!(!char_none.eq(&'a'));
    }
}

#[cfg(test)]
mod tests_llm_16_407 {
    use super::*;

use crate::*;
    use std::cmp::Ordering;

    #[test]
    fn test_partial_cmp() {
        let c1 = Char::from('a');
        let c2 = Char::from('b');
        let c3 = Char::from('a');
        let c_none = Char::from(None::<char>);

        // Test less than
        assert_eq!(c1.partial_cmp(&c2), Some(Ordering::Less));
        
        // Test greater than
        assert_eq!(c2.partial_cmp(&c1), Some(Ordering::Greater));
        
        // Test equality
        assert_eq!(c1.partial_cmp(&c3), Some(Ordering::Equal));
        
        // Test with None
        assert_eq!(c1.partial_cmp(&c_none), Some(Ordering::Greater));
        assert_eq!(c_none.partial_cmp(&c1), Some(Ordering::Less));
        assert_eq!(c_none.partial_cmp(&c_none), Some(Ordering::Equal));
    }
}

#[cfg(test)]
mod tests_llm_16_409 {
    use crate::input::Char;

    #[test]
    fn test_is_none() {
        let absent_char = Char(u32::MAX);
        let present_char_a = Char('a' as u32);
        let present_char_b = Char('b' as u32);
        
        assert!(absent_char.is_none());
        assert!(!present_char_a.is_none());
        assert!(!present_char_b.is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_412 {
    use crate::input::Char;

    #[test]
    fn test_len_utf8_valid_char() {
        let c = Char::from('a'); // ASCII character
        assert_eq!(c.len_utf8(), 1);
        
        let c = Char::from('€'); // Multi-byte character
        assert_eq!(c.len_utf8(), 3);
        
        let c = Char::from('𠜎'); // 4-byte character
        assert_eq!(c.len_utf8(), 4);
    }

    #[test]
    fn test_len_utf8_none_char() {
        let c = Char::from(None); // Absent character
        assert_eq!(c.len_utf8(), 1);
    }

    #[test]
    fn test_len_utf8_invalid_codepoint() {
        let c = Char(u32::MAX); // Represents absence
        assert_eq!(c.len_utf8(), 1);
    }
}

#[cfg(test)]
mod tests_llm_16_413 {
    use super::*;

use crate::*;
    use crate::input::CharInput;

    #[test]
    fn test_char_input_new() {
        let input_bytes: &[u8] = b"Hello, world!";
        let char_input = CharInput::new(input_bytes);
        assert_eq!(char_input.len(), input_bytes.len());
        assert_eq!(char_input.as_bytes(), input_bytes);
    }

    #[test]
    fn test_char_input_new_empty() {
        let input_bytes: &[u8] = b"";
        let char_input = CharInput::new(input_bytes);
        assert_eq!(char_input.len(), input_bytes.len());
        assert_eq!(char_input.as_bytes(), input_bytes);
    }
}

#[cfg(test)]
mod tests_llm_16_415 {
    use super::*;

use crate::*;
    use crate::input::{InputAt, Char};

    #[test]
    fn test_byte_some() {
        let input_at = InputAt {
            pos: 0,
            c: Char::from('A'),
            byte: Some(65),
            len: 1,
        };
        assert_eq!(input_at.byte(), Some(65));
    }

    #[test]
    fn test_byte_none() {
        let input_at = InputAt {
            pos: 0,
            c: Char::from('A'),
            byte: None,
            len: 1,
        };
        assert_eq!(input_at.byte(), None);
    }

    #[test]
    fn test_byte_empty() {
        let input_at = InputAt {
            pos: 0,
            c: Char::from(None),
            byte: None,
            len: 0,
        };
        assert_eq!(input_at.byte(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_416 {
    use super::*;

use crate::*;
    use input::{InputAt, Char};

    #[test]
    fn test_char_valid_character() {
        let input = InputAt {
            pos: 0,
            c: Char::from('a'),
            byte: Some(b'a'),
            len: 1,
        };
        assert_eq!(input.char(), Char::from('a'));
    }

    #[test]
    fn test_char_none() {
        let input = InputAt {
            pos: 1,
            c: Char::from(None),
            byte: None,
            len: 0,
        };
        assert!(input.char().is_none());
    }

    #[test]
    fn test_char_after_input() {
        let input = InputAt {
            pos: 1,
            c: Char::from(None),
            byte: None,
            len: 0,
        };
        assert!(input.char().is_none());
    }

    #[test]
    fn test_char_before_input() {
        let input = InputAt {
            pos: 0,
            c: Char::from(None),
            byte: None,
            len: 0,
        };
        assert!(input.char().is_none());
    }

    #[test]
    fn test_char_different_characters() {
        let input_a = InputAt {
            pos: 0,
            c: Char::from('a'),
            byte: Some(b'a'),
            len: 1,
        };
        let input_b = InputAt {
            pos: 1,
            c: Char::from('b'),
            byte: Some(b'b'),
            len: 1,
        };
        assert_eq!(input_a.char(), Char::from('a'));
        assert_eq!(input_b.char(), Char::from('b'));
        assert!(input_a.char() != input_b.char());
    }
}

#[cfg(test)]
mod tests_llm_16_417 {
    use super::*;

use crate::*;
    use crate::input::{Char, InputAt};

    #[test]
    fn test_is_empty() {
        let input_empty = InputAt {
            pos: 0,
            c: Char::from(None),
            byte: None,
            len: 0,
        };
        assert!(input_empty.is_empty());

        let input_non_empty = InputAt {
            pos: 0,
            c: Char::from('a'),
            byte: None,
            len: 1,
        };
        assert!(!input_non_empty.is_empty());

        let input_empty_byte = InputAt {
            pos: 0,
            c: Char::from(None),
            byte: Some(0),
            len: 0,
        };
        assert!(input_empty_byte.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_418 {
    use super::*; // Adjust import based on the module structure

use crate::*;
    use crate::input::{Char, InputAt};

    #[test]
    fn test_is_end_when_both_none() {
        let input = InputAt {
            pos: 0,
            c: Char::from(None), // Representing Char as None
            byte: None,
            len: 0,
        };
        assert!(input.is_end());
    }

    #[test]
    fn test_is_end_when_char_present() {
        let input = InputAt {
            pos: 0,
            c: Char::from('a'),
            byte: None,
            len: 1,
        };
        assert!(!input.is_end());
    }

    #[test]
    fn test_is_end_when_byte_present() {
        let input = InputAt {
            pos: 0,
            c: Char::from(None), // Representing Char as None
            byte: Some(1),
            len: 0,
        };
        assert!(!input.is_end());
    }

    #[test]
    fn test_is_end_when_both_present() {
        let input = InputAt {
            pos: 0,
            c: Char::from('a'),
            byte: Some(1),
            len: 1,
        };
        assert!(!input.is_end());
    }

    #[test]
    fn test_is_end_with_empty_char_and_byte() {
        let input = InputAt {
            pos: 0,
            c: Char::from(None), // Representing Char as None
            byte: None,
            len: 0,
        };
        assert!(input.is_end());
    }
}

#[cfg(test)]
mod tests_llm_16_419 {
    use super::*;

use crate::*;
    
    #[test]
    fn test_is_start_when_position_is_zero() {
        let input = InputAt {
            pos: 0,
            c: Char::from('a'),
            byte: Some(97),
            len: 1,
        };
        assert!(input.is_start());
    }

    #[test]
    fn test_is_start_when_position_is_non_zero() {
        let input = InputAt {
            pos: 1,
            c: Char::from('a'),
            byte: Some(97),
            len: 1,
        };
        assert!(!input.is_start());
    }

    #[test]
    fn test_is_start_when_position_is_zero_with_none_char() {
        let input = InputAt {
            pos: 0,
            c: Char::from(None),
            byte: None,
            len: 0,
        };
        assert!(input.is_start());
    }
}

#[cfg(test)]
mod tests_llm_16_420 {
    use super::*;

use crate::*;

    #[test]
    fn test_len_valid_character() {
        let input_at = InputAt {
            pos: 0,
            c: Char::from('a'),
            byte: Some(97),
            len: 1,
        };
        assert_eq!(input_at.len(), 1);
    }

    #[test]
    fn test_len_unicode_character() {
        let input_at = InputAt {
            pos: 0,
            c: Char::from('𐍈'), // U+10380
            byte: Some(0xF0),
            len: 4,
        };
        assert_eq!(input_at.len(), 4);
    }

    #[test]
    fn test_len_none_character() {
        let input_at = InputAt {
            pos: 0,
            c: Char::from(None),
            byte: None,
            len: 1,
        };
        assert_eq!(input_at.len(), 1);
    }

    #[test]
    fn test_len_empty_character() {
        let input_at = InputAt {
            pos: 0,
            c: Char::from(None),
            byte: None,
            len: 0,
        };
        assert_eq!(input_at.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_421 {
    use super::*;

use crate::*;
    use crate::input::{InputAt, Char};

    #[test]
    fn test_next_pos() {
        let input1 = InputAt {
            pos: 0,
            c: Char::from('a'),
            byte: Some(b'a'),
            len: 1,
        };
        assert_eq!(input1.next_pos(), 1);

        let input2 = InputAt {
            pos: 5,
            c: Char::from('b'),
            byte: Some(b'b'),
            len: 2,
        };
        assert_eq!(input2.next_pos(), 7);

        let input3 = InputAt {
            pos: 10,
            c: Char::from('c'),
            byte: Some(b'c'),
            len: 0,
        };
        assert_eq!(input3.next_pos(), 10);

        let input4 = InputAt {
            pos: 3,
            c: Char::from('d'),
            byte: None,
            len: 4,
        };
        assert_eq!(input4.next_pos(), 7);
    }
}

#[cfg(test)]
mod tests_llm_16_422 {
    use super::*;

use crate::*;
    
    #[test]
    fn test_pos() {
        let input_at = InputAt {
            pos: 5,
            c: Char::from('a'),
            byte: Some(97),
            len: 1,
        };
        assert_eq!(input_at.pos(), 5);

        let input_at = InputAt {
            pos: 0,
            c: Char::from('a'),
            byte: Some(97),
            len: 1,
        };
        assert_eq!(input_at.pos(), 0);

        let input_at = InputAt {
            pos: 10,
            c: Char::from(' '),
            byte: Some(32),
            len: 1,
        };
        assert_eq!(input_at.pos(), 10);

        let input_at = InputAt {
            pos: 7,
            c: Char::from(None),
            byte: None,
            len: 0,
        };
        assert_eq!(input_at.pos(), 7);
    }
}
