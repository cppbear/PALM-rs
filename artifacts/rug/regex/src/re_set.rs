macro_rules! define_set {
    ($name:ident, $builder_mod:ident, $text_ty:ty, $as_bytes:expr,
     $(#[$doc_regexset_example:meta])* ) => {
        pub mod $name {
            use std::fmt;
            use std::iter;
            use std::slice;
            use std::vec;

            use crate::error::Error;
            use crate::exec::Exec;
            use crate::re_builder::$builder_mod::RegexSetBuilder;
            use crate::re_trait::RegularExpression;

/// Match multiple (possibly overlapping) regular expressions in a single scan.
///
/// A regex set corresponds to the union of two or more regular expressions.
/// That is, a regex set will match text where at least one of its
/// constituent regular expressions matches. A regex set as its formulated here
/// provides a touch more power: it will also report *which* regular
/// expressions in the set match. Indeed, this is the key difference between
/// regex sets and a single `Regex` with many alternates, since only one
/// alternate can match at a time.
///
/// For example, consider regular expressions to match email addresses and
/// domains: `[a-z]+@[a-z]+\.(com|org|net)` and `[a-z]+\.(com|org|net)`. If a
/// regex set is constructed from those regexes, then searching the text
/// `foo@example.com` will report both regexes as matching. Of course, one
/// could accomplish this by compiling each regex on its own and doing two
/// searches over the text. The key advantage of using a regex set is that it
/// will report the matching regexes using a *single pass through the text*.
/// If one has hundreds or thousands of regexes to match repeatedly (like a URL
/// router for a complex web application or a user agent matcher), then a regex
/// set can realize huge performance gains.
///
/// # Example
///
/// This shows how the above two regexes (for matching email addresses and
/// domains) might work:
///
$(#[$doc_regexset_example])*
///
/// Note that it would be possible to adapt the above example to using `Regex`
/// with an expression like:
///
/// ```text
/// (?P<email>[a-z]+@(?P<email_domain>[a-z]+[.](com|org|net)))|(?P<domain>[a-z]+[.](com|org|net))
/// ```
///
/// After a match, one could then inspect the capture groups to figure out
/// which alternates matched. The problem is that it is hard to make this
/// approach scale when there are many regexes since the overlap between each
/// alternate isn't always obvious to reason about.
///
/// # Limitations
///
/// Regex sets are limited to answering the following two questions:
///
/// 1. Does any regex in the set match?
/// 2. If so, which regexes in the set match?
///
/// As with the main [`Regex`][crate::Regex] type, it is cheaper to ask (1)
/// instead of (2) since the matching engines can stop after the first match
/// is found.
///
/// You cannot directly extract [`Match`][crate::Match] or
/// [`Captures`][crate::Captures] objects from a regex set. If you need these
/// operations, the recommended approach is to compile each pattern in the set
/// independently and scan the exact same input a second time with those
/// independently compiled patterns:
///
/// ```rust
/// use regex::{Regex, RegexSet};
///
/// let patterns = ["foo", "bar"];
/// // Both patterns will match different ranges of this string.
/// let text = "barfoo";
///
/// // Compile a set matching any of our patterns.
/// let set = RegexSet::new(&patterns).unwrap();
/// // Compile each pattern independently.
/// let regexes: Vec<_> = set.patterns().iter()
///     .map(|pat| Regex::new(pat).unwrap())
///     .collect();
///
/// // Match against the whole set first and identify the individual
/// // matching patterns.
/// let matches: Vec<&str> = set.matches(text).into_iter()
///     // Dereference the match index to get the corresponding
///     // compiled pattern.
///     .map(|match_idx| &regexes[match_idx])
///     // To get match locations or any other info, we then have to search
///     // the exact same text again, using our separately-compiled pattern.
///     .map(|pat| pat.find(text).unwrap().as_str())
///     .collect();
///
/// // Matches arrive in the order the constituent patterns were declared,
/// // not the order they appear in the input.
/// assert_eq!(vec!["foo", "bar"], matches);
/// ```
///
/// # Performance
///
/// A `RegexSet` has the same performance characteristics as `Regex`. Namely,
/// search takes `O(mn)` time, where `m` is proportional to the size of the
/// regex set and `n` is proportional to the length of the search text.
#[derive(Clone)]
pub struct RegexSet(Exec);

impl RegexSet {
    /// Create a new regex set with the given regular expressions.
    ///
    /// This takes an iterator of `S`, where `S` is something that can produce
    /// a `&str`. If any of the strings in the iterator are not valid regular
    /// expressions, then an error is returned.
    ///
    /// # Example
    ///
    /// Create a new regex set from an iterator of strings:
    ///
    /// ```rust
    /// # use regex::RegexSet;
    /// let set = RegexSet::new(&[r"\w+", r"\d+"]).unwrap();
    /// assert!(set.is_match("foo"));
    /// ```
    pub fn new<I, S>(exprs: I) -> Result<RegexSet, Error>
            where S: AsRef<str>, I: IntoIterator<Item=S> {
        RegexSetBuilder::new(exprs).build()
    }

    /// Create a new empty regex set.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use regex::RegexSet;
    /// let set = RegexSet::empty();
    /// assert!(set.is_empty());
    /// ```
    pub fn empty() -> RegexSet {
        RegexSetBuilder::new(&[""; 0]).build().unwrap()
    }

    /// Returns true if and only if one of the regexes in this set matches
    /// the text given.
    ///
    /// This method should be preferred if you only need to test whether any
    /// of the regexes in the set should match, but don't care about *which*
    /// regexes matched. This is because the underlying matching engine will
    /// quit immediately after seeing the first match instead of continuing to
    /// find all matches.
    ///
    /// Note that as with searches using `Regex`, the expression is unanchored
    /// by default. That is, if the regex does not start with `^` or `\A`, or
    /// end with `$` or `\z`, then it is permitted to match anywhere in the
    /// text.
    ///
    /// # Example
    ///
    /// Tests whether a set matches some text:
    ///
    /// ```rust
    /// # use regex::RegexSet;
    /// let set = RegexSet::new(&[r"\w+", r"\d+"]).unwrap();
    /// assert!(set.is_match("foo"));
    /// assert!(!set.is_match("☃"));
    /// ```
    pub fn is_match(&self, text: $text_ty) -> bool {
        self.is_match_at(text, 0)
    }

    /// Returns the same as is_match, but starts the search at the given
    /// offset.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only
    /// match when `start == 0`.
    #[doc(hidden)]
    pub fn is_match_at(&self, text: $text_ty, start: usize) -> bool {
        self.0.searcher().is_match_at($as_bytes(text), start)
    }

    /// Returns the set of regular expressions that match in the given text.
    ///
    /// The set returned contains the index of each regular expression that
    /// matches in the given text. The index is in correspondence with the
    /// order of regular expressions given to `RegexSet`'s constructor.
    ///
    /// The set can also be used to iterate over the matched indices.
    ///
    /// Note that as with searches using `Regex`, the expression is unanchored
    /// by default. That is, if the regex does not start with `^` or `\A`, or
    /// end with `$` or `\z`, then it is permitted to match anywhere in the
    /// text.
    ///
    /// # Example
    ///
    /// Tests which regular expressions match the given text:
    ///
    /// ```rust
    /// # use regex::RegexSet;
    /// let set = RegexSet::new(&[
    ///     r"\w+",
    ///     r"\d+",
    ///     r"\pL+",
    ///     r"foo",
    ///     r"bar",
    ///     r"barfoo",
    ///     r"foobar",
    /// ]).unwrap();
    /// let matches: Vec<_> = set.matches("foobar").into_iter().collect();
    /// assert_eq!(matches, vec![0, 2, 3, 4, 6]);
    ///
    /// // You can also test whether a particular regex matched:
    /// let matches = set.matches("foobar");
    /// assert!(!matches.matched(5));
    /// assert!(matches.matched(6));
    /// ```
    pub fn matches(&self, text: $text_ty) -> SetMatches {
        let mut matches = vec![false; self.0.regex_strings().len()];
        let any = self.read_matches_at(&mut matches, text, 0);
        SetMatches {
            matched_any: any,
            matches: matches,
        }
    }

    /// Returns the same as matches, but starts the search at the given
    /// offset and stores the matches into the slice given.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only
    /// match when `start == 0`.
    ///
    /// `matches` must have a length that is at least the number of regexes
    /// in this set.
    ///
    /// This method returns true if and only if at least one member of
    /// `matches` is true after executing the set against `text`.
    #[doc(hidden)]
    pub fn read_matches_at(
        &self,
        matches: &mut [bool],
        text: $text_ty,
        start: usize,
    ) -> bool {
        self.0.searcher().many_matches_at(matches, $as_bytes(text), start)
    }

    /// Returns the total number of regular expressions in this set.
    pub fn len(&self) -> usize {
        self.0.regex_strings().len()
    }

    /// Returns `true` if this set contains no regular expressions.
    pub fn is_empty(&self) -> bool {
        self.0.regex_strings().is_empty()
    }

    /// Returns the patterns that this set will match on.
    ///
    /// This function can be used to determine the pattern for a match. The
    /// slice returned has exactly as many patterns givens to this regex set,
    /// and the order of the slice is the same as the order of the patterns
    /// provided to the set.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use regex::RegexSet;
    /// let set = RegexSet::new(&[
    ///     r"\w+",
    ///     r"\d+",
    ///     r"\pL+",
    ///     r"foo",
    ///     r"bar",
    ///     r"barfoo",
    ///     r"foobar",
    /// ]).unwrap();
    /// let matches: Vec<_> = set
    ///     .matches("foobar")
    ///     .into_iter()
    ///     .map(|match_idx| &set.patterns()[match_idx])
    ///     .collect();
    /// assert_eq!(matches, vec![r"\w+", r"\pL+", r"foo", r"bar", r"foobar"]);
    /// ```
    pub fn patterns(&self) -> &[String] {
        self.0.regex_strings()
    }
}

impl Default for RegexSet {
    fn default() -> Self {
        RegexSet::empty()
    }
}

/// A set of matches returned by a regex set.
#[derive(Clone, Debug)]
pub struct SetMatches {
    matched_any: bool,
    matches: Vec<bool>,
}

impl SetMatches {
    /// Whether this set contains any matches.
    pub fn matched_any(&self) -> bool {
        self.matched_any
    }

    /// Whether the regex at the given index matched.
    ///
    /// The index for a regex is determined by its insertion order upon the
    /// initial construction of a `RegexSet`, starting at `0`.
    ///
    /// # Panics
    ///
    /// If `regex_index` is greater than or equal to `self.len()`.
    pub fn matched(&self, regex_index: usize) -> bool {
        self.matches[regex_index]
    }

    /// The total number of regexes in the set that created these matches.
    ///
    /// **WARNING:** This always returns the same value as [`RegexSet::len`].
    /// In particular, it does *not* return the number of elements yielded by
    /// [`SetMatches::iter`]. The only way to determine the total number of
    /// matched regexes is to iterate over them.
    pub fn len(&self) -> usize {
        self.matches.len()
    }

    /// Returns an iterator over indexes in the regex that matched.
    ///
    /// This will always produces matches in ascending order of index, where
    /// the index corresponds to the index of the regex that matched with
    /// respect to its position when initially building the set.
    pub fn iter(&self) -> SetMatchesIter<'_> {
        SetMatchesIter((&*self.matches).into_iter().enumerate())
    }
}

impl IntoIterator for SetMatches {
    type IntoIter = SetMatchesIntoIter;
    type Item = usize;

    fn into_iter(self) -> Self::IntoIter {
        SetMatchesIntoIter(self.matches.into_iter().enumerate())
    }
}

impl<'a> IntoIterator for &'a SetMatches {
    type IntoIter = SetMatchesIter<'a>;
    type Item = usize;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An owned iterator over the set of matches from a regex set.
///
/// This will always produces matches in ascending order of index, where the
/// index corresponds to the index of the regex that matched with respect to
/// its position when initially building the set.
#[derive(Debug)]
pub struct SetMatchesIntoIter(iter::Enumerate<vec::IntoIter<bool>>);

impl Iterator for SetMatchesIntoIter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        loop {
            match self.0.next() {
                None => return None,
                Some((_, false)) => {}
                Some((i, true)) => return Some(i),
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for SetMatchesIntoIter {
    fn next_back(&mut self) -> Option<usize> {
        loop {
            match self.0.next_back() {
                None => return None,
                Some((_, false)) => {}
                Some((i, true)) => return Some(i),
            }
        }
    }
}

impl iter::FusedIterator for SetMatchesIntoIter {}

/// A borrowed iterator over the set of matches from a regex set.
///
/// The lifetime `'a` refers to the lifetime of a `SetMatches` value.
///
/// This will always produces matches in ascending order of index, where the
/// index corresponds to the index of the regex that matched with respect to
/// its position when initially building the set.
#[derive(Clone, Debug)]
pub struct SetMatchesIter<'a>(iter::Enumerate<slice::Iter<'a, bool>>);

impl<'a> Iterator for SetMatchesIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        loop {
            match self.0.next() {
                None => return None,
                Some((_, &false)) => {}
                Some((i, &true)) => return Some(i),
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> DoubleEndedIterator for SetMatchesIter<'a> {
    fn next_back(&mut self) -> Option<usize> {
        loop {
            match self.0.next_back() {
                None => return None,
                Some((_, &false)) => {}
                Some((i, &true)) => return Some(i),
            }
        }
    }
}

impl<'a> iter::FusedIterator for SetMatchesIter<'a> {}

#[doc(hidden)]
impl From<Exec> for RegexSet {
    fn from(exec: Exec) -> Self {
        RegexSet(exec)
    }
}

impl fmt::Debug for RegexSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RegexSet({:?})", self.0.regex_strings())
    }
}

#[allow(dead_code)] fn as_bytes_str(text: &str) -> &[u8] { text.as_bytes() }
#[allow(dead_code)] fn as_bytes_bytes(text: &[u8]) -> &[u8] { text }
        }
    }
}

define_set! {
    unicode,
    set_unicode,
    &str,
    as_bytes_str,
/// ```rust
/// # use regex::RegexSet;
/// let set = RegexSet::new(&[
///     r"[a-z]+@[a-z]+\.(com|org|net)",
///     r"[a-z]+\.(com|org|net)",
/// ]).unwrap();
///
/// // Ask whether any regexes in the set match.
/// assert!(set.is_match("foo@example.com"));
///
/// // Identify which regexes in the set match.
/// let matches: Vec<_> = set.matches("foo@example.com").into_iter().collect();
/// assert_eq!(vec![0, 1], matches);
///
/// // Try again, but with text that only matches one of the regexes.
/// let matches: Vec<_> = set.matches("example.com").into_iter().collect();
/// assert_eq!(vec![1], matches);
///
/// // Try again, but with text that doesn't match any regex in the set.
/// let matches: Vec<_> = set.matches("example").into_iter().collect();
/// assert!(matches.is_empty());
/// ```
}

define_set! {
    bytes,
    set_bytes,
    &[u8],
    as_bytes_bytes,
/// ```rust
/// # use regex::bytes::RegexSet;
/// let set = RegexSet::new(&[
///     r"[a-z]+@[a-z]+\.(com|org|net)",
///     r"[a-z]+\.(com|org|net)",
/// ]).unwrap();
///
/// // Ask whether any regexes in the set match.
/// assert!(set.is_match(b"foo@example.com"));
///
/// // Identify which regexes in the set match.
/// let matches: Vec<_> = set.matches(b"foo@example.com").into_iter().collect();
/// assert_eq!(vec![0, 1], matches);
///
/// // Try again, but with text that only matches one of the regexes.
/// let matches: Vec<_> = set.matches(b"example.com").into_iter().collect();
/// assert_eq!(vec![1], matches);
///
/// // Try again, but with text that doesn't match any regex in the set.
/// let matches: Vec<_> = set.matches(b"example").into_iter().collect();
/// assert!(matches.is_empty());
/// ```
}

#[cfg(test)]
mod tests_llm_16_153 {
    use super::*;

use crate::*;
    use re_set::bytes::RegexSet;

    #[test]
    fn test_regex_set_default() {
        let default_set = RegexSet::default();
        assert!(default_set.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_163 {
    use super::*;

use crate::*;
    use crate::re_set::unicode::RegexSet;

    #[test]
    fn test_regex_set_default() {
        let default_set: RegexSet = Default::default();
        assert!(default_set.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_573 {
    use super::*;

use crate::*;
    use crate::RegexSet;

    #[test]
    fn test_empty_regex_set() {
        let set = RegexSet::empty();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_574 {
    use super::*;

use crate::*;
    use re_set::bytes::RegexSet;

    #[test]
    fn test_is_empty() {
        let empty_set = RegexSet::empty();
        assert!(empty_set.is_empty());

        let non_empty_set = RegexSet::new(&["a", "b"]).unwrap();
        assert!(!non_empty_set.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_575 {
    use super::*;

use crate::*;
    use crate::RegexSet;

    #[test]
    fn test_is_match() {
        let set = RegexSet::new(&[r"\w+", r"\d+"]).unwrap();
        assert!(set.is_match("foo"));
        assert!(set.is_match("123"));
        assert!(set.is_match("foo123"));
        assert!(!set.is_match("☃"));
        assert!(!set.is_match(" "));
    }

    #[test]
    fn test_is_match_empty_set() {
        let set = RegexSet::empty();
        assert!(!set.is_match("foo"));
        assert!(!set.is_match("123"));
    }

    #[test]
    fn test_is_match_with_special_characters() {
        let set = RegexSet::new(&[r"\w+", r"\d+"]).unwrap();
        assert!(set.is_match("test@123"));
        assert!(!set.is_match("!@#$%^&*()"));
    }
}

#[cfg(test)]
mod tests_llm_16_576 {
    use super::*;

use crate::*;
    use crate::RegexSet;

    #[test]
    fn test_is_match_at() {
        let patterns = vec![r"\w+", r"\d+", r"foo"];
        let set = RegexSet::new(&patterns).unwrap();

        assert!(set.is_match_at("foo bar 123", 0)); // matches "foo"
        assert!(set.is_match_at("foo bar 123", 0)); // matches "foo" at the start
        assert!(set.is_match_at("foo bar 123", 4)); // matches "bar" at offset 4
        assert!(set.is_match_at("bar123", 0)); // matches "bar"
        assert!(set.is_match_at("foo123", 3)); // matches "123" at offset 3
        assert!(!set.is_match_at("!@#", 0)); // no match
    }

    #[test]
    fn test_is_match_at_empty() {
        let set = RegexSet::empty();
        assert!(!set.is_match_at("foo", 0)); // no match in empty set
    }

    #[test]
    fn test_is_match_at_with_offset() {
        let patterns = vec![r"\Afoo", r"bar", r"\d+"];
        let set = RegexSet::new(&patterns).unwrap();

        assert!(set.is_match_at("foobar", 0)); // matches "foo" at start
        assert!(set.is_match_at("123", 0)); // matches "123" at start
        assert!(!set.is_match_at("foobar", 1)); // no match at offset 1
        assert!(set.is_match_at("barfoo", 0)); // matches "bar" at start
    }
}

#[cfg(test)]
mod tests_llm_16_577 {
    use super::*;

use crate::*;
    use crate::re_set::bytes::RegexSet;

    #[test]
    fn test_regex_set_len() {
        let patterns = vec![r"\w+", r"\d+", r"[a-z]+"];
        let regex_set = RegexSet::new(&patterns).unwrap();
        assert_eq!(regex_set.len(), patterns.len());
    }

    #[test]
    fn test_empty_regex_set_len() {
        let regex_set = RegexSet::empty();
        assert_eq!(regex_set.len(), 0);
    }

    #[test]
    fn test_single_regex_set_len() {
        let patterns = vec![r"foo"];
        let regex_set = RegexSet::new(&patterns).unwrap();
        assert_eq!(regex_set.len(), 1);
    }
}

#[cfg(test)]
mod tests_llm_16_578 {
    use super::*;

use crate::*;
    use crate::RegexSet;

    #[test]
    fn test_matches() {
        let set = RegexSet::new(&[
            r"\w+",
            r"\d+",
            r"\pL+",
            r"foo",
            r"bar",
            r"barfoo",
            r"foobar",
        ])
        .unwrap();

        let matches: Vec<_> = set.matches("foobar").into_iter().collect();
        assert_eq!(matches, vec![0, 2, 3, 4, 6]);

        let matches = set.matches("foobar");
        assert!(!matches.matched(5));
        assert!(matches.matched(6));
    }

    #[test]
    fn test_no_matches() {
        let set = RegexSet::new(&[
            r"\d+",
            r"hello",
            r"world",
        ])
        .unwrap();

        let matches: Vec<_> = set.matches("foo").into_iter().collect();
        assert!(matches.is_empty());
    }

    #[test]
    fn test_partial_matches() {
        let set = RegexSet::new(&[
            r"bar",
            r"foo",
            r"hello",
        ])
        .unwrap();

        let matches: Vec<_> = set.matches("barfoo").into_iter().collect();
        assert_eq!(matches, vec![0, 1]);
    }

    #[test]
    fn test_matches_empty_string() {
        let set = RegexSet::new(&[
            r"foo",
            r"bar",
        ])
        .unwrap();

        let matches: Vec<_> = set.matches("").into_iter().collect();
        assert!(matches.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_580 {
    use super::*;

use crate::*;
    use crate::RegexSet;

    #[test]
    fn test_patterns() {
        let set = RegexSet::new(&[
            r"\w+",
            r"\d+",
            r"\pL+",
            r"foo",
            r"bar",
            r"barfoo",
            r"foobar",
        ]).unwrap();

        let expected_patterns = vec![
            r"\w+".to_string(),
            r"\d+".to_string(),
            r"\pL+".to_string(),
            r"foo".to_string(),
            r"bar".to_string(),
            r"barfoo".to_string(),
            r"foobar".to_string(),
        ];

        assert_eq!(set.patterns(), expected_patterns.as_slice());
    }
}

#[cfg(test)]
mod tests_llm_16_588 {
    use super::*;

use crate::*;
    use crate::RegexSet;

    #[test]
    fn test_empty_regex_set() {
        let set = RegexSet::empty();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_590 {
    use super::*;

use crate::*;
    use crate::RegexSet;

    #[test]
    fn test_is_match() {
        let set = RegexSet::new(&[r"\w+", r"\d+"]).unwrap();
        assert!(set.is_match("foo"));
        assert!(set.is_match("123"));
        assert!(!set.is_match("☃"));
        assert!(!set.is_match(""));
    }

    #[test]
    fn test_is_match_empty_set() {
        let set = RegexSet::empty();
        assert!(!set.is_match("foo"));
    }

    #[test]
    fn test_is_match_multiple_patterns() {
        let set = RegexSet::new(&[r"foo", r"bar", r"baz"]).unwrap();
        assert!(set.is_match("foo"));
        assert!(set.is_match("bar"));
        assert!(set.is_match("baz"));
        assert!(!set.is_match("qux"));
    }
    
    #[test]
    fn test_is_match_anchored() {
        let set = RegexSet::new(&[r"^foo$", r"^bar$"]).unwrap();
        assert!(set.is_match("foo"));
        assert!(set.is_match("bar"));
        assert!(!set.is_match("foobar"));
        assert!(!set.is_match("barfoo"));
    }
}

#[cfg(test)]
mod tests_llm_16_591 {
    use super::*;

use crate::*;
    use crate::re_set::unicode::RegexSet;

    #[test]
    fn test_is_match_at() {
        let patterns = vec![r"\d+", r"foo", r"bar"];
        let regex_set = RegexSet::new(&patterns).unwrap();
        
        assert!(regex_set.is_match_at("123foo", 0));
        assert!(regex_set.is_match_at("foo123", 0));
        assert!(regex_set.is_match_at("bar123", 0));
        assert!(!regex_set.is_match_at("abc", 0));
        assert!(regex_set.is_match_at("123foo", 3));
        assert!(!regex_set.is_match_at("abc123", 0));
        assert!(regex_set.is_match_at("foobar", 0));
        assert!(regex_set.is_match_at("barfoo", 0));
    }

    #[test]
    fn test_is_match_at_with_offset() {
        let patterns = vec![r"\Afoo", r"bar$"];
        let regex_set = RegexSet::new(&patterns).unwrap();

        assert!(regex_set.is_match_at("foobar", 0));
        assert!(regex_set.is_match_at("xyzbar", 3));
        assert!(!regex_set.is_match_at("fooabc", 0));
        assert!(!regex_set.is_match_at("barxyz", 0));
    }
}

#[cfg(test)]
mod tests_llm_16_592 {
    use super::*;

use crate::*;
    use crate::RegexSet;

    #[test]
    fn test_regex_set_len() {
        let patterns = vec![r"foo", r"bar", r"baz"];
        let set = RegexSet::new(&patterns).unwrap();
        assert_eq!(set.len(), patterns.len());
    }

    #[test]
    fn test_empty_regex_set_len() {
        let set = RegexSet::empty();
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_regex_set_len_after_addition() {
        let initial_patterns = vec![r"foo", r"bar"];
        let mut set = RegexSet::new(&initial_patterns).unwrap();
        assert_eq!(set.len(), initial_patterns.len());

        let additional_patterns = vec![r"baz"];
        let new_set = RegexSet::new(&additional_patterns).unwrap();
        assert_eq!(new_set.len(), additional_patterns.len());

        // Testing len of combined patterns
        let combined_patterns = initial_patterns.iter().chain(additional_patterns.iter()).collect::<Vec<_>>();
        let combined_set = RegexSet::new(&combined_patterns).unwrap();
        assert_eq!(combined_set.len(), combined_patterns.len());
    }
}

#[cfg(test)]
mod tests_llm_16_595 {
    use super::*; // Import the necessary items

use crate::*;
    use crate::RegexSet;

    #[test]
    fn test_patterns() {
        let set = RegexSet::new(&[
            r"\w+",
            r"\d+",
            r"\pL+",
            r"foo",
            r"bar",
            r"barfoo",
            r"foobar",
        ]).unwrap();
        let patterns = set.patterns();
        assert_eq!(patterns.len(), 7);
        assert_eq!(patterns, &vec![
            r"\w+".to_string(),
            r"\d+".to_string(),
            r"\pL+".to_string(),
            r"foo".to_string(),
            r"bar".to_string(),
            r"barfoo".to_string(),
            r"foobar".to_string(),
        ]);
    }
}

#[cfg(test)]
mod tests_llm_16_596 {
    use super::*;

use crate::*;
    use crate::RegexSet;

    #[test]
    fn test_read_matches_at() {
        let patterns = ["foo", "bar", "baz"];
        let set = RegexSet::new(&patterns).unwrap();
        
        let mut matches = vec![false; patterns.len()];
        let text = "foobarbaz";
        
        // Test with start at 0
        let any = set.read_matches_at(&mut matches, text, 0);
        assert!(any);
        assert!(matches[0]); // "foo" matches
        assert!(matches[1]); // "bar" matches
        assert!(matches[2]); // "baz" matches

        // Reset matches
        matches.iter_mut().for_each(|m| *m = false);
        
        // Test with start at 3
        let any = set.read_matches_at(&mut matches, text, 3);
        assert!(any);
        assert!(matches[1]); // "bar" matches
        assert!(matches[2]); // "baz" matches
        assert!(!matches[0]); // "foo" does not match

        // Reset matches
        matches.iter_mut().for_each(|m| *m = false);
        
        // Test with start at 6
        let any = set.read_matches_at(&mut matches, text, 6);
        assert!(any);
        assert!(matches[2]); // "baz" matches
        assert!(!matches[0]); // "foo" does not match
        assert!(!matches[1]); // "bar" does not match

        // Test with start beyond text length
        let any = set.read_matches_at(&mut matches, text, 10);
        assert!(!any);
        assert!(!matches[0]); // "foo" does not match
        assert!(!matches[1]); // "bar" does not match
        assert!(!matches[2]); // "baz" does not match
    }
}
