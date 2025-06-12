use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::iter::FusedIterator;
use std::ops::{Index, Range};
use std::str::FromStr;
use std::sync::Arc;

use crate::find_byte::find_byte;

use crate::error::Error;
use crate::exec::{Exec, ExecNoSync};
use crate::expand::expand_bytes;
use crate::re_builder::bytes::RegexBuilder;
use crate::re_trait::{self, RegularExpression, SubCapturesPosIter};

/// Match represents a single match of a regex in a haystack.
///
/// The lifetime parameter `'t` refers to the lifetime of the matched text.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'t> {
    text: &'t [u8],
    start: usize,
    end: usize,
}

impl<'t> Match<'t> {
    /// Returns the starting byte offset of the match in the haystack.
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the ending byte offset of the match in the haystack.
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns true if and only if this match has a length of zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Returns the length, in bytes, of this match.
    #[inline]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns the range over the starting and ending byte offsets of the
    /// match in the haystack.
    #[inline]
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// Returns the matched text.
    #[inline]
    pub fn as_bytes(&self) -> &'t [u8] {
        &self.text[self.range()]
    }

    /// Creates a new match from the given haystack and byte offsets.
    #[inline]
    fn new(haystack: &'t [u8], start: usize, end: usize) -> Match<'t> {
        Match { text: haystack, start, end }
    }
}

impl<'t> std::fmt::Debug for Match<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Match");
        fmt.field("start", &self.start).field("end", &self.end);
        if let Ok(s) = std::str::from_utf8(self.as_bytes()) {
            fmt.field("bytes", &s);
        } else {
            // FIXME: It would be nice if this could be printed as a string
            // with invalid UTF-8 replaced with hex escapes. A alloc would
            // probably okay if that makes it easier, but regex-automata does
            // (at time of writing) have internal routines that do this. So
            // maybe we should expose them.
            fmt.field("bytes", &self.as_bytes());
        }
        fmt.finish()
    }
}

impl<'t> From<Match<'t>> for Range<usize> {
    fn from(m: Match<'t>) -> Range<usize> {
        m.range()
    }
}

/// A compiled regular expression for matching arbitrary bytes.
///
/// It can be used to search, split or replace text. All searching is done with
/// an implicit `.*?` at the beginning and end of an expression. To force an
/// expression to match the whole string (or a prefix or a suffix), you must
/// use an anchor like `^` or `$` (or `\A` and `\z`).
///
/// Like the `Regex` type in the parent module, matches with this regex return
/// byte offsets into the search text. **Unlike** the parent `Regex` type,
/// these byte offsets may not correspond to UTF-8 sequence boundaries since
/// the regexes in this module can match arbitrary bytes.
#[derive(Clone)]
pub struct Regex(Exec);

impl fmt::Display for Regex {
    /// Shows the original regular expression.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Debug for Regex {
    /// Shows the original regular expression.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// A constructor for Regex from an Exec.
///
/// This is hidden because Exec isn't actually part of the public API.
#[doc(hidden)]
impl From<Exec> for Regex {
    fn from(exec: Exec) -> Regex {
        Regex(exec)
    }
}

impl FromStr for Regex {
    type Err = Error;

    /// Attempts to parse a string into a regular expression
    fn from_str(s: &str) -> Result<Regex, Error> {
        Regex::new(s)
    }
}

/// Core regular expression methods.
impl Regex {
    /// Compiles a regular expression. Once compiled, it can be used repeatedly
    /// to search, split or replace text in a string.
    ///
    /// If an invalid expression is given, then an error is returned.
    pub fn new(re: &str) -> Result<Regex, Error> {
        RegexBuilder::new(re).build()
    }

    /// Returns true if and only if there is a match for the regex in the
    /// string given.
    ///
    /// It is recommended to use this method if all you need to do is test
    /// a match, since the underlying matching engine may be able to do less
    /// work.
    ///
    /// # Example
    ///
    /// Test if some text contains at least one word with exactly 13 ASCII word
    /// bytes:
    ///
    /// ```rust
    /// # use regex::bytes::Regex;
    /// # fn main() {
    /// let text = b"I categorically deny having triskaidekaphobia.";
    /// assert!(Regex::new(r"\b\w{13}\b").unwrap().is_match(text));
    /// # }
    /// ```
    pub fn is_match(&self, text: &[u8]) -> bool {
        self.is_match_at(text, 0)
    }

    /// Returns the start and end byte range of the leftmost-first match in
    /// `text`. If no match exists, then `None` is returned.
    ///
    /// Note that this should only be used if you want to discover the position
    /// of the match. Testing the existence of a match is faster if you use
    /// `is_match`.
    ///
    /// # Example
    ///
    /// Find the start and end location of the first word with exactly 13
    /// ASCII word bytes:
    ///
    /// ```rust
    /// # use regex::bytes::Regex;
    /// # fn main() {
    /// let text = b"I categorically deny having triskaidekaphobia.";
    /// let mat = Regex::new(r"\b\w{13}\b").unwrap().find(text).unwrap();
    /// assert_eq!((mat.start(), mat.end()), (2, 15));
    /// # }
    /// ```
    pub fn find<'t>(&self, text: &'t [u8]) -> Option<Match<'t>> {
        self.find_at(text, 0)
    }

    /// Returns an iterator for each successive non-overlapping match in
    /// `text`, returning the start and end byte indices with respect to
    /// `text`.
    ///
    /// # Example
    ///
    /// Find the start and end location of every word with exactly 13 ASCII
    /// word bytes:
    ///
    /// ```rust
    /// # use regex::bytes::Regex;
    /// # fn main() {
    /// let text = b"Retroactively relinquishing remunerations is reprehensible.";
    /// for mat in Regex::new(r"\b\w{13}\b").unwrap().find_iter(text) {
    ///     println!("{:?}", mat);
    /// }
    /// # }
    /// ```
    pub fn find_iter<'r, 't>(&'r self, text: &'t [u8]) -> Matches<'r, 't> {
        Matches(self.0.searcher().find_iter(text))
    }

    /// Returns the capture groups corresponding to the leftmost-first
    /// match in `text`. Capture group `0` always corresponds to the entire
    /// match. If no match is found, then `None` is returned.
    ///
    /// You should only use `captures` if you need access to the location of
    /// capturing group matches. Otherwise, `find` is faster for discovering
    /// the location of the overall match.
    ///
    /// # Examples
    ///
    /// Say you have some text with movie names and their release years,
    /// like "'Citizen Kane' (1941)". It'd be nice if we could search for text
    /// looking like that, while also extracting the movie name and its release
    /// year separately.
    ///
    /// ```rust
    /// # use regex::bytes::Regex;
    /// # fn main() {
    /// let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    /// let text = b"Not my favorite movie: 'Citizen Kane' (1941).";
    /// let caps = re.captures(text).unwrap();
    /// assert_eq!(caps.get(1).unwrap().as_bytes(), &b"Citizen Kane"[..]);
    /// assert_eq!(caps.get(2).unwrap().as_bytes(), &b"1941"[..]);
    /// assert_eq!(caps.get(0).unwrap().as_bytes(), &b"'Citizen Kane' (1941)"[..]);
    /// // You can also access the groups by index using the Index notation.
    /// // Note that this will panic on an invalid index.
    /// assert_eq!(&caps[1], b"Citizen Kane");
    /// assert_eq!(&caps[2], b"1941");
    /// assert_eq!(&caps[0], b"'Citizen Kane' (1941)");
    /// # }
    /// ```
    ///
    /// Note that the full match is at capture group `0`. Each subsequent
    /// capture group is indexed by the order of its opening `(`.
    ///
    /// We can make this example a bit clearer by using *named* capture groups:
    ///
    /// ```rust
    /// # use regex::bytes::Regex;
    /// # fn main() {
    /// let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)")
    ///                .unwrap();
    /// let text = b"Not my favorite movie: 'Citizen Kane' (1941).";
    /// let caps = re.captures(text).unwrap();
    /// assert_eq!(caps.name("title").unwrap().as_bytes(), b"Citizen Kane");
    /// assert_eq!(caps.name("year").unwrap().as_bytes(), b"1941");
    /// assert_eq!(caps.get(0).unwrap().as_bytes(), &b"'Citizen Kane' (1941)"[..]);
    /// // You can also access the groups by name using the Index notation.
    /// // Note that this will panic on an invalid group name.
    /// assert_eq!(&caps["title"], b"Citizen Kane");
    /// assert_eq!(&caps["year"], b"1941");
    /// assert_eq!(&caps[0], b"'Citizen Kane' (1941)");
    ///
    /// # }
    /// ```
    ///
    /// Here we name the capture groups, which we can access with the `name`
    /// method or the `Index` notation with a `&str`. Note that the named
    /// capture groups are still accessible with `get` or the `Index` notation
    /// with a `usize`.
    ///
    /// The `0`th capture group is always unnamed, so it must always be
    /// accessed with `get(0)` or `[0]`.
    pub fn captures<'t>(&self, text: &'t [u8]) -> Option<Captures<'t>> {
        self.captures_at(text, 0)
    }

    /// Returns an iterator over all the non-overlapping capture groups matched
    /// in `text`. This is operationally the same as `find_iter`, except it
    /// yields information about capturing group matches.
    ///
    /// # Example
    ///
    /// We can use this to find all movie titles and their release years in
    /// some text, where the movie is formatted like "'Title' (xxxx)":
    ///
    /// ```rust
    /// # use std::str; use regex::bytes::Regex;
    /// # fn main() {
    /// let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)")
    ///                .unwrap();
    /// let text = b"'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";
    /// for caps in re.captures_iter(text) {
    ///     let title = str::from_utf8(&caps["title"]).unwrap();
    ///     let year = str::from_utf8(&caps["year"]).unwrap();
    ///     println!("Movie: {:?}, Released: {:?}", title, year);
    /// }
    /// // Output:
    /// // Movie: Citizen Kane, Released: 1941
    /// // Movie: The Wizard of Oz, Released: 1939
    /// // Movie: M, Released: 1931
    /// # }
    /// ```
    pub fn captures_iter<'r, 't>(
        &'r self,
        text: &'t [u8],
    ) -> CaptureMatches<'r, 't> {
        CaptureMatches(self.0.searcher().captures_iter(text))
    }

    /// Returns an iterator of substrings of `text` delimited by a match of the
    /// regular expression. Namely, each element of the iterator corresponds to
    /// text that *isn't* matched by the regular expression.
    ///
    /// This method will *not* copy the text given.
    ///
    /// # Example
    ///
    /// To split a string delimited by arbitrary amounts of spaces or tabs:
    ///
    /// ```rust
    /// # use regex::bytes::Regex;
    /// # fn main() {
    /// let re = Regex::new(r"[ \t]+").unwrap();
    /// let fields: Vec<&[u8]> = re.split(b"a b \t  c\td    e").collect();
    /// assert_eq!(fields, vec![
    ///     &b"a"[..], &b"b"[..], &b"c"[..], &b"d"[..], &b"e"[..],
    /// ]);
    /// # }
    /// ```
    pub fn split<'r, 't>(&'r self, text: &'t [u8]) -> Split<'r, 't> {
        Split { finder: self.find_iter(text), last: 0 }
    }

    /// Returns an iterator of at most `limit` substrings of `text` delimited
    /// by a match of the regular expression. (A `limit` of `0` will return no
    /// substrings.) Namely, each element of the iterator corresponds to text
    /// that *isn't* matched by the regular expression. The remainder of the
    /// string that is not split will be the last element in the iterator.
    ///
    /// This method will *not* copy the text given.
    ///
    /// # Example
    ///
    /// Get the first two words in some text:
    ///
    /// ```rust
    /// # use regex::bytes::Regex;
    /// # fn main() {
    /// let re = Regex::new(r"\W+").unwrap();
    /// let fields: Vec<&[u8]> = re.splitn(b"Hey! How are you?", 3).collect();
    /// assert_eq!(fields, vec![&b"Hey"[..], &b"How"[..], &b"are you?"[..]]);
    /// # }
    /// ```
    pub fn splitn<'r, 't>(
        &'r self,
        text: &'t [u8],
        limit: usize,
    ) -> SplitN<'r, 't> {
        SplitN { splits: self.split(text), n: limit }
    }

    /// Replaces the leftmost-first match with the replacement provided. The
    /// replacement can be a regular byte string (where `$N` and `$name` are
    /// expanded to match capture groups) or a function that takes the matches'
    /// `Captures` and returns the replaced byte string.
    ///
    /// If no match is found, then a copy of the byte string is returned
    /// unchanged.
    ///
    /// # Replacement string syntax
    ///
    /// All instances of `$name` in the replacement text is replaced with the
    /// corresponding capture group `name`.
    ///
    /// `name` may be an integer corresponding to the index of the
    /// capture group (counted by order of opening parenthesis where `0` is the
    /// entire match) or it can be a name (consisting of letters, digits or
    /// underscores) corresponding to a named capture group.
    ///
    /// If `name` isn't a valid capture group (whether the name doesn't exist
    /// or isn't a valid index), then it is replaced with the empty string.
    ///
    /// The longest possible name is used. e.g., `$1a` looks up the capture
    /// group named `1a` and not the capture group at index `1`. To exert more
    /// precise control over the name, use braces, e.g., `${1}a`.
    ///
    /// To write a literal `$` use `$$`.
    ///
    /// # Examples
    ///
    /// Note that this function is polymorphic with respect to the replacement.
    /// In typical usage, this can just be a normal byte string:
    ///
    /// ```rust
    /// # use regex::bytes::Regex;
    /// # fn main() {
    /// let re = Regex::new("[^01]+").unwrap();
    /// assert_eq!(re.replace(b"1078910", &b""[..]), &b"1010"[..]);
    /// # }
    /// ```
    ///
    /// But anything satisfying the `Replacer` trait will work. For example, a
    /// closure of type `|&Captures| -> Vec<u8>` provides direct access to the
    /// captures corresponding to a match. This allows one to access capturing
    /// group matches easily:
    ///
    /// ```rust
    /// # use regex::bytes::Regex;
    /// # use regex::bytes::Captures; fn main() {
    /// let re = Regex::new(r"([^,\s]+),\s+(\S+)").unwrap();
    /// let result = re.replace(b"Springsteen, Bruce", |caps: &Captures| {
    ///     let mut replacement = caps[2].to_owned();
    ///     replacement.push(b' ');
    ///     replacement.extend(&caps[1]);
    ///     replacement
    /// });
    /// assert_eq!(result, &b"Bruce Springsteen"[..]);
    /// # }
    /// ```
    ///
    /// But this is a bit cumbersome to use all the time. Instead, a simple
    /// syntax is supported that expands `$name` into the corresponding capture
    /// group. Here's the last example, but using this expansion technique
    /// with named capture groups:
    ///
    /// ```rust
    /// # use regex::bytes::Regex;
    /// # fn main() {
    /// let re = Regex::new(r"(?P<last>[^,\s]+),\s+(?P<first>\S+)").unwrap();
    /// let result = re.replace(b"Springsteen, Bruce", &b"$first $last"[..]);
    /// assert_eq!(result, &b"Bruce Springsteen"[..]);
    /// # }
    /// ```
    ///
    /// Note that using `$2` instead of `$first` or `$1` instead of `$last`
    /// would produce the same result. To write a literal `$` use `$$`.
    ///
    /// Sometimes the replacement string requires use of curly braces to
    /// delineate a capture group replacement and surrounding literal text.
    /// For example, if we wanted to join two words together with an
    /// underscore:
    ///
    /// ```rust
    /// # use regex::bytes::Regex;
    /// # fn main() {
    /// let re = Regex::new(r"(?P<first>\w+)\s+(?P<second>\w+)").unwrap();
    /// let result = re.replace(b"deep fried", &b"${first}_$second"[..]);
    /// assert_eq!(result, &b"deep_fried"[..]);
    /// # }
    /// ```
    ///
    /// Without the curly braces, the capture group name `first_` would be
    /// used, and since it doesn't exist, it would be replaced with the empty
    /// string.
    ///
    /// Finally, sometimes you just want to replace a literal string with no
    /// regard for capturing group expansion. This can be done by wrapping a
    /// byte string with `NoExpand`:
    ///
    /// ```rust
    /// # use regex::bytes::Regex;
    /// # fn main() {
    /// use regex::bytes::NoExpand;
    ///
    /// let re = Regex::new(r"(?P<last>[^,\s]+),\s+(\S+)").unwrap();
    /// let result = re.replace(b"Springsteen, Bruce", NoExpand(b"$2 $last"));
    /// assert_eq!(result, &b"$2 $last"[..]);
    /// # }
    /// ```
    pub fn replace<'t, R: Replacer>(
        &self,
        text: &'t [u8],
        rep: R,
    ) -> Cow<'t, [u8]> {
        self.replacen(text, 1, rep)
    }

    /// Replaces all non-overlapping matches in `text` with the replacement
    /// provided. This is the same as calling `replacen` with `limit` set to
    /// `0`.
    ///
    /// See the documentation for `replace` for details on how to access
    /// capturing group matches in the replacement text.
    pub fn replace_all<'t, R: Replacer>(
        &self,
        text: &'t [u8],
        rep: R,
    ) -> Cow<'t, [u8]> {
        self.replacen(text, 0, rep)
    }

    /// Replaces at most `limit` non-overlapping matches in `text` with the
    /// replacement provided. If `limit` is 0, then all non-overlapping matches
    /// are replaced.
    ///
    /// See the documentation for `replace` for details on how to access
    /// capturing group matches in the replacement text.
    pub fn replacen<'t, R: Replacer>(
        &self,
        text: &'t [u8],
        limit: usize,
        mut rep: R,
    ) -> Cow<'t, [u8]> {
        if let Some(rep) = rep.no_expansion() {
            let mut it = self.find_iter(text).enumerate().peekable();
            if it.peek().is_none() {
                return Cow::Borrowed(text);
            }
            let mut new = Vec::with_capacity(text.len());
            let mut last_match = 0;
            for (i, m) in it {
                new.extend_from_slice(&text[last_match..m.start()]);
                new.extend_from_slice(&rep);
                last_match = m.end();
                if limit > 0 && i >= limit - 1 {
                    break;
                }
            }
            new.extend_from_slice(&text[last_match..]);
            return Cow::Owned(new);
        }

        // The slower path, which we use if the replacement needs access to
        // capture groups.
        let mut it = self.captures_iter(text).enumerate().peekable();
        if it.peek().is_none() {
            return Cow::Borrowed(text);
        }
        let mut new = Vec::with_capacity(text.len());
        let mut last_match = 0;
        for (i, cap) in it {
            // unwrap on 0 is OK because captures only reports matches
            let m = cap.get(0).unwrap();
            new.extend_from_slice(&text[last_match..m.start()]);
            rep.replace_append(&cap, &mut new);
            last_match = m.end();
            if limit > 0 && i >= limit - 1 {
                break;
            }
        }
        new.extend_from_slice(&text[last_match..]);
        Cow::Owned(new)
    }
}

/// Advanced or "lower level" search methods.
impl Regex {
    /// Returns the end location of a match in the text given.
    ///
    /// This method may have the same performance characteristics as
    /// `is_match`, except it provides an end location for a match. In
    /// particular, the location returned *may be shorter* than the proper end
    /// of the leftmost-first match that you would find via `Regex::find`.
    ///
    /// Note that it is not guaranteed that this routine finds the shortest or
    /// "earliest" possible match. Instead, the main idea of this API is that
    /// it returns the offset at the point at which the internal regex engine
    /// has determined that a match has occurred. This may vary depending on
    /// which internal regex engine is used, and thus, the offset itself may
    /// change.
    ///
    /// # Example
    ///
    /// Typically, `a+` would match the entire first sequence of `a` in some
    /// text, but `shortest_match` can give up as soon as it sees the first
    /// `a`.
    ///
    /// ```rust
    /// # use regex::bytes::Regex;
    /// # fn main() {
    /// let text = b"aaaaa";
    /// let pos = Regex::new(r"a+").unwrap().shortest_match(text);
    /// assert_eq!(pos, Some(1));
    /// # }
    /// ```
    pub fn shortest_match(&self, text: &[u8]) -> Option<usize> {
        self.shortest_match_at(text, 0)
    }

    /// Returns the same as shortest_match, but starts the search at the given
    /// offset.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only
    /// match when `start == 0`.
    pub fn shortest_match_at(
        &self,
        text: &[u8],
        start: usize,
    ) -> Option<usize> {
        self.0.searcher().shortest_match_at(text, start)
    }

    /// Returns the same as is_match, but starts the search at the given
    /// offset.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only
    /// match when `start == 0`.
    pub fn is_match_at(&self, text: &[u8], start: usize) -> bool {
        self.0.searcher().is_match_at(text, start)
    }

    /// Returns the same as find, but starts the search at the given
    /// offset.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only
    /// match when `start == 0`.
    pub fn find_at<'t>(
        &self,
        text: &'t [u8],
        start: usize,
    ) -> Option<Match<'t>> {
        self.0
            .searcher()
            .find_at(text, start)
            .map(|(s, e)| Match::new(text, s, e))
    }

    /// Returns the same as [`Regex::captures`], but starts the search at the
    /// given offset.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only
    /// match when `start == 0`.
    pub fn captures_at<'t>(
        &self,
        text: &'t [u8],
        start: usize,
    ) -> Option<Captures<'t>> {
        let mut locs = self.capture_locations();
        self.captures_read_at(&mut locs, text, start).map(move |_| Captures {
            text,
            locs: locs.0,
            named_groups: self.0.capture_name_idx().clone(),
        })
    }

    /// This is like `captures`, but uses
    /// [`CaptureLocations`](struct.CaptureLocations.html)
    /// instead of
    /// [`Captures`](struct.Captures.html) in order to amortize allocations.
    ///
    /// To create a `CaptureLocations` value, use the
    /// `Regex::capture_locations` method.
    ///
    /// This returns the overall match if this was successful, which is always
    /// equivalence to the `0`th capture group.
    pub fn captures_read<'t>(
        &self,
        locs: &mut CaptureLocations,
        text: &'t [u8],
    ) -> Option<Match<'t>> {
        self.captures_read_at(locs, text, 0)
    }

    /// Returns the same as `captures_read`, but starts the search at the given
    /// offset and populates the capture locations given.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only
    /// match when `start == 0`.
    pub fn captures_read_at<'t>(
        &self,
        locs: &mut CaptureLocations,
        text: &'t [u8],
        start: usize,
    ) -> Option<Match<'t>> {
        self.0
            .searcher()
            .captures_read_at(&mut locs.0, text, start)
            .map(|(s, e)| Match::new(text, s, e))
    }

    /// An undocumented alias for `captures_read_at`.
    ///
    /// The `regex-capi` crate previously used this routine, so to avoid
    /// breaking that crate, we continue to provide the name as an undocumented
    /// alias.
    #[doc(hidden)]
    pub fn read_captures_at<'t>(
        &self,
        locs: &mut CaptureLocations,
        text: &'t [u8],
        start: usize,
    ) -> Option<Match<'t>> {
        self.captures_read_at(locs, text, start)
    }
}

/// Auxiliary methods.
impl Regex {
    /// Returns the original string of this regex.
    pub fn as_str(&self) -> &str {
        &self.0.regex_strings()[0]
    }

    /// Returns an iterator over the capture names.
    pub fn capture_names(&self) -> CaptureNames<'_> {
        CaptureNames(self.0.capture_names().iter())
    }

    /// Returns the number of captures.
    pub fn captures_len(&self) -> usize {
        self.0.capture_names().len()
    }

    /// Returns the total number of capturing groups that appear in every
    /// possible match.
    ///
    /// If the number of capture groups can vary depending on the match, then
    /// this returns `None`. That is, a value is only returned when the number
    /// of matching groups is invariant or "static."
    ///
    /// Note that like [`Regex::captures_len`], this **does** include the
    /// implicit capturing group corresponding to the entire match. Therefore,
    /// when a non-None value is returned, it is guaranteed to be at least `1`.
    /// Stated differently, a return value of `Some(0)` is impossible.
    ///
    /// # Example
    ///
    /// This shows a few cases where a static number of capture groups is
    /// available and a few cases where it is not.
    ///
    /// ```
    /// use regex::bytes::Regex;
    ///
    /// let len = |pattern| {
    ///     Regex::new(pattern).map(|re| re.static_captures_len())
    /// };
    ///
    /// assert_eq!(Some(1), len("a")?);
    /// assert_eq!(Some(2), len("(a)")?);
    /// assert_eq!(Some(2), len("(a)|(b)")?);
    /// assert_eq!(Some(3), len("(a)(b)|(c)(d)")?);
    /// assert_eq!(None, len("(a)|b")?);
    /// assert_eq!(None, len("a|(b)")?);
    /// assert_eq!(None, len("(b)*")?);
    /// assert_eq!(Some(2), len("(b)+")?);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn static_captures_len(&self) -> Option<usize> {
        self.0.static_captures_len().map(|len| len.saturating_add(1))
    }

    /// Returns an empty set of capture locations that can be reused in
    /// multiple calls to `captures_read` or `captures_read_at`.
    pub fn capture_locations(&self) -> CaptureLocations {
        CaptureLocations(self.0.searcher().locations())
    }

    /// An alias for `capture_locations` to preserve backward compatibility.
    ///
    /// The `regex-capi` crate uses this method, so to avoid breaking that
    /// crate, we continue to export it as an undocumented API.
    #[doc(hidden)]
    pub fn locations(&self) -> CaptureLocations {
        CaptureLocations(self.0.searcher().locations())
    }
}

/// An iterator over all non-overlapping matches for a particular string.
///
/// The iterator yields a tuple of integers corresponding to the start and end
/// of the match. The indices are byte offsets. The iterator stops when no more
/// matches can be found.
///
/// `'r` is the lifetime of the compiled regular expression and `'t` is the
/// lifetime of the matched byte string.
#[derive(Debug)]
pub struct Matches<'r, 't>(re_trait::Matches<'t, ExecNoSync<'r>>);

impl<'r, 't> Iterator for Matches<'r, 't> {
    type Item = Match<'t>;

    fn next(&mut self) -> Option<Match<'t>> {
        let text = self.0.text();
        self.0.next().map(|(s, e)| Match::new(text, s, e))
    }
}

impl<'r, 't> FusedIterator for Matches<'r, 't> {}

/// An iterator that yields all non-overlapping capture groups matching a
/// particular regular expression.
///
/// The iterator stops when no more matches can be found.
///
/// `'r` is the lifetime of the compiled regular expression and `'t` is the
/// lifetime of the matched byte string.
#[derive(Debug)]
pub struct CaptureMatches<'r, 't>(
    re_trait::CaptureMatches<'t, ExecNoSync<'r>>,
);

impl<'r, 't> Iterator for CaptureMatches<'r, 't> {
    type Item = Captures<'t>;

    fn next(&mut self) -> Option<Captures<'t>> {
        self.0.next().map(|locs| Captures {
            text: self.0.text(),
            locs,
            named_groups: self.0.regex().capture_name_idx().clone(),
        })
    }
}

impl<'r, 't> FusedIterator for CaptureMatches<'r, 't> {}

/// Yields all substrings delimited by a regular expression match.
///
/// `'r` is the lifetime of the compiled regular expression and `'t` is the
/// lifetime of the byte string being split.
#[derive(Debug)]
pub struct Split<'r, 't> {
    finder: Matches<'r, 't>,
    last: usize,
}

impl<'r, 't> Iterator for Split<'r, 't> {
    type Item = &'t [u8];

    fn next(&mut self) -> Option<&'t [u8]> {
        let text = self.finder.0.text();
        match self.finder.next() {
            None => {
                if self.last > text.len() {
                    None
                } else {
                    let s = &text[self.last..];
                    self.last = text.len() + 1; // Next call will return None
                    Some(s)
                }
            }
            Some(m) => {
                let matched = &text[self.last..m.start()];
                self.last = m.end();
                Some(matched)
            }
        }
    }
}

impl<'r, 't> FusedIterator for Split<'r, 't> {}

/// Yields at most `N` substrings delimited by a regular expression match.
///
/// The last substring will be whatever remains after splitting.
///
/// `'r` is the lifetime of the compiled regular expression and `'t` is the
/// lifetime of the byte string being split.
#[derive(Debug)]
pub struct SplitN<'r, 't> {
    splits: Split<'r, 't>,
    n: usize,
}

impl<'r, 't> Iterator for SplitN<'r, 't> {
    type Item = &'t [u8];

    fn next(&mut self) -> Option<&'t [u8]> {
        if self.n == 0 {
            return None;
        }

        self.n -= 1;
        if self.n > 0 {
            return self.splits.next();
        }

        let text = self.splits.finder.0.text();
        if self.splits.last > text.len() {
            // We've already returned all substrings.
            None
        } else {
            // self.n == 0, so future calls will return None immediately
            Some(&text[self.splits.last..])
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.n))
    }
}

impl<'r, 't> FusedIterator for SplitN<'r, 't> {}

/// An iterator over the names of all possible captures.
///
/// `None` indicates an unnamed capture; the first element (capture 0, the
/// whole matched region) is always unnamed.
///
/// `'r` is the lifetime of the compiled regular expression.
#[derive(Clone, Debug)]
pub struct CaptureNames<'r>(::std::slice::Iter<'r, Option<String>>);

impl<'r> Iterator for CaptureNames<'r> {
    type Item = Option<&'r str>;

    fn next(&mut self) -> Option<Option<&'r str>> {
        self.0
            .next()
            .as_ref()
            .map(|slot| slot.as_ref().map(|name| name.as_ref()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    fn count(self) -> usize {
        self.0.count()
    }
}

impl<'r> ExactSizeIterator for CaptureNames<'r> {}

impl<'r> FusedIterator for CaptureNames<'r> {}

/// CaptureLocations is a low level representation of the raw offsets of each
/// submatch.
///
/// You can think of this as a lower level
/// [`Captures`](struct.Captures.html), where this type does not support
/// named capturing groups directly and it does not borrow the text that these
/// offsets were matched on.
///
/// Primarily, this type is useful when using the lower level `Regex` APIs
/// such as `read_captures`, which permits amortizing the allocation in which
/// capture match locations are stored.
///
/// In order to build a value of this type, you'll need to call the
/// `capture_locations` method on the `Regex` being used to execute the search.
/// The value returned can then be reused in subsequent searches.
///
/// # Example
///
/// This example shows how to create and use `CaptureLocations` in a search.
///
/// ```
/// use regex::bytes::Regex;
///
/// let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
/// let mut locs = re.capture_locations();
/// let m = re.captures_read(&mut locs, b"Bruce Springsteen").unwrap();
/// assert_eq!(0..17, m.range());
/// assert_eq!(Some((0, 17)), locs.get(0));
/// assert_eq!(Some((0, 5)), locs.get(1));
/// assert_eq!(Some((6, 17)), locs.get(2));
///
/// // Asking for an invalid capture group always returns None.
/// assert_eq!(None, locs.get(3));
/// assert_eq!(None, locs.get(34973498648));
/// assert_eq!(None, locs.get(9944060567225171988));
/// ```
#[derive(Clone, Debug)]
pub struct CaptureLocations(re_trait::Locations);

/// A type alias for `CaptureLocations` for backwards compatibility.
///
/// Previously, we exported `CaptureLocations` as `Locations` in an
/// undocumented API. To prevent breaking that code (e.g., in `regex-capi`),
/// we continue re-exporting the same undocumented API.
#[doc(hidden)]
pub type Locations = CaptureLocations;

impl CaptureLocations {
    /// Returns the start and end positions of the Nth capture group. Returns
    /// `None` if `i` is not a valid capture group or if the capture group did
    /// not match anything. The positions returned are *always* byte indices
    /// with respect to the original string matched.
    #[inline]
    pub fn get(&self, i: usize) -> Option<(usize, usize)> {
        self.0.pos(i)
    }

    /// Returns the total number of capture groups (even if they didn't match).
    ///
    /// This is always at least `1` since every regex has at least `1`
    /// capturing group that corresponds to the entire match.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// An alias for the `get` method for backwards compatibility.
    ///
    /// Previously, we exported `get` as `pos` in an undocumented API. To
    /// prevent breaking that code (e.g., in `regex-capi`), we continue
    /// re-exporting the same undocumented API.
    #[doc(hidden)]
    #[inline]
    pub fn pos(&self, i: usize) -> Option<(usize, usize)> {
        self.get(i)
    }
}

/// Captures represents a group of captured byte strings for a single match.
///
/// The 0th capture always corresponds to the entire match. Each subsequent
/// index corresponds to the next capture group in the regex. If a capture
/// group is named, then the matched byte string is *also* available via the
/// `name` method. (Note that the 0th capture is always unnamed and so must be
/// accessed with the `get` method.)
///
/// Positions returned from a capture group are always byte indices.
///
/// `'t` is the lifetime of the matched text.
pub struct Captures<'t> {
    text: &'t [u8],
    locs: re_trait::Locations,
    named_groups: Arc<HashMap<String, usize>>,
}

impl<'t> Captures<'t> {
    /// Returns the match associated with the capture group at index `i`. If
    /// `i` does not correspond to a capture group, or if the capture group
    /// did not participate in the match, then `None` is returned.
    ///
    /// # Examples
    ///
    /// Get the text of the match with a default of an empty string if this
    /// group didn't participate in the match:
    ///
    /// ```rust
    /// # use regex::bytes::Regex;
    /// let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    /// let caps = re.captures(b"abc123").unwrap();
    ///
    /// let text1 = caps.get(1).map_or(&b""[..], |m| m.as_bytes());
    /// let text2 = caps.get(2).map_or(&b""[..], |m| m.as_bytes());
    /// assert_eq!(text1, &b"123"[..]);
    /// assert_eq!(text2, &b""[..]);
    /// ```
    pub fn get(&self, i: usize) -> Option<Match<'t>> {
        self.locs.pos(i).map(|(s, e)| Match::new(self.text, s, e))
    }

    /// Returns the match for the capture group named `name`. If `name` isn't a
    /// valid capture group or didn't match anything, then `None` is returned.
    pub fn name(&self, name: &str) -> Option<Match<'t>> {
        self.named_groups.get(name).and_then(|&i| self.get(i))
    }

    /// An iterator that yields all capturing matches in the order in which
    /// they appear in the regex. If a particular capture group didn't
    /// participate in the match, then `None` is yielded for that capture.
    ///
    /// The first match always corresponds to the overall match of the regex.
    pub fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 't> {
        SubCaptureMatches { caps: self, it: self.locs.iter() }
    }

    /// Expands all instances of `$name` in `replacement` to the corresponding
    /// capture group `name`, and writes them to the `dst` buffer given.
    ///
    /// `name` may be an integer corresponding to the index of the capture
    /// group (counted by order of opening parenthesis where `0` is the
    /// entire match) or it can be a name (consisting of letters, digits or
    /// underscores) corresponding to a named capture group.
    ///
    /// If `name` isn't a valid capture group (whether the name doesn't exist
    /// or isn't a valid index), then it is replaced with the empty string.
    ///
    /// The longest possible name consisting of the characters `[_0-9A-Za-z]`
    /// is used. e.g., `$1a` looks up the capture group named `1a` and not the
    /// capture group at index `1`. To exert more precise control over the
    /// name, or to refer to a capture group name that uses characters outside
    /// of `[_0-9A-Za-z]`, use braces, e.g., `${1}a` or `${foo[bar].baz}`. When
    /// using braces, any sequence of valid UTF-8 bytes is permitted. If the
    /// sequence does not refer to a capture group name in the corresponding
    /// regex, then it is replaced with an empty string.
    ///
    /// To write a literal `$` use `$$`.
    pub fn expand(&self, replacement: &[u8], dst: &mut Vec<u8>) {
        expand_bytes(self, replacement, dst)
    }

    /// Returns the total number of capture groups (even if they didn't match).
    ///
    /// This is always at least `1`, since every regex has at least one capture
    /// group that corresponds to the full match.
    #[inline]
    pub fn len(&self) -> usize {
        self.locs.len()
    }
}

impl<'t> fmt::Debug for Captures<'t> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Captures").field(&CapturesDebug(self)).finish()
    }
}

struct CapturesDebug<'c, 't>(&'c Captures<'t>);

impl<'c, 't> fmt::Debug for CapturesDebug<'c, 't> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

        // We'd like to show something nice here, even if it means an
        // allocation to build a reverse index.
        let slot_to_name: HashMap<&usize, &String> =
            self.0.named_groups.iter().map(|(a, b)| (b, a)).collect();
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

/// Get a group by index.
///
/// `'t` is the lifetime of the matched text.
///
/// The text can't outlive the `Captures` object if this method is
/// used, because of how `Index` is defined (normally `a[i]` is part
/// of `a` and can't outlive it); to do that, use `get()` instead.
///
/// # Panics
///
/// If there is no group at the given index.
impl<'t> Index<usize> for Captures<'t> {
    type Output = [u8];

    fn index(&self, i: usize) -> &[u8] {
        self.get(i)
            .map(|m| m.as_bytes())
            .unwrap_or_else(|| panic!("no group at index '{}'", i))
    }
}

/// Get a group by name.
///
/// `'t` is the lifetime of the matched text and `'i` is the lifetime
/// of the group name (the index).
///
/// The text can't outlive the `Captures` object if this method is
/// used, because of how `Index` is defined (normally `a[i]` is part
/// of `a` and can't outlive it); to do that, use `name` instead.
///
/// # Panics
///
/// If there is no group named by the given value.
impl<'t, 'i> Index<&'i str> for Captures<'t> {
    type Output = [u8];

    fn index<'a>(&'a self, name: &'i str) -> &'a [u8] {
        self.name(name)
            .map(|m| m.as_bytes())
            .unwrap_or_else(|| panic!("no group named '{}'", name))
    }
}

/// An iterator that yields all capturing matches in the order in which they
/// appear in the regex.
///
/// If a particular capture group didn't participate in the match, then `None`
/// is yielded for that capture. The first match always corresponds to the
/// overall match of the regex.
///
/// The lifetime `'c` corresponds to the lifetime of the `Captures` value, and
/// the lifetime `'t` corresponds to the originally matched text.
#[derive(Clone, Debug)]
pub struct SubCaptureMatches<'c, 't> {
    caps: &'c Captures<'t>,
    it: SubCapturesPosIter<'c>,
}

impl<'c, 't> Iterator for SubCaptureMatches<'c, 't> {
    type Item = Option<Match<'t>>;

    fn next(&mut self) -> Option<Option<Match<'t>>> {
        self.it
            .next()
            .map(|cap| cap.map(|(s, e)| Match::new(self.caps.text, s, e)))
    }
}

impl<'c, 't> FusedIterator for SubCaptureMatches<'c, 't> {}

/// Replacer describes types that can be used to replace matches in a byte
/// string.
///
/// In general, users of this crate shouldn't need to implement this trait,
/// since implementations are already provided for `&[u8]` along with other
/// variants of bytes types and `FnMut(&Captures) -> Vec<u8>` (or any
/// `FnMut(&Captures) -> T` where `T: AsRef<[u8]>`), which covers most use cases.
pub trait Replacer {
    /// Appends text to `dst` to replace the current match.
    ///
    /// The current match is represented by `caps`, which is guaranteed to
    /// have a match at capture group `0`.
    ///
    /// For example, a no-op replacement would be
    /// `dst.extend(&caps[0])`.
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>);

    /// Return a fixed unchanging replacement byte string.
    ///
    /// When doing replacements, if access to `Captures` is not needed (e.g.,
    /// the replacement byte string does not need `$` expansion), then it can
    /// be beneficial to avoid finding sub-captures.
    ///
    /// In general, this is called once for every call to `replacen`.
    fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, [u8]>> {
        None
    }

    /// Return a `Replacer` that borrows and wraps this `Replacer`.
    ///
    /// This is useful when you want to take a generic `Replacer` (which might
    /// not be cloneable) and use it without consuming it, so it can be used
    /// more than once.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::bytes::{Regex, Replacer};
    ///
    /// fn replace_all_twice<R: Replacer>(
    ///     re: Regex,
    ///     src: &[u8],
    ///     mut rep: R,
    /// ) -> Vec<u8> {
    ///     let dst = re.replace_all(src, rep.by_ref());
    ///     let dst = re.replace_all(&dst, rep.by_ref());
    ///     dst.into_owned()
    /// }
    /// ```
    fn by_ref<'r>(&'r mut self) -> ReplacerRef<'r, Self> {
        ReplacerRef(self)
    }
}

/// By-reference adaptor for a `Replacer`
///
/// Returned by [`Replacer::by_ref`](trait.Replacer.html#method.by_ref).
#[derive(Debug)]
pub struct ReplacerRef<'a, R: ?Sized>(&'a mut R);

impl<'a, R: Replacer + ?Sized + 'a> Replacer for ReplacerRef<'a, R> {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>) {
        self.0.replace_append(caps, dst)
    }
    fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, [u8]>> {
        self.0.no_expansion()
    }
}

impl<'a> Replacer for &'a [u8] {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>) {
        caps.expand(*self, dst);
    }

    fn no_expansion(&mut self) -> Option<Cow<'_, [u8]>> {
        no_expansion(self)
    }
}

impl<'a> Replacer for &'a Vec<u8> {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>) {
        caps.expand(*self, dst);
    }

    fn no_expansion(&mut self) -> Option<Cow<'_, [u8]>> {
        no_expansion(self)
    }
}

impl Replacer for Vec<u8> {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>) {
        caps.expand(self, dst);
    }

    fn no_expansion(&mut self) -> Option<Cow<'_, [u8]>> {
        no_expansion(self)
    }
}

impl<'a> Replacer for Cow<'a, [u8]> {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>) {
        caps.expand(self.as_ref(), dst);
    }

    fn no_expansion(&mut self) -> Option<Cow<'_, [u8]>> {
        no_expansion(self)
    }
}

impl<'a> Replacer for &'a Cow<'a, [u8]> {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>) {
        caps.expand(self.as_ref(), dst);
    }

    fn no_expansion(&mut self) -> Option<Cow<'_, [u8]>> {
        no_expansion(self)
    }
}

fn no_expansion<T: AsRef<[u8]>>(t: &T) -> Option<Cow<'_, [u8]>> {
    let s = t.as_ref();
    match find_byte(b'$', s) {
        Some(_) => None,
        None => Some(Cow::Borrowed(s)),
    }
}

impl<F, T> Replacer for F
where
    F: FnMut(&Captures<'_>) -> T,
    T: AsRef<[u8]>,
{
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>) {
        dst.extend_from_slice((*self)(caps).as_ref());
    }
}

/// `NoExpand` indicates literal byte string replacement.
///
/// It can be used with `replace` and `replace_all` to do a literal byte string
/// replacement without expanding `$name` to their corresponding capture
/// groups. This can be both convenient (to avoid escaping `$`, for example)
/// and performant (since capture groups don't need to be found).
///
/// `'t` is the lifetime of the literal text.
#[derive(Clone, Debug)]
pub struct NoExpand<'t>(pub &'t [u8]);

impl<'t> Replacer for NoExpand<'t> {
    fn replace_append(&mut self, _: &Captures<'_>, dst: &mut Vec<u8>) {
        dst.extend_from_slice(self.0);
    }

    fn no_expansion(&mut self) -> Option<Cow<'_, [u8]>> {
        Some(Cow::Borrowed(self.0))
    }
}

#[cfg(test)]
mod tests_llm_16_58 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_replace_append() {
        let re = Regex::new(r"(?P<digit>[0-9]+)").unwrap();
        let text = b"abc123def";
        let caps = re.captures(text).unwrap();
        let mut replacement = b"[$digit] ".to_vec();
        let mut dst = Vec::new();
        
        replacement.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, b"[123] ");
    }

    #[test]
    fn test_replace_append_with_no_capture() {
        let re = Regex::new(r"(?P<digit>[0-9]+)").unwrap();
        let text = b"abcdef";
        let caps = re.captures(text).unwrap();
        let mut replacement = b"[$digit] ".to_vec();
        let mut dst = Vec::new();
        
        replacement.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, b"[] ");
    }

    #[test]
    fn test_replace_append_with_multiple_captures() {
        let re = Regex::new(r"(?P<first>[a-z]+)(?P<second>[0-9]+)").unwrap();
        let text = b"abc123";
        let caps = re.captures(text).unwrap();
        let mut replacement = b"[$first][$second] ".to_vec();
        let mut dst = Vec::new();
        
        replacement.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, b"[abc][123] ");
    }
}

#[cfg(test)]
mod tests_llm_16_69 {
    use super::*;

use crate::*;
    use crate::{Captures, Regex};
    use std::borrow::Cow;

    #[test]
    fn test_no_expansion() {
        let mut vec: Vec<u8> = vec![1, 2, 3];
        let result: Option<Cow<[u8]>> = vec.no_expansion();
        assert_eq!(result, Some(Cow::Borrowed(&[1, 2, 3][..])));
    }
}

#[cfg(test)]
mod tests_llm_16_70 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;
    use std::vec::Vec;
    use re_bytes::{Captures, Replacer};

    #[test]
    fn test_replace_append() {
        let re = Regex::new(r"(?P<digits>\d+)").unwrap();
        let text = b"abc123def";
        let caps = re.captures(text).unwrap();
        
        let mut dst = Vec::new();
        let mut replacer = Vec::from(b"[$digits]"); // Using a replacement as an example
        replacer.replace_append(&caps, &mut dst);

        assert_eq!(dst, b"[123]"); // Verify that dst contains the expected replacement output
    }
}

#[cfg(test)]
mod tests_llm_16_73 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;
    use std::sync::Arc;
    use std::collections::HashMap;

    struct MockReplacer {
        replacement: Vec<u8>,
    }

    impl re_bytes::Replacer for MockReplacer {
        fn replace_append(&mut self, caps: &re_bytes::Captures, dst: &mut Vec<u8>) {
            dst.extend_from_slice(&self.replacement);
        }

        fn no_expansion(&mut self) -> Option<std::borrow::Cow<'_, [u8]>> {
            Some(std::borrow::Cow::Borrowed(&self.replacement))
        }
    }

    #[test]
    fn test_replace_append() {
        let re = Regex::new(r"(\d+)").unwrap();
        let caps = re.captures(b"abc123").unwrap();

        let mut dst = Vec::new();
        let mut replacer = MockReplacer {
            replacement: b"replaced".to_vec(),
        };

        replacer.replace_append(&caps, &mut dst);
        let expected = b"replaced".to_vec();
        assert_eq!(dst, expected);
    }

    #[test]
    fn test_replace_append_empty() {
        let re = Regex::new(r"(\d+)").unwrap();
        let caps = re.captures(b"abc").unwrap();

        let mut dst = Vec::new();
        let mut replacer = MockReplacer {
            replacement: b"replaced".to_vec(),
        };

        // Here, we should handle the case when no digits are captured
        replacer.replace_append(&caps, &mut dst);
        assert!(dst.is_empty()); // Expecting empty since there's no match
    }
}

#[cfg(test)]
mod tests_llm_16_129 {
    use super::*;

use crate::*;
    use std::slice;

    #[test]
    fn test_count_with_no_captures() {
        let captures: Vec<Option<String>> = Vec::new();
        let capture_names = CaptureNames(captures.iter());
        assert_eq!(capture_names.count(), 0);
    }

    #[test]
    fn test_count_with_one_named_capture() {
        let captures: Vec<Option<String>> = vec![Some("name".to_string())];
        let capture_names = CaptureNames(captures.iter());
        assert_eq!(capture_names.count(), 1);
    }

    #[test]
    fn test_count_with_multiple_captures() {
        let captures: Vec<Option<String>> = vec![
            None,
            Some("name1".to_string()),
            Some("name2".to_string()),
        ];
        let capture_names = CaptureNames(captures.iter());
        assert_eq!(capture_names.count(), 3);
    }

    #[test]
    fn test_count_with_only_unnamed_captures() {
        let captures: Vec<Option<String>> = vec![None, None, None];
        let capture_names = CaptureNames(captures.iter());
        assert_eq!(capture_names.count(), 3);
    }
}

#[cfg(test)]
mod tests_llm_16_131 {
    use super::*;

use crate::*;
    use std::slice;

    #[test]
    fn test_size_hint() {
        let names: Vec<Option<String>> = vec![None, Some("first".to_string()), Some("second".to_string())];
        let slice: &[Option<String>] = &names;
        let capture_names = CaptureNames(slice.iter());
        
        let (lower, upper) = capture_names.size_hint();
        
        assert_eq!(lower, 3);
        assert_eq!(upper, Some(3));
    }

    #[test]
    fn test_empty_size_hint() {
        let names: Vec<Option<String>> = vec![];
        let slice: &[Option<String>] = &names;
        let capture_names = CaptureNames(slice.iter());
        
        let (lower, upper) = capture_names.size_hint();
        
        assert_eq!(lower, 0);
        assert_eq!(upper, Some(0));
    }
}

#[cfg(test)]
mod tests_llm_16_151 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_sub_capture_matches_next() {
        let re = Regex::new(r"(?P<first>[a-z]+)(?P<second>[0-9]+)").unwrap();
        let caps = re.captures(b"abc123").unwrap();
        let mut sub_matches = caps.iter();

        let first_match = sub_matches.next().unwrap();
        assert_eq!(first_match.as_ref().map(|m| m.as_bytes()), Some(&b"abc"[..]));

        let second_match = sub_matches.next().unwrap();
        assert_eq!(second_match.as_ref().map(|m| m.as_bytes()), Some(&b"123"[..]));

        let none_match = sub_matches.next();
        assert_eq!(none_match, None);
    }

    #[test]
    fn test_sub_capture_matches_next_no_match() {
        let re = Regex::new(r"(?P<first>[0-9]+)(?P<second>[a-z]+)").unwrap();
        let caps = re.captures(b"abc123").unwrap();
        let mut sub_matches = caps.iter();

        let first_match = sub_matches.next().unwrap();
        assert_eq!(first_match.as_ref().map(|m| m.as_bytes()), Some(&b"abc123"[..]));

        let second_match = sub_matches.next().unwrap();
        assert_eq!(second_match, None); // second group didn't match, should return None

        let none_match = sub_matches.next();
        assert_eq!(none_match, None);
    }
}

#[cfg(test)]
mod tests_llm_16_204 {
    use super::*;

use crate::*;
    use std::borrow::Cow;
    use crate::bytes::Regex;
    use std::sync::Arc;
    use std::collections::HashMap;

    #[test]
    fn test_replace_append() {
        let re = Regex::new(r"(?P<first>[a-z]+)(?P<second>[0-9]+)").unwrap();
        let caps = re.captures(b"abc123").unwrap();
        
        let mut dst = Vec::new();
        let mut replacer: Cow<[u8]> = Cow::Borrowed(b"$first-$second");
        
        replacer.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, b"abc-123");
    }
    
    #[test]
    fn test_replace_append_no_match() {
        let re = Regex::new(r"(?P<first>[a-z]+)(?P<second>[0-9]+)").unwrap();
        let caps = re.captures(b"xyz").unwrap();
        
        let mut dst = Vec::new();
        let mut replacer: Cow<[u8]> = Cow::Borrowed(b"$first-$second");
        
        replacer.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, b"xyz-");
    }

    #[test]
    fn test_replace_append_empty() {
        let re = Regex::new(r"a?b?").unwrap();
        let caps = re.captures(b"b").unwrap();
        
        let mut dst = Vec::new();
        let mut replacer: Cow<[u8]> = Cow::Borrowed(b"$0");
        
        replacer.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, b"b");
    }
}

#[cfg(test)]
mod tests_llm_16_209 {
    use super::*;

use crate::*;
    use crate::re_bytes::Replacer; // Ensure correct path
    use std::borrow::Cow;

    #[test]
    fn test_no_expansion() {
        let mut vec: Vec<u8> = vec![1, 2, 3];
        let result = vec.no_expansion();
        assert_eq!(result, Some(Cow::Borrowed(&[1, 2, 3][..])));
    }

    #[test]
    fn test_no_expansion_empty() {
        let mut vec: Vec<u8> = vec![];
        let result = vec.no_expansion();
        assert_eq!(result, Some(Cow::Borrowed(&[][..])));
    }
}

#[cfg(test)]
mod tests_llm_16_210 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn test_replace_append() {
        let re = Regex::new(r"(?P<foo>[a-z]+)(?P<bar>[0-9]+)").unwrap();
        let text = b"abc123";
        let caps = re.captures(text).unwrap();
        
        let mut dst = Vec::new();
        let mut replacer = Vec::from(b"$foo-$bar");

        replacer.replace_append(&caps, &mut dst);
        
        assert_eq!(&dst, b"abc-123");
    }
    
    #[test]
    fn test_replace_append_no_matches() {
        let re = Regex::new(r"(?P<foo>[a-z]+)(?P<bar>[0-9]+)").unwrap();
        let text = b"xyz";
        let caps = re.captures(text).unwrap();
        
        let mut dst = Vec::new();
        let mut replacer = Vec::from(b"$foo-$bar");

        replacer.replace_append(&caps, &mut dst);
        
        assert_eq!(&dst, b"xyz-");
    }

    #[test]
    fn test_replace_append_empty_replacer() {
        let re = Regex::new(r"(?P<foo>[a-z]+)(?P<bar>[0-9]+)").unwrap();
        let text = b"abc123";
        let caps = re.captures(text).unwrap();
        
        let mut dst = Vec::new();
        let mut replacer = Vec::new();

        replacer.replace_append(&caps, &mut dst);
        
        assert!(dst.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_530 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_capture_locations_get() {
        let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let m = re.captures_read(&mut locs, b"Bruce Springsteen").unwrap();

        assert_eq!(Some((0, 17)), locs.get(0));
        assert_eq!(Some((0, 5)), locs.get(1));
        assert_eq!(Some((6, 17)), locs.get(2));
        assert_eq!(None, locs.get(3));
        assert_eq!(None, locs.get(34973498648));
        assert_eq!(None, locs.get(9944060567225171988));
    }
}

#[cfg(test)]
mod tests_llm_16_532 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;
    use re_bytes::CaptureLocations;

    #[test]
    fn test_pos_valid_capture() {
        let re = Regex::new(r"(?P<first>\w+)\s+(?P<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let m = re.captures_read(&mut locs, b"Bruce Springsteen").unwrap();
        
        assert_eq!(locs.pos(0), Some((0, 17))); // Full match
        assert_eq!(locs.pos(1), Some((0, 5)));  // First capture
        assert_eq!(locs.pos(2), Some((6, 17))); // Second capture
    }

    #[test]
    fn test_pos_invalid_capture() {
        let re = Regex::new(r"(?P<first>\w+)\s+(?P<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let m = re.captures_read(&mut locs, b"Bruce Springsteen").unwrap();
        
        assert_eq!(locs.pos(3), None);  // Invalid capture group
        assert_eq!(locs.pos(10), None); // Invalid capture group index
    }
}

#[cfg(test)]
mod tests_llm_16_533 {
    use super::*;

use crate::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use re_bytes::Captures;
    use crate::bytes::Regex;

    #[test]
    fn test_expand_with_numerical_capture() {
        let re = Regex::new(r"(?P<group1>[a-z]+)(?P<group2>\d+)").unwrap();
        let caps = re.captures(b"abc123").unwrap();

        let mut dst = Vec::new();
        caps.expand(b"$1$2", &mut dst);
        assert_eq!(&dst[..], b"abc123");
    }

    #[test]
    fn test_expand_with_named_capture() {
        let re = Regex::new(r"(?P<letters>[a-z]+)(?P<numbers>\d+)").unwrap();
        let caps = re.captures(b"abc123").unwrap();

        let mut dst = Vec::new();
        caps.expand(b"${letters}-${numbers}", &mut dst);
        assert_eq!(&dst[..], b"abc-123");
    }

    #[test]
    fn test_expand_with_invalid_capture() {
        let re = Regex::new(r"(?P<valid>[a-z]+)").unwrap();
        let caps = re.captures(b"abc").unwrap();

        let mut dst = Vec::new();
        caps.expand(b"$1$invalid$2", &mut dst);
        assert_eq!(&dst[..], b"$invalid");
    }

    #[test]
    fn test_expand_with_literal_dollar() {
        let re = Regex::new(r"(?P<group>[a-z]+)").unwrap();
        let caps = re.captures(b"abc").unwrap();

        let mut dst = Vec::new();
        caps.expand(b"$$group$$", &mut dst);
        assert_eq!(&dst[..], b"$group$");
    }
}

#[cfg(test)]
mod tests_llm_16_534 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_get_success() {
        let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
        let caps = re.captures(b"abc123").unwrap();
        
        let match_1 = caps.get(1);
        let match_2 = caps.get(2);
        
        assert_eq!(match_1.map(|m| m.as_bytes()), Some(&b"123"[..]));
        assert_eq!(match_2.map(|m| m.as_bytes()), Some(&b""[..]));
    }

    #[test]
    fn test_get_invalid_index() {
        let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
        let caps = re.captures(b"abc123").unwrap();
        
        let match_3 = caps.get(3); // Invalid index
        assert_eq!(match_3, None);
    }

    #[test]
    fn test_get_zero_index() {
        let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
        let caps = re.captures(b"abc123").unwrap();
        
        let match_0 = caps.get(0);
        
        assert_eq!(match_0.map(|m| m.as_bytes()), Some(b"abc123".as_slice()));
    }

    #[test]
    fn test_get_non_capturing_group() {
        let re = Regex::new(r"[a-z]+(?:[0-9]+|[A-Z]+)").unwrap();
        let caps = re.captures(b"abc").unwrap();
        
        let match_1 = caps.get(1); // No capturing group
        let match_2 = caps.get(0);
        
        assert_eq!(match_1, None);
        assert_eq!(match_2.map(|m| m.as_bytes()), Some(b"abc".as_slice()));
    }
}

#[cfg(test)]
mod tests_llm_16_536 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;
    use re_trait::Locations;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn test_captures_len() {
        let re = Regex::new(r"(\d+)|([a-z]+)").unwrap();
        let text = b"abc123";
        let captures = re.captures(text).unwrap();

        // The regex has 3 capturing groups: full match, group 1, and group 2.
        assert_eq!(captures.len(), 3);
    }

    #[test]
    fn test_captures_len_empty() {
        let re = Regex::new(r"([a-z]+)").unwrap();
        let text = b"";
        let captures = re.captures(text);

        // There should be no captures for the empty string, but len should still be 1.
        assert!(captures.is_none());
    }

    #[test]
    fn test_captures_len_single_match() {
        let re = Regex::new(r"([a-z]+)").unwrap();
        let text = b"abc";
        let captures = re.captures(text).unwrap();

        // The regex has 2 capturing groups: full match and group 1.
        assert_eq!(captures.len(), 2);
    }
}

#[cfg(test)]
mod tests_llm_16_540 {
    use super::*;

use crate::*;
    use re_bytes::Match;

    #[test]
    fn test_is_empty_when_match_is_empty() {
        let haystack: &[u8] = b"test";
        let m = Match::new(haystack, 0, 0);
        assert!(m.is_empty());
    }

    #[test]
    fn test_is_empty_when_match_is_not_empty() {
        let haystack: &[u8] = b"test";
        let m = Match::new(haystack, 0, 2);
        assert!(!m.is_empty());
    }

    #[test]
    fn test_is_empty_when_match_length_is_zero() {
        let haystack: &[u8] = b"test";
        let m = Match::new(haystack, 2, 2);
        assert!(m.is_empty());
    }

    #[test]
    fn test_is_empty_with_different_positions() {
        let haystack: &[u8] = b"hello";
        let m1 = Match::new(haystack, 2, 2);
        let m2 = Match::new(haystack, 0, 5);
        assert!(m1.is_empty());
        assert!(!m2.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_541 {
    use super::*;

use crate::*;
    use crate::re_bytes::Match;

    #[test]
    fn test_len() {
        let haystack: &[u8] = b"hello";
        let m = Match::new(haystack, 1, 4);
        assert_eq!(m.len(), 3);
    }

    #[test]
    fn test_len_zero() {
        let haystack: &[u8] = b"hello";
        let m = Match::new(haystack, 2, 2);
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn test_len_full_match() {
        let haystack: &[u8] = b"hello";
        let m = Match::new(haystack, 0, 5);
        assert_eq!(m.len(), 5);
    }
}

#[cfg(test)]
mod tests_llm_16_542 {
    use super::*;

use crate::*;
    use re_bytes::Match;

    #[test]
    fn test_match_new() {
        let haystack = b"hello, world";
        let start = 0;
        let end = 5;
        let m = Match::new(haystack, start, end);

        assert_eq!(m.start(), start);
        assert_eq!(m.end(), end);
        assert_eq!(m.len(), end - start);
        assert!(!m.is_empty());
        assert_eq!(m.as_bytes(), &haystack[start..end]);
    }

    #[test]
    fn test_match_new_empty() {
        let haystack = b"hello, world";
        let start = 5;
        let end = 5;
        let m = Match::new(haystack, start, end);

        assert_eq!(m.start(), start);
        assert_eq!(m.end(), end);
        assert_eq!(m.len(), end - start);
        assert!(m.is_empty());
        assert_eq!(m.as_bytes(), &haystack[start..end]);
    }
}

#[cfg(test)]
mod tests_llm_16_545 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_as_str() {
        let regex_str = r"\d+";
        let regex = Regex::new(regex_str).expect("Failed to create regex");
        assert_eq!(regex.as_str(), regex_str);
    }

    #[test]
    fn test_as_str_invalid_regex() {
        let regex_str = r"(";
        let regex = Regex::new(regex_str);
        assert!(regex.is_err());
    }

    #[test]
    fn test_as_str_on_empty_regex() {
        let regex_str = ""; 
        let regex = Regex::new(regex_str).expect("Failed to create regex");
        assert_eq!(regex.as_str(), regex_str);
    }

    #[test]
    fn test_as_str_on_complex_regex() {
        let regex_str = r"([a-zA-Z]+) (\d+)";
        let regex = Regex::new(regex_str).expect("Failed to create regex");
        assert_eq!(regex.as_str(), regex_str);
    }
}

#[cfg(test)]
mod tests_llm_16_546 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_capture_locations_empty_regex() {
        let re = Regex::new(r"").unwrap();
        let locs = re.capture_locations();
        assert_eq!(locs.len(), 1); // At least one capturing group for the entire match
        assert_eq!(locs.get(0), Some((0, 0))); // Should match empty string
    }

    #[test]
    fn test_capture_locations_single_match() {
        let re = Regex::new(r"(?<first>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let m = re.captures_read(&mut locs, b"Hello").unwrap();
        assert_eq!(0..5, m.range());
        assert_eq!(locs.get(0), Some((0, 5))); // Full match
        assert_eq!(locs.get(1), Some((0, 5))); // First capturing group
        assert_eq!(locs.get(2), None); // Invalid group
    }

    #[test]
    fn test_capture_locations_multiple_matches() {
        let re = Regex::new(r"(?P<first>\w+)\s+(?P<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let m = re.captures_read(&mut locs, b"Bruce Springsteen").unwrap();
        assert_eq!(0..17, m.range());
        assert_eq!(locs.get(0), Some((0, 17))); // Full match
        assert_eq!(locs.get(1), Some((0, 5))); // First capturing group
        assert_eq!(locs.get(2), Some((6, 17))); // Second capturing group
        assert_eq!(locs.get(3), None); // Invalid group
    }

    #[test]
    fn test_capture_locations_invalid_group() {
        let re = Regex::new(r"(?<name>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let m = re.captures_read(&mut locs, b"Rust").unwrap();
        assert!(matches!(locs.get(5), None)); // Requesting a non-existent group
    }

    #[test]
    fn test_capture_locations_with_no_matches() {
        let re = Regex::new(r"(?<name>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let m = re.captures_read(&mut locs, b"").unwrap();
        assert_eq!(0..0, m.range());
        assert_eq!(locs.get(0), Some((0, 0))); // Full match on empty string
    }
}

#[cfg(test)]
mod tests_llm_16_547 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_capture_names() {
        let re = Regex::new(r"(?P<name>[a-zA-Z]+)").unwrap();
        let capture_names: Vec<Option<&str>> = re.capture_names().collect();
        assert_eq!(capture_names.len(), 2); // Group 0 is unnamed, group 1 is named "name".
        assert_eq!(capture_names[0], None); // Group 0 should be None (unnamed).
        assert_eq!(capture_names[1], Some("name"));
    }

    #[test]
    fn test_capture_names_no_named_groups() {
        let re = Regex::new(r"(a)(b)(c)").unwrap();
        let capture_names: Vec<Option<&str>> = re.capture_names().collect();
        assert_eq!(capture_names.len(), 4); // Group 0 is unnamed, groups 1, 2, 3 are unnamed.
        assert_eq!(capture_names[0], None); // Group 0 should be None (unnamed).
        assert_eq!(capture_names[1], None); // Group 1 should be None (unnamed).
        assert_eq!(capture_names[2], None); // Group 2 should be None (unnamed).
        assert_eq!(capture_names[3], None); // Group 3 should be None (unnamed).
    }

    #[test]
    fn test_capture_names_with_multiple_named_groups() {
        let re = Regex::new(r"(?P<first>[a-z]+) (?P<second>[0-9]+)").unwrap();
        let capture_names: Vec<Option<&str>> = re.capture_names().collect();
        assert_eq!(capture_names.len(), 3); // Group 0 is unnamed, groups 1, 2 are named.
        assert_eq!(capture_names[0], None); // Group 0 should be None (unnamed).
        assert_eq!(capture_names[1], Some("first"));
        assert_eq!(capture_names[2], Some("second"));
    }
}

#[cfg(test)]
mod tests_llm_16_548 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_captures() {
        let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
        let text = b"Not my favorite movie: 'Citizen Kane' (1941).";
        let caps = re.captures(text).unwrap();
        assert_eq!(caps.get(1).unwrap().as_bytes(), b"Citizen Kane");
        assert_eq!(caps.get(2).unwrap().as_bytes(), b"1941");
        assert_eq!(caps.get(0).unwrap().as_bytes(), b"'Citizen Kane' (1941)");
    }

    #[test]
    fn test_named_captures() {
        let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
        let text = b"Not my favorite movie: 'Citizen Kane' (1941).";
        let caps = re.captures(text).unwrap();
        assert_eq!(caps.name("title").unwrap().as_bytes(), b"Citizen Kane");
        assert_eq!(caps.name("year").unwrap().as_bytes(), b"1941");
        assert_eq!(caps.get(0).unwrap().as_bytes(), b"'Citizen Kane' (1941)");
    }

    #[test]
    fn test_no_match() {
        let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
        let text = b"Not my favorite movie: 'Citizen Kane' (abc).";
        let caps = re.captures(text);
        assert!(caps.is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_549 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_captures_at() {
        let re = Regex::new(r"(?P<name>\w+)").unwrap();
        let text = b"hello world";

        // Test without offset
        let caps = re.captures_at(text, 0).unwrap();
        assert_eq!(caps.name("name").unwrap().as_bytes(), b"hello");

        // Test with offset
        let caps = re.captures_at(text, 6).unwrap();
        assert_eq!(caps.name("name").unwrap().as_bytes(), b"world");

        // Test offset beyond the text length
        assert!(re.captures_at(text, 20).is_none());

        // Test with offset at the position of the match
        let caps = re.captures_at(text, 5).unwrap();
        assert_eq!(caps.get(0).unwrap().as_bytes(), b"hello");

        // Test with a regex that matches at offset
        let re = Regex::new(r"world").unwrap();
        let caps = re.captures_at(text, 5).unwrap();
        assert_eq!(caps.get(0).unwrap().as_bytes(), b"world");
    }
}

#[cfg(test)]
mod tests_llm_16_551 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_captures_len_empty_regex() {
        let re = Regex::new(r"").unwrap();
        assert_eq!(re.captures_len(), 0);
    }

    #[test]
    fn test_captures_len_single_capture() {
        let re = Regex::new(r"(\d+)").unwrap();
        assert_eq!(re.captures_len(), 1);
    }

    #[test]
    fn test_captures_len_multiple_captures() {
        let re = Regex::new(r"(\d+)-(\d+)").unwrap();
        assert_eq!(re.captures_len(), 2);
    }

    #[test]
    fn test_captures_len_named_capture() {
        let re = Regex::new(r"(?P<first>\d+)-(?P<second>\d+)").unwrap();
        assert_eq!(re.captures_len(), 2);
    }

    #[test]
    fn test_captures_len_mixed_captures() {
        let re = Regex::new(r"(?P<first>\d+)-(\d+)").unwrap();
        assert_eq!(re.captures_len(), 2);
    }

    #[test]
    fn test_captures_len_no_captures() {
        let re = Regex::new(r"\w+").unwrap();
        assert_eq!(re.captures_len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_552 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_captures_read_success() {
        let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let text = b"Bruce Springsteen";
        let m = re.captures_read(&mut locs, text).unwrap();
        assert_eq!(0..17, m.range());
        assert_eq!(Some((0, 5)), locs.get(1));
        assert_eq!(Some((6, 17)), locs.get(2));
    }

    #[test]
    fn test_captures_read_invalid_capture() {
        let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let text = b"SingleWord";
        let m = re.captures_read(&mut locs, text);
        assert!(m.is_none());
    }

    #[test]
    fn test_captures_read_non_matching() {
        let re = Regex::new(r"(?<num>\d+)").unwrap();
        let mut locs = re.capture_locations();
        let text = b"NoDigitsHere";
        let m = re.captures_read(&mut locs, text);
        assert!(m.is_none());
    }

    #[test]
    fn test_captures_read_repeated_calls() {
        let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let text = b"John Doe";
        
        assert!(re.captures_read(&mut locs, text).is_some());
        assert_eq!(Some((0, 4)), locs.get(1));
        assert_eq!(Some((5, 8)), locs.get(2));
        
        // Reusing locs
        let text2 = b"Jane Smith";
        assert!(re.captures_read(&mut locs, text2).is_some());
        assert_eq!(Some((0, 10)), locs.get(0));
    }
}

#[cfg(test)]
mod tests_llm_16_553 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;
    use crate::bytes::CaptureLocations;

    #[test]
    fn test_captures_read_at() {
        let re = Regex::new(r"(?P<word>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let text = b"hello world";

        // Test searching starting from the beginning
        let match_at_start = re.captures_read_at(&mut locs, text, 0).unwrap();
        assert_eq!(0..5, match_at_start.range());
        assert_eq!(Some((0, 5)), locs.get(0)); // entire match
        assert_eq!(Some((0, 5)), locs.get(1)); // named group "word"

        // Test searching starting from an offset
        let match_at_offset = re.captures_read_at(&mut locs, text, 6).unwrap();
        assert_eq!(6..11, match_at_offset.range());
        assert_eq!(Some((6, 11)), locs.get(0)); // entire match
        assert_eq!(Some((6, 11)), locs.get(1)); // named group "word"

        // Test no match case
        let no_match = re.captures_read_at(&mut locs, text, 20);
        assert!(no_match.is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_554 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_find_first_match() {
        let text = b"I categorically deny having triskaidekaphobia.";
        let regex = Regex::new(r"\b\w{13}\b").unwrap();
        let mat = regex.find(text).unwrap();
        assert_eq!((mat.start(), mat.end()), (2, 15));
    }

    #[test]
    fn test_find_no_match() {
        let text = b"This sentence has no matches.";
        let regex = Regex::new(r"\b\w{15}\b").unwrap();
        let mat = regex.find(text);
        assert!(mat.is_none());
    }

    #[test]
    fn test_find_multiple_matches() {
        let text = b"I deny having triskaidekaphobia, and I know another word.";
        let regex = Regex::new(r"\b\w{13}\b").unwrap();
        let first_match = regex.find(text).unwrap();
        assert_eq!((first_match.start(), first_match.end()), (2, 15));
    }
}

#[cfg(test)]
mod tests_llm_16_555 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_find_at() {
        let re = Regex::new(r"\d+").unwrap();

        let text = b"abc123def456";
        let match_at_0 = re.find_at(text, 0);
        let match_at_4 = re.find_at(text, 4);
        let match_at_10 = re.find_at(text, 10);
        let match_at_11 = re.find_at(text, 11); // out of bounds

        assert_eq!(match_at_0.map(|m| (m.start(), m.end())), Some((3, 6))); // match "123"
        assert_eq!(match_at_4.map(|m| (m.start(), m.end())), Some((3, 6))); // match "123" again
        assert_eq!(match_at_10, None); // no match after the end of text
        assert_eq!(match_at_11, None); // no match out of bounds
    }
    
    #[test]
    fn test_find_at_with_anchor() {
        let re = Regex::new(r"\Aabc").unwrap();

        let text = b"abcabc";
        let match_at_0 = re.find_at(text, 0);
        let match_at_1 = re.find_at(text, 1); // match "abc" at index 1

        assert_eq!(match_at_0.map(|m| (m.start(), m.end())), Some((0, 3))); // match "abc"
        assert_eq!(match_at_1, None); // no match at index 1
    }
}

#[cfg(test)]
mod tests_llm_16_556 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_find_iter_basic() {
        let text = b"Retroactively relinquishing remunerations is reprehensible.";
        let regex = Regex::new(r"\b\w{13}\b").unwrap();
        let matches: Vec<(usize, usize)> = regex.find_iter(text)
            .map(|mat| (mat.start(), mat.end()))
            .collect();
        assert_eq!(matches, vec![(0, 13), (14, 27), (28, 41)]);
    }

    #[test]
    fn test_find_iter_no_matches() {
        let text = b"short words.";
        let regex = Regex::new(r"\b\w{13}\b").unwrap();
        let matches: Vec<(usize, usize)> = regex.find_iter(text)
            .map(|mat| (mat.start(), mat.end()))
            .collect();
        assert!(matches.is_empty());
    }

    #[test]
    fn test_find_iter_multiple_matches() {
        let text = b"divisibility responsibilities and indefatigable.";
        let regex = Regex::new(r"\b\w{13}\b").unwrap();
        let matches: Vec<(usize, usize)> = regex.find_iter(text)
            .map(|mat| (mat.start(), mat.end()))
            .collect();
        assert_eq!(matches, vec![(0, 13), (14, 27)]);
    }
}

#[cfg(test)]
mod tests_llm_16_558 {
    use super::*; // Assuming this is in the same module as the `Regex` struct and `is_match_at` method.

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_is_match_at() {
        // Test case where the match is found at the beginning
        let regex = Regex::new(r"\Aabc").unwrap();
        let text = b"abcde";
        assert!(regex.is_match_at(text, 0));

        // Test case where the match is found in the middle
        let regex = Regex::new(r"bcd").unwrap();
        let text = b"abcde";
        assert!(regex.is_match_at(text, 1));

        // Test case where the match is at the end
        let regex = Regex::new(r"de$").unwrap();
        let text = b"abcde";
        assert!(regex.is_match_at(text, 3));

        // Test case where the match is not found
        let regex = Regex::new(r"xyz").unwrap();
        let text = b"abcde";
        assert!(!regex.is_match_at(text, 0));

        // Test case where the match is found but at the wrong starting position
        let regex = Regex::new(r"\Aabc").unwrap();
        let text = b"abcde";
        assert!(!regex.is_match_at(text, 1));

        // Test case with empty text
        let regex = Regex::new(r"\Aabc").unwrap();
        let text = b"";
        assert!(!regex.is_match_at(text, 0));
    }
}

#[cfg(test)]
mod tests_llm_16_559 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_locations() {
        let re = Regex::new(r"(?P<first>\w+)\s+(?P<last>\w+)").unwrap();
        let mut locs = re.locations();
        let m = re.captures_read(&mut locs, b"Bruce Springsteen").unwrap();
        
        assert_eq!(0..17, m.range());
        assert_eq!(Some((0, 17)), locs.get(0));
        assert_eq!(Some((0, 5)), locs.get(1));
        assert_eq!(Some((6, 17)), locs.get(2));
        
        // Invalid capture groups
        assert_eq!(None, locs.get(3));
        assert_eq!(None, locs.get(99));
    }
}

#[cfg(test)]
mod tests_llm_16_561 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;
    use crate::bytes::CaptureLocations;

    #[test]
    fn test_read_captures_at() {
        let re = Regex::new(r"(?P<first>\w+)\s+(?P<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let text = b"Bruce Springsteen";

        // Test for successful match
        let match_result = re.read_captures_at(&mut locs, text, 0);
        assert!(match_result.is_some());
        let m = match_result.unwrap();
        assert_eq!(m.range(), 0..17);
        assert_eq!(Some((0, 5)), locs.get(1));
        assert_eq!(Some((6, 17)), locs.get(2));
        assert_eq!(Some((0, 17)), locs.get(0)); // Entire match

        // Test for invalid start index
        assert_eq!(re.read_captures_at(&mut locs, text, 20), None);
    }
}

#[cfg(test)]
mod tests_llm_16_565 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_shortest_match() {
        let regex = Regex::new(r"a+").unwrap();
        let text = b"aaaaa";
        let pos = regex.shortest_match(text);
        assert_eq!(pos, Some(1));
    }

    #[test]
    fn test_shortest_match_no_match() {
        let regex = Regex::new(r"a+").unwrap();
        let text = b"bbbbb";
        let pos = regex.shortest_match(text);
        assert_eq!(pos, None);
    }

    #[test]
    fn test_shortest_match_multiple_matches() {
        let regex = Regex::new(r"a+").unwrap();
        let text = b"bcaabbaa";
        let pos = regex.shortest_match(text);
        assert_eq!(pos, Some(4)); // Matches 'aa' at position 4
    }

    #[test]
    fn test_shortest_match_empty_text() {
        let regex = Regex::new(r"a+").unwrap();
        let text = b"";
        let pos = regex.shortest_match(text);
        assert_eq!(pos, None);
    }

    #[test]
    fn test_shortest_match_at_start() {
        let regex = Regex::new(r"b+").unwrap();
        let text = b"bbabc";
        let pos = regex.shortest_match(text);
        assert_eq!(pos, Some(0)); // Matches 'bb' at position 0
    }
}

#[cfg(test)]
mod tests_llm_16_566 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_shortest_match_at() {
        let regex = Regex::new(r"a+").unwrap();
        let text = b"aaaabaaa";
        
        // Starting from index 0, the first match of 'a' would be at index 0
        assert_eq!(regex.shortest_match_at(text, 0), Some(0));
        
        // Starting from index 1, the first match of 'a' would be at index 1
        assert_eq!(regex.shortest_match_at(text, 1), Some(1));
        
        // Starting from index 4, the first match of 'a' would be at index 4
        assert_eq!(regex.shortest_match_at(text, 4), Some(4));
        
        // Starting from index 8, there are no matches
        assert_eq!(regex.shortest_match_at(text, 8), None);
    }

    #[test]
    fn test_shortest_match_at_with_empty_text() {
        let regex = Regex::new(r"a+").unwrap();
        let text = b"";
        
        // There are no matches, regardless of the starting index
        assert_eq!(regex.shortest_match_at(text, 0), None);
        assert_eq!(regex.shortest_match_at(text, 1), None);
    }

    #[test]
    fn test_shortest_match_at_with_non_matching_text() {
        let regex = Regex::new(r"a+").unwrap();
        let text = b"bbbb";
        
        // There are no matches, starting from any index
        assert_eq!(regex.shortest_match_at(text, 0), None);
        assert_eq!(regex.shortest_match_at(text, 1), None);
        assert_eq!(regex.shortest_match_at(text, 2), None);
    }
}

#[cfg(test)]
mod tests_llm_16_567 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_split_basic() {
        let re = Regex::new(r"[ \t]+").unwrap();
        let fields: Vec<&[u8]> = re.split(b"a b \t  c\td    e").collect();
        assert_eq!(fields, vec![&b"a"[..], &b"b"[..], &b"c"[..], &b"d"[..], &b"e"[..]]);
    }

    #[test]
    fn test_split_multiple_spaces() {
        let re = Regex::new(r"[ ]+").unwrap();
        let fields: Vec<&[u8]> = re.split(b"a    b     c     d    e").collect();
        assert_eq!(fields, vec![&b"a"[..], &b"b"[..], &b"c"[..], &b"d"[..], &b"e"[..]]);
    }

    #[test]
    fn test_split_tabs() {
        let re = Regex::new(r"[\t]+").unwrap();
        let fields: Vec<&[u8]> = re.split(b"a\tb\t\tc\td").collect();
        assert_eq!(fields, vec![&b"a"[..], &b"b"[..], &b"c"[..], &b"d"[..]]);
    }

    #[test]
    fn test_split_empty_string() {
        let re = Regex::new(r"[ \t]+").unwrap();
        let fields: Vec<&[u8]> = re.split(b"").collect();
        assert_eq!(fields, vec![&b""[..]]);
    }

    #[test]
    fn test_split_no_delimiters() {
        let re = Regex::new(r"[ \t]+").unwrap();
        let fields: Vec<&[u8]> = re.split(b"abc").collect();
        assert_eq!(fields, vec![&b"abc"[..]]);
    }

    #[test]
    fn test_split_leading_delimiters() {
        let re = Regex::new(r"[ \t]+").unwrap();
        let fields: Vec<&[u8]> = re.split(b"    a b").collect();
        assert_eq!(fields, vec![&b""[..], &b"a"[..], &b"b"[..]]);
    }

    #[test]
    fn test_split_trailing_delimiters() {
        let re = Regex::new(r"[ \t]+").unwrap();
        let fields: Vec<&[u8]> = re.split(b"a b    ").collect();
        assert_eq!(fields, vec![&b"a"[..], &b"b"[..], &b""[..]]);
    }
}

#[cfg(test)]
mod tests_llm_16_568 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_splitn_basic() {
        let re = Regex::new(r"\W+").unwrap();
        let fields: Vec<&[u8]> = re.splitn(b"Hey! How are you?", 3).collect();
        assert_eq!(fields, vec![&b"Hey"[..], &b"How"[..], &b"are you?"[..]]);
    }

    #[test]
    fn test_splitn_limit_zero() {
        let re = Regex::new(r"\W+").unwrap();
        let fields: Vec<&[u8]> = re.splitn(b"Hey! How are you?", 0).collect();
        assert_eq!(fields, Vec::<&[u8]>::new());
    }

    #[test]
    fn test_splitn_limit_one() {
        let re = Regex::new(r"\W+").unwrap();
        let fields: Vec<&[u8]> = re.splitn(b"Hey! How are you?", 1).collect();
        assert_eq!(fields, vec![&b"Hey! How are you?"[..]]);
    }

    #[test]
    fn test_splitn_limit_two() {
        let re = Regex::new(r"\W+").unwrap();
        let fields: Vec<&[u8]> = re.splitn(b"Hey, How are you?", 2).collect();
        assert_eq!(fields, vec![&b"Hey"[..], &b"How are you?"[..]]);
    }

    #[test]
    fn test_splitn_limit_exceed() {
        let re = Regex::new(r"\W+").unwrap();
        let fields: Vec<&[u8]> = re.splitn(b"Hey! How are you?", 10).collect();
        assert_eq!(fields, vec![&b"Hey"[..], &b"How"[..], &b"are"[..], &b"you?"[..]]);
    }
}

#[cfg(test)]
mod tests_llm_16_569 {
    use super::*;

use crate::*;
    use crate::bytes::Regex;

    #[test]
    fn test_static_captures_len() {
        let len = |pattern| {
            Regex::new(pattern).map(|re| re.static_captures_len())
        };

        assert_eq!(Some(1), len("a").unwrap());
        assert_eq!(Some(2), len("(a)").unwrap());
        assert_eq!(Some(2), len("(a)|(b)").unwrap());
        assert_eq!(Some(3), len("(a)(b)|(c)(d)").unwrap());
        assert_eq!(None, len("(a)|b").unwrap());
        assert_eq!(None, len("a|(b)").unwrap());
        assert_eq!(None, len("(b)*").unwrap());
        assert_eq!(Some(2), len("(b)+").unwrap());
    }
}
