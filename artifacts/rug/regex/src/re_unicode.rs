use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::iter::FusedIterator;
use std::ops::{Index, Range};
use std::str::FromStr;
use std::sync::Arc;

use crate::find_byte::find_byte;

use crate::error::Error;
use crate::exec::{Exec, ExecNoSyncStr};
use crate::expand::expand_str;
use crate::re_builder::unicode::RegexBuilder;
use crate::re_trait::{self, RegularExpression, SubCapturesPosIter};

/// Escapes all regular expression meta characters in `text`.
///
/// The string returned may be safely used as a literal in a regular
/// expression.
pub fn escape(text: &str) -> String {
    regex_syntax::escape(text)
}

/// Match represents a single match of a regex in a haystack.
///
/// The lifetime parameter `'t` refers to the lifetime of the matched text.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'t> {
    text: &'t str,
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
    pub fn as_str(&self) -> &'t str {
        &self.text[self.range()]
    }

    /// Creates a new match from the given haystack and byte offsets.
    #[inline]
    fn new(haystack: &'t str, start: usize, end: usize) -> Match<'t> {
        Match { text: haystack, start, end }
    }
}

impl<'t> std::fmt::Debug for Match<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Match")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("string", &self.as_str())
            .finish()
    }
}

impl<'t> From<Match<'t>> for &'t str {
    fn from(m: Match<'t>) -> &'t str {
        m.as_str()
    }
}

impl<'t> From<Match<'t>> for Range<usize> {
    fn from(m: Match<'t>) -> Range<usize> {
        m.range()
    }
}

/// A compiled regular expression for matching Unicode strings.
///
/// It is represented as either a sequence of bytecode instructions (dynamic)
/// or as a specialized Rust function (native). It can be used to search, split
/// or replace text. All searching is done with an implicit `.*?` at the
/// beginning and end of an expression. To force an expression to match the
/// whole string (or a prefix or a suffix), you must use an anchor like `^` or
/// `$` (or `\A` and `\z`).
///
/// While this crate will handle Unicode strings (whether in the regular
/// expression or in the search text), all positions returned are **byte
/// indices**. Every byte index is guaranteed to be at a Unicode code point
/// boundary.
///
/// The lifetimes `'r` and `'t` in this crate correspond to the lifetime of a
/// compiled regular expression and text to search, respectively.
///
/// The only methods that allocate new strings are the string replacement
/// methods. All other methods (searching and splitting) return borrowed
/// pointers into the string given.
///
/// # Examples
///
/// Find the location of a US phone number:
///
/// ```rust
/// # use regex::Regex;
/// let re = Regex::new("[0-9]{3}-[0-9]{3}-[0-9]{4}").unwrap();
/// let mat = re.find("phone: 111-222-3333").unwrap();
/// assert_eq!((mat.start(), mat.end()), (7, 19));
/// ```
///
/// # Using the `std::str::pattern` methods with `Regex`
///
/// > **Note**: This section requires that this crate is compiled with the
/// > `pattern` Cargo feature enabled, which **requires nightly Rust**.
///
/// Since `Regex` implements `Pattern`, you can use regexes with methods
/// defined on `&str`. For example, `is_match`, `find`, `find_iter`
/// and `split` can be replaced with `str::contains`, `str::find`,
/// `str::match_indices` and `str::split`.
///
/// Here are some examples:
///
/// ```rust,ignore
/// # use regex::Regex;
/// let re = Regex::new(r"\d+").unwrap();
/// let haystack = "a111b222c";
///
/// assert!(haystack.contains(&re));
/// assert_eq!(haystack.find(&re), Some(1));
/// assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
///            vec![(1, "111"), (5, "222")]);
/// assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);
/// ```
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
    /// Test if some text contains at least one word with exactly 13
    /// Unicode word characters:
    ///
    /// ```rust
    /// # use regex::Regex;
    /// # fn main() {
    /// let text = "I categorically deny having triskaidekaphobia.";
    /// assert!(Regex::new(r"\b\w{13}\b").unwrap().is_match(text));
    /// # }
    /// ```
    pub fn is_match(&self, text: &str) -> bool {
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
    /// Unicode word characters:
    ///
    /// ```rust
    /// # use regex::Regex;
    /// # fn main() {
    /// let text = "I categorically deny having triskaidekaphobia.";
    /// let mat = Regex::new(r"\b\w{13}\b").unwrap().find(text).unwrap();
    /// assert_eq!(mat.start(), 2);
    /// assert_eq!(mat.end(), 15);
    /// # }
    /// ```
    pub fn find<'t>(&self, text: &'t str) -> Option<Match<'t>> {
        self.find_at(text, 0)
    }

    /// Returns an iterator for each successive non-overlapping match in
    /// `text`, returning the start and end byte indices with respect to
    /// `text`.
    ///
    /// # Example
    ///
    /// Find the start and end location of every word with exactly 13 Unicode
    /// word characters:
    ///
    /// ```rust
    /// # use regex::Regex;
    /// # fn main() {
    /// let text = "Retroactively relinquishing remunerations is reprehensible.";
    /// for mat in Regex::new(r"\b\w{13}\b").unwrap().find_iter(text) {
    ///     println!("{:?}", mat);
    /// }
    /// # }
    /// ```
    pub fn find_iter<'r, 't>(&'r self, text: &'t str) -> Matches<'r, 't> {
        Matches(self.0.searcher_str().find_iter(text))
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
    /// # use regex::Regex;
    /// # fn main() {
    /// let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    /// let text = "Not my favorite movie: 'Citizen Kane' (1941).";
    /// let caps = re.captures(text).unwrap();
    /// assert_eq!(caps.get(1).unwrap().as_str(), "Citizen Kane");
    /// assert_eq!(caps.get(2).unwrap().as_str(), "1941");
    /// assert_eq!(caps.get(0).unwrap().as_str(), "'Citizen Kane' (1941)");
    /// // You can also access the groups by index using the Index notation.
    /// // Note that this will panic on an invalid index.
    /// assert_eq!(&caps[1], "Citizen Kane");
    /// assert_eq!(&caps[2], "1941");
    /// assert_eq!(&caps[0], "'Citizen Kane' (1941)");
    /// # }
    /// ```
    ///
    /// Note that the full match is at capture group `0`. Each subsequent
    /// capture group is indexed by the order of its opening `(`.
    ///
    /// We can make this example a bit clearer by using *named* capture groups:
    ///
    /// ```rust
    /// # use regex::Regex;
    /// # fn main() {
    /// let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)")
    ///                .unwrap();
    /// let text = "Not my favorite movie: 'Citizen Kane' (1941).";
    /// let caps = re.captures(text).unwrap();
    /// assert_eq!(caps.name("title").unwrap().as_str(), "Citizen Kane");
    /// assert_eq!(caps.name("year").unwrap().as_str(), "1941");
    /// assert_eq!(caps.get(0).unwrap().as_str(), "'Citizen Kane' (1941)");
    /// // You can also access the groups by name using the Index notation.
    /// // Note that this will panic on an invalid group name.
    /// assert_eq!(&caps["title"], "Citizen Kane");
    /// assert_eq!(&caps["year"], "1941");
    /// assert_eq!(&caps[0], "'Citizen Kane' (1941)");
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
    pub fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>> {
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
    /// # use regex::Regex;
    /// # fn main() {
    /// let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)")
    ///                .unwrap();
    /// let text = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";
    /// for caps in re.captures_iter(text) {
    ///     println!("Movie: {:?}, Released: {:?}",
    ///              &caps["title"], &caps["year"]);
    /// }
    /// // Output:
    /// // Movie: Citizen Kane, Released: 1941
    /// // Movie: The Wizard of Oz, Released: 1939
    /// // Movie: M, Released: 1931
    /// # }
    /// ```
    pub fn captures_iter<'r, 't>(
        &'r self,
        text: &'t str,
    ) -> CaptureMatches<'r, 't> {
        CaptureMatches(self.0.searcher_str().captures_iter(text))
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
    /// # use regex::Regex;
    /// # fn main() {
    /// let re = Regex::new(r"[ \t]+").unwrap();
    /// let fields: Vec<&str> = re.split("a b \t  c\td    e").collect();
    /// assert_eq!(fields, vec!["a", "b", "c", "d", "e"]);
    /// # }
    /// ```
    pub fn split<'r, 't>(&'r self, text: &'t str) -> Split<'r, 't> {
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
    /// # use regex::Regex;
    /// # fn main() {
    /// let re = Regex::new(r"\W+").unwrap();
    /// let fields: Vec<&str> = re.splitn("Hey! How are you?", 3).collect();
    /// assert_eq!(fields, vec!("Hey", "How", "are you?"));
    /// # }
    /// ```
    pub fn splitn<'r, 't>(
        &'r self,
        text: &'t str,
        limit: usize,
    ) -> SplitN<'r, 't> {
        SplitN { splits: self.split(text), n: limit }
    }

    /// Replaces the leftmost-first match with the replacement provided.
    /// The replacement can be a regular string (where `$N` and `$name` are
    /// expanded to match capture groups) or a function that takes the matches'
    /// `Captures` and returns the replaced string.
    ///
    /// If no match is found, then a copy of the string is returned unchanged.
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
    /// In typical usage, this can just be a normal string:
    ///
    /// ```rust
    /// # use regex::Regex;
    /// # fn main() {
    /// let re = Regex::new("[^01]+").unwrap();
    /// assert_eq!(re.replace("1078910", ""), "1010");
    /// # }
    /// ```
    ///
    /// But anything satisfying the `Replacer` trait will work. For example,
    /// a closure of type `|&Captures| -> String` provides direct access to the
    /// captures corresponding to a match. This allows one to access
    /// capturing group matches easily:
    ///
    /// ```rust
    /// # use regex::Regex;
    /// # use regex::Captures; fn main() {
    /// let re = Regex::new(r"([^,\s]+),\s+(\S+)").unwrap();
    /// let result = re.replace("Springsteen, Bruce", |caps: &Captures| {
    ///     format!("{} {}", &caps[2], &caps[1])
    /// });
    /// assert_eq!(result, "Bruce Springsteen");
    /// # }
    /// ```
    ///
    /// But this is a bit cumbersome to use all the time. Instead, a simple
    /// syntax is supported that expands `$name` into the corresponding capture
    /// group. Here's the last example, but using this expansion technique
    /// with named capture groups:
    ///
    /// ```rust
    /// # use regex::Regex;
    /// # fn main() {
    /// let re = Regex::new(r"(?P<last>[^,\s]+),\s+(?P<first>\S+)").unwrap();
    /// let result = re.replace("Springsteen, Bruce", "$first $last");
    /// assert_eq!(result, "Bruce Springsteen");
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
    /// # use regex::Regex;
    /// # fn main() {
    /// let re = Regex::new(r"(?P<first>\w+)\s+(?P<second>\w+)").unwrap();
    /// let result = re.replace("deep fried", "${first}_$second");
    /// assert_eq!(result, "deep_fried");
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
    /// # use regex::Regex;
    /// # fn main() {
    /// use regex::NoExpand;
    ///
    /// let re = Regex::new(r"(?P<last>[^,\s]+),\s+(\S+)").unwrap();
    /// let result = re.replace("Springsteen, Bruce", NoExpand("$2 $last"));
    /// assert_eq!(result, "$2 $last");
    /// # }
    /// ```
    pub fn replace<'t, R: Replacer>(
        &self,
        text: &'t str,
        rep: R,
    ) -> Cow<'t, str> {
        self.replacen(text, 1, rep)
    }

    /// Replaces all non-overlapping matches in `text` with the replacement
    /// provided. This is the same as calling `replacen` with `limit` set to
    /// `0`.
    ///
    /// See the documentation for `replace` for details on how to access
    /// capturing group matches in the replacement string.
    pub fn replace_all<'t, R: Replacer>(
        &self,
        text: &'t str,
        rep: R,
    ) -> Cow<'t, str> {
        self.replacen(text, 0, rep)
    }

    /// Replaces at most `limit` non-overlapping matches in `text` with the
    /// replacement provided. If `limit` is 0, then all non-overlapping matches
    /// are replaced.
    ///
    /// See the documentation for `replace` for details on how to access
    /// capturing group matches in the replacement string.
    pub fn replacen<'t, R: Replacer>(
        &self,
        text: &'t str,
        limit: usize,
        mut rep: R,
    ) -> Cow<'t, str> {
        // If we know that the replacement doesn't have any capture expansions,
        // then we can use the fast path. The fast path can make a tremendous
        // difference:
        //
        //   1) We use `find_iter` instead of `captures_iter`. Not asking for
        //      captures generally makes the regex engines faster.
        //   2) We don't need to look up all of the capture groups and do
        //      replacements inside the replacement string. We just push it
        //      at each match and be done with it.
        if let Some(rep) = rep.no_expansion() {
            let mut it = self.find_iter(text).enumerate().peekable();
            if it.peek().is_none() {
                return Cow::Borrowed(text);
            }
            let mut new = String::with_capacity(text.len());
            let mut last_match = 0;
            for (i, m) in it {
                new.push_str(&text[last_match..m.start()]);
                new.push_str(&rep);
                last_match = m.end();
                if limit > 0 && i >= limit - 1 {
                    break;
                }
            }
            new.push_str(&text[last_match..]);
            return Cow::Owned(new);
        }

        // The slower path, which we use if the replacement needs access to
        // capture groups.
        let mut it = self.captures_iter(text).enumerate().peekable();
        if it.peek().is_none() {
            return Cow::Borrowed(text);
        }
        let mut new = String::with_capacity(text.len());
        let mut last_match = 0;
        for (i, cap) in it {
            // unwrap on 0 is OK because captures only reports matches
            let m = cap.get(0).unwrap();
            new.push_str(&text[last_match..m.start()]);
            rep.replace_append(&cap, &mut new);
            last_match = m.end();
            if limit > 0 && i >= limit - 1 {
                break;
            }
        }
        new.push_str(&text[last_match..]);
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
    /// # use regex::Regex;
    /// # fn main() {
    /// let text = "aaaaa";
    /// let pos = Regex::new(r"a+").unwrap().shortest_match(text);
    /// assert_eq!(pos, Some(1));
    /// # }
    /// ```
    pub fn shortest_match(&self, text: &str) -> Option<usize> {
        self.shortest_match_at(text, 0)
    }

    /// Returns the same as `shortest_match`, but starts the search at the
    /// given offset.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only match
    /// when `start == 0`.
    pub fn shortest_match_at(
        &self,
        text: &str,
        start: usize,
    ) -> Option<usize> {
        self.0.searcher_str().shortest_match_at(text, start)
    }

    /// Returns the same as is_match, but starts the search at the given
    /// offset.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only
    /// match when `start == 0`.
    pub fn is_match_at(&self, text: &str, start: usize) -> bool {
        self.0.searcher_str().is_match_at(text, start)
    }

    /// Returns the same as find, but starts the search at the given
    /// offset.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only
    /// match when `start == 0`.
    pub fn find_at<'t>(
        &self,
        text: &'t str,
        start: usize,
    ) -> Option<Match<'t>> {
        self.0
            .searcher_str()
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
        text: &'t str,
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
        text: &'t str,
    ) -> Option<Match<'t>> {
        self.captures_read_at(locs, text, 0)
    }

    /// Returns the same as captures, but starts the search at the given
    /// offset and populates the capture locations given.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only
    /// match when `start == 0`.
    pub fn captures_read_at<'t>(
        &self,
        locs: &mut CaptureLocations,
        text: &'t str,
        start: usize,
    ) -> Option<Match<'t>> {
        self.0
            .searcher_str()
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
        text: &'t str,
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
    /// use regex::Regex;
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
        CaptureLocations(self.0.searcher_str().locations())
    }

    /// An alias for `capture_locations` to preserve backward compatibility.
    ///
    /// The `regex-capi` crate uses this method, so to avoid breaking that
    /// crate, we continue to export it as an undocumented API.
    #[doc(hidden)]
    pub fn locations(&self) -> CaptureLocations {
        CaptureLocations(self.0.searcher_str().locations())
    }
}

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

/// Yields all substrings delimited by a regular expression match.
///
/// `'r` is the lifetime of the compiled regular expression and `'t` is the
/// lifetime of the string being split.
#[derive(Debug)]
pub struct Split<'r, 't> {
    finder: Matches<'r, 't>,
    last: usize,
}

impl<'r, 't> Iterator for Split<'r, 't> {
    type Item = &'t str;

    fn next(&mut self) -> Option<&'t str> {
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
/// lifetime of the string being split.
#[derive(Debug)]
pub struct SplitN<'r, 't> {
    splits: Split<'r, 't>,
    n: usize,
}

impl<'r, 't> Iterator for SplitN<'r, 't> {
    type Item = &'t str;

    fn next(&mut self) -> Option<&'t str> {
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
/// use regex::Regex;
///
/// let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
/// let mut locs = re.capture_locations();
/// let m = re.captures_read(&mut locs, "Bruce Springsteen").unwrap();
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

/// Captures represents a group of captured strings for a single match.
///
/// The 0th capture always corresponds to the entire match. Each subsequent
/// index corresponds to the next capture group in the regex. If a capture
/// group is named, then the matched string is *also* available via the `name`
/// method. (Note that the 0th capture is always unnamed and so must be
/// accessed with the `get` method.)
///
/// Positions returned from a capture group are always byte indices.
///
/// `'t` is the lifetime of the matched text.
pub struct Captures<'t> {
    text: &'t str,
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
    /// # use regex::Regex;
    /// let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    /// let caps = re.captures("abc123").unwrap();
    ///
    /// let text1 = caps.get(1).map_or("", |m| m.as_str());
    /// let text2 = caps.get(2).map_or("", |m| m.as_str());
    /// assert_eq!(text1, "123");
    /// assert_eq!(text2, "");
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
    /// using braces, any sequence of characters is permitted. If the sequence
    /// does not refer to a capture group name in the corresponding regex, then
    /// it is replaced with an empty string.
    ///
    /// To write a literal `$` use `$$`.
    pub fn expand(&self, replacement: &str, dst: &mut String) {
        expand_str(self, replacement, dst)
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
        // We'd like to show something nice here, even if it means an
        // allocation to build a reverse index.
        let slot_to_name: HashMap<&usize, &String> =
            self.0.named_groups.iter().map(|(a, b)| (b, a)).collect();
        let mut map = f.debug_map();
        for (slot, m) in self.0.locs.iter().enumerate() {
            let m = m.map(|(s, e)| &self.0.text[s..e]);
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
    type Output = str;

    fn index(&self, i: usize) -> &str {
        self.get(i)
            .map(|m| m.as_str())
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
    type Output = str;

    fn index<'a>(&'a self, name: &'i str) -> &'a str {
        self.name(name)
            .map(|m| m.as_str())
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }

    fn count(self) -> usize {
        self.it.count()
    }
}

impl<'c, 't> ExactSizeIterator for SubCaptureMatches<'c, 't> {}

impl<'c, 't> FusedIterator for SubCaptureMatches<'c, 't> {}

/// An iterator that yields all non-overlapping capture groups matching a
/// particular regular expression.
///
/// The iterator stops when no more matches can be found.
///
/// `'r` is the lifetime of the compiled regular expression and `'t` is the
/// lifetime of the matched string.
#[derive(Debug)]
pub struct CaptureMatches<'r, 't>(
    re_trait::CaptureMatches<'t, ExecNoSyncStr<'r>>,
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

/// An iterator over all non-overlapping matches for a particular string.
///
/// The iterator yields a `Match` value. The iterator stops when no more
/// matches can be found.
///
/// `'r` is the lifetime of the compiled regular expression and `'t` is the
/// lifetime of the matched string.
#[derive(Debug)]
pub struct Matches<'r, 't>(re_trait::Matches<'t, ExecNoSyncStr<'r>>);

impl<'r, 't> Iterator for Matches<'r, 't> {
    type Item = Match<'t>;

    fn next(&mut self) -> Option<Match<'t>> {
        let text = self.0.text();
        self.0.next().map(|(s, e)| Match::new(text, s, e))
    }
}

impl<'r, 't> FusedIterator for Matches<'r, 't> {}

/// Replacer describes types that can be used to replace matches in a string.
///
/// In general, users of this crate shouldn't need to implement this trait,
/// since implementations are already provided for `&str` along with other
/// variants of string types and `FnMut(&Captures) -> String` (or any
/// `FnMut(&Captures) -> T` where `T: AsRef<str>`), which covers most use cases.
pub trait Replacer {
    /// Appends text to `dst` to replace the current match.
    ///
    /// The current match is represented by `caps`, which is guaranteed to
    /// have a match at capture group `0`.
    ///
    /// For example, a no-op replacement would be
    /// `dst.push_str(caps.get(0).unwrap().as_str())`.
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String);

    /// Return a fixed unchanging replacement string.
    ///
    /// When doing replacements, if access to `Captures` is not needed (e.g.,
    /// the replacement byte string does not need `$` expansion), then it can
    /// be beneficial to avoid finding sub-captures.
    ///
    /// In general, this is called once for every call to `replacen`.
    fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>> {
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
    /// use regex::{Regex, Replacer};
    ///
    /// fn replace_all_twice<R: Replacer>(
    ///     re: Regex,
    ///     src: &str,
    ///     mut rep: R,
    /// ) -> String {
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
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        self.0.replace_append(caps, dst)
    }
    fn no_expansion(&mut self) -> Option<Cow<'_, str>> {
        self.0.no_expansion()
    }
}

impl<'a> Replacer for &'a str {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        caps.expand(*self, dst);
    }

    fn no_expansion(&mut self) -> Option<Cow<'_, str>> {
        no_expansion(self)
    }
}

impl<'a> Replacer for &'a String {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        self.as_str().replace_append(caps, dst)
    }

    fn no_expansion(&mut self) -> Option<Cow<'_, str>> {
        no_expansion(self)
    }
}

impl Replacer for String {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        self.as_str().replace_append(caps, dst)
    }

    fn no_expansion(&mut self) -> Option<Cow<'_, str>> {
        no_expansion(self)
    }
}

impl<'a> Replacer for Cow<'a, str> {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        self.as_ref().replace_append(caps, dst)
    }

    fn no_expansion(&mut self) -> Option<Cow<'_, str>> {
        no_expansion(self)
    }
}

impl<'a> Replacer for &'a Cow<'a, str> {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        self.as_ref().replace_append(caps, dst)
    }

    fn no_expansion(&mut self) -> Option<Cow<'_, str>> {
        no_expansion(self)
    }
}

fn no_expansion<T: AsRef<str>>(t: &T) -> Option<Cow<'_, str>> {
    let s = t.as_ref();
    match find_byte(b'$', s.as_bytes()) {
        Some(_) => None,
        None => Some(Cow::Borrowed(s)),
    }
}

impl<F, T> Replacer for F
where
    F: FnMut(&Captures<'_>) -> T,
    T: AsRef<str>,
{
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        dst.push_str((*self)(caps).as_ref());
    }
}

/// `NoExpand` indicates literal string replacement.
///
/// It can be used with `replace` and `replace_all` to do a literal string
/// replacement without expanding `$name` to their corresponding capture
/// groups. This can be both convenient (to avoid escaping `$`, for example)
/// and performant (since capture groups don't need to be found).
///
/// `'t` is the lifetime of the literal text.
#[derive(Clone, Debug)]
pub struct NoExpand<'t>(pub &'t str);

impl<'t> Replacer for NoExpand<'t> {
    fn replace_append(&mut self, _: &Captures<'_>, dst: &mut String) {
        dst.push_str(self.0);
    }

    fn no_expansion(&mut self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed(self.0))
    }
}

#[cfg(test)]
mod tests_llm_16_65 {
    use super::*;

use crate::*;
    use std::borrow::Cow;
    use crate::Regex;
    
    #[test]
    fn test_no_expansion() {
        let input: Cow<str> = Cow::Borrowed("test");
        let mut replacer = input.clone(); // Clone to use as mutable
        let result = replacer.no_expansion();
        assert_eq!(result, Some(Cow::Borrowed("test")));
    }
    
    #[test]
    fn test_no_expansion_empty() {
        let input: Cow<str> = Cow::Borrowed("");
        let mut replacer = input.clone(); // Clone to use as mutable
        let result = replacer.no_expansion();
        assert_eq!(result, Some(Cow::Borrowed("")));
    }
}

#[cfg(test)]
mod tests_llm_16_66 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_replace_append() {
        let re = Regex::new(r"(?P<digit>\d+)").unwrap();
        let text = "Number 42";
        let caps = re.captures(text).unwrap();
        
        let mut dst = String::new();
        let mut replacer = std::borrow::Cow::from("$digit");

        replacer.replace_append(&caps, &mut dst);

        assert_eq!(dst, "42");
    }

    #[test]
    fn test_replace_append_no_match() {
        let re = Regex::new(r"(?P<digit>\d+)").unwrap();
        let text = "Hello World";
        let caps = re.captures(text).unwrap();
        
        let mut dst = String::new();
        let mut replacer = std::borrow::Cow::from("$digit");

        replacer.replace_append(&caps, &mut dst);

        assert_eq!(dst, ""); // No digits, dst should remain empty
    }

    #[test]
    fn test_replace_append_empty_input() {
        let re = Regex::new(r"(?P<digit>\d+)").unwrap();
        let text = "";
        let caps = re.captures(text).unwrap();
        
        let mut dst = String::new();
        let mut replacer = std::borrow::Cow::from("$digit");

        replacer.replace_append(&caps, &mut dst);

        assert_eq!(dst, ""); // No input, dst should remain empty
    }
}

#[cfg(test)]
mod tests_llm_16_67 {
    use super::*;

use crate::*;
    use crate::Regex;
    use std::borrow::Cow;

    #[test]
    fn test_no_expansion() {
        let mut s = String::from("test");
        let result = s.no_expansion();
        assert_eq!(result, Some(Cow::Borrowed("test")));
    }
}

#[cfg(test)]
mod tests_llm_16_68 {
    use super::*; // Ensure we have access to the necessary types

use crate::*;
    use crate::Regex; // Import the regex crate
    use std::string::String;

    #[test]
    fn test_replace_append() {
        let re = Regex::new(r"(\d+)").unwrap();
        let caps = re.captures("abc123").unwrap();
        
        let mut dst = String::new();
        
        let mut replacer = String::from("Value: $1");
        replacer.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, "Value: 123");
    }
    
    #[test]
    fn test_replace_append_no_match() {
        let re = Regex::new(r"(\d+)").unwrap();
        let caps = re.captures("abc").unwrap();
        
        let mut dst = String::new();
        
        let mut replacer = String::from("Value: $1");
        replacer.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, "Value: ");
    }
}

#[cfg(test)]
mod tests_llm_16_71 {
    use super::*;

use crate::*;
    use std::borrow::Cow;

    #[test]
    fn test_no_expansion() {
        let mut input: &str = "test";
        let mut replacer = input; // Assuming `replacer` is instantiated properly
        
        let result: Option<Cow<'_, str>> = replacer.no_expansion();
        assert_eq!(result, Some(Cow::Borrowed("test")));
    }
}

#[cfg(test)]
mod tests_llm_16_72 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_replace_append() {
        let re = Regex::new(r"(?P<word>\w+)").unwrap();
        let caps = re.captures("hello").unwrap();
        let mut dst = String::new();
        let mut replacer = String::from("$word");
        
        replacer.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, "hello");
    }

    #[test]
    fn test_replace_append_with_no_capture() {
        let re = Regex::new(r"(?P<word>\w+)").unwrap();
        let caps = re.captures("123").unwrap();
        let mut dst = String::new();
        let mut replacer = String::from("$word");
        
        replacer.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, "123");
    }

    #[test]
    fn test_replace_append_with_undefined_capture() {
        let re = Regex::new(r"(?P<word>\w+)").unwrap();
        let caps = re.captures("foo").unwrap();
        let mut dst = String::new();
        let mut replacer = String::from("$undefined");
        
        replacer.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, "");
    }
}

#[cfg(test)]
mod tests_llm_16_74 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_replace_append_with_string() {
        let re = Regex::new(r"(?P<num>\d+)").unwrap();
        let caps = re.captures("abc123").unwrap();
        let mut dst = String::new();
        let mut replacer = String::from("Number: $num");

        replacer.replace_append(&caps, &mut dst);
        assert_eq!(dst, "Number: 123");
    }

    #[test]
    fn test_replace_append_with_empty_capture() {
        let re = Regex::new(r"(?P<num>\d+)").unwrap();
        let caps = re.captures("abc").unwrap();
        let mut dst = String::new();
        let mut replacer = String::from("Number: $num");

        replacer.replace_append(&caps, &mut dst);
        assert_eq!(dst, "Number: ");
    }

    #[test]
    fn test_replace_append_with_no_capture() {
        let re = Regex::new(r"(?P<num>\d+)").unwrap();
        let caps = re.captures("abc").unwrap();
        let mut dst = String::new();
        let mut replacer = String::from("No match: $no_capture");

        replacer.replace_append(&caps, &mut dst);
        assert_eq!(dst, "No match: ");
    }
}

#[cfg(test)]
mod tests_llm_16_177 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_capture_matches_next() {
        let regex = Regex::new(r"(?P<name>\w+)").unwrap();
        let text = "Alice and Bob";
        let mut captures = regex.captures_iter(text);

        // first match
        let first = captures.next();
        assert!(first.is_some());
        let first_capture = first.unwrap();
        assert_eq!(first_capture.name("name").unwrap().as_str(), "Alice");

        // second match
        let second = captures.next();
        assert!(second.is_some());
        let second_capture = second.unwrap();
        assert_eq!(second_capture.name("name").unwrap().as_str(), "Bob");

        // no more matches
        let third = captures.next();
        assert!(third.is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_178 {
    use super::*;

use crate::*;
    use std::slice;

    #[test]
    fn test_count() {
        let names: Vec<Option<String>> = vec![None, Some("first".to_string()), Some("second".to_string())];
        let capture_names = CaptureNames(names.iter());

        assert_eq!(capture_names.count(), 3);
    }
}

#[cfg(test)]
mod tests_llm_16_180 {
    use super::*;

use crate::*;
    use std::slice;

    #[test]
    fn test_size_hint_empty() {
        let captures: &[Option<String>] = &[];
        let iter = CaptureNames(captures.iter());
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_size_hint_some() {
        let captures: &[Option<String>] = &[Some("first".to_string()), Some("second".to_string())];
        let iter = CaptureNames(captures.iter());
        assert_eq!(iter.size_hint(), (2, Some(2)));
    }

    #[test]
    fn test_size_hint_none() {
        let captures: &[Option<String>] = &[None, Some("second".to_string())];
        let iter = CaptureNames(captures.iter());
        assert_eq!(iter.size_hint(), (2, Some(2)));
    }
}

#[cfg(test)]
mod tests_llm_16_183 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_index_valid() {
        let re = Regex::new(r"(\w+) (\d+)").unwrap();
        let caps = re.captures("hello 123").unwrap();
        
        assert_eq!(caps.index(0), "hello 123"); // entire match
        assert_eq!(caps.index(1), "hello");     // first capture group
        assert_eq!(caps.index(2), "123");       // second capture group
    }

    #[test]
    #[should_panic(expected = "no group at index '3'")]
    fn test_index_invalid() {
        let re = Regex::new(r"(\w+) (\d+)").unwrap();
        let caps = re.captures("hello 123").unwrap();
        
        caps.index(3); // index out of range
    }

    #[test]
    #[should_panic(expected = "no group at index '1'")]
    fn test_index_empty_capture() {
        let re = Regex::new(r"(\w+)").unwrap();
        let caps = re.captures("hello").unwrap();
        
        assert_eq!(caps.index(0), "hello"); // valid
        caps.index(1); // index 1 does not exist
    }
}

#[cfg(test)]
mod tests_llm_16_187 {
    use super::*; // Ensure to use the correct module scope

use crate::*;
    use std::borrow::Cow;

    #[test]
    fn test_no_expansion() {
        let literal = "test_string";
        let mut no_expand = NoExpand(literal);

        let result = no_expand.no_expansion();
        assert_eq!(result, Some(Cow::Borrowed(literal)));
    }
}

#[cfg(test)]
mod tests_llm_16_192 {
    use super::*;

use crate::*;
    use crate::Regex;
    use std::str::FromStr;

    #[test]
    fn test_from_str_valid_pattern() {
        let pattern = r"^[a-z]+$";
        let regex = re_unicode::Regex::from_str(pattern);
        assert!(regex.is_ok());
    }

    #[test]
    fn test_from_str_invalid_pattern() {
        let pattern = r"^[a-z+";
        let regex = re_unicode::Regex::from_str(pattern);
        assert!(regex.is_err());
    }

    #[test]
    fn test_from_str_empty_pattern() {
        let pattern = "";
        let regex = re_unicode::Regex::from_str(pattern);
        assert!(regex.is_ok());
    }
}

#[cfg(test)]
mod tests_llm_16_193 {
    use super::*;

use crate::*;
    use std::borrow::Cow;

    struct MockReplacer {
        expansion: Option<Cow<'static, str>>,
    }

    impl MockReplacer {
        fn new(expansion: Option<Cow<'static, str>>) -> Self {
            MockReplacer { expansion }
        }
    }

    impl re_unicode::Replacer for MockReplacer {
        fn replace_append(&mut self, _caps: &crate::Captures<'_>, _dst: &mut String) {
            // Mock implementation
        }

        fn no_expansion(&mut self) -> Option<Cow<'_, str>> {
            self.expansion.clone()
        }
    }

    #[test]
    fn test_no_expansion_some() {
        let mut mock = MockReplacer::new(Some(Cow::Borrowed("test")));
        let mut replacer_ref = re_unicode::ReplacerRef(&mut mock);
        assert_eq!(replacer_ref.no_expansion(), Some(Cow::Borrowed("test")));
    }

    #[test]
    fn test_no_expansion_none() {
        let mut mock = MockReplacer::new(None);
        let mut replacer_ref = re_unicode::ReplacerRef(&mut mock);
        assert_eq!(replacer_ref.no_expansion(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_198 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_sub_capture_matches_count() {
        let re = Regex::new(r"(\d+)(\w+)").unwrap();
        let text = "42abc";
        let caps = re.captures(text).unwrap();
        let sub_captures = caps.iter();

        assert_eq!(sub_captures.count(), 3); // 0: "42abc", 1: "42", 2: "abc"
    }

    #[test]
    fn test_sub_capture_matches_count_empty() {
        let re = Regex::new(r"(\d+)(\w+)").unwrap();
        let text = "abc";
        let caps = re.captures(text).unwrap();
        let sub_captures = caps.iter();

        assert_eq!(sub_captures.count(), 3); // 0: "abc", 1: None, 2: None
    }
}

#[cfg(test)]
mod tests_llm_16_200 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_size_hint() {
        let re = Regex::new(r"(a)(b)(c)").unwrap();
        let text = "abc";
        let caps = re.captures(text).unwrap();
        
        let sub_caps = caps.iter();
        let size_hint = sub_caps.size_hint();
        
        assert_eq!(size_hint, (3, Some(3))); // 3 capturing groups available
    }

    #[test]
    fn test_size_hint_with_non_matching() {
        let re = Regex::new(r"(a)(b)(c)?").unwrap();
        let text = "ab";
        let caps = re.captures(text).unwrap();

        let sub_caps = caps.iter();
        let size_hint = sub_caps.size_hint();
        
        assert_eq!(size_hint, (3, Some(3))); // 3 groups defined, but the last one may not match
    }

    #[test]
    fn test_size_hint_empty() {
        let re = Regex::new(r"(abc)").unwrap();
        let text = "";
        let caps = re.captures(text);
        
        assert!(caps.is_none()); // No matches in the empty text

        let empty_iter = caps.iter();
        let size_hint = empty_iter.size_hint();
        
        assert_eq!(size_hint, (1, Some(1))); // At least one capturing group (the full match group)
    }
}

#[cfg(test)]
mod tests_llm_16_205 {
    use super::*;

use crate::*;
    use std::borrow::Cow;
    use crate::bytes::Captures;

    #[test]
    fn test_no_expansion() {
        let mut cow_str: Cow<str> = Cow::Borrowed("test");
        let result = cow_str.no_expansion();
        assert_eq!(result, Some(Cow::Borrowed("test")));
    }

    #[test]
    fn test_no_expansion_with_empty() {
        let mut cow_empty: Cow<str> = Cow::Borrowed("");
        let result = cow_empty.no_expansion();
        assert_eq!(result, Some(Cow::Borrowed("")));
    }

    #[test]
    fn test_no_expansion_with_owned() {
        let mut cow_owned: Cow<str> = Cow::Owned("owned".to_string());
        let result = cow_owned.no_expansion();
        assert_eq!(result, Some(Cow::Owned("owned".to_string())));
    }
}

#[cfg(test)]
mod tests_llm_16_206 {
    use super::*;

use crate::*;
    use std::borrow::Cow;
    use crate::Regex;

    #[test]
    fn test_replace_append() {
        let re = Regex::new(r"(\d+)").unwrap();
        let text = "abc123xyz";
        let caps = re.captures(text).unwrap();
        
        let mut dst = String::new();
        
        let mut replacer: Cow<str> = Cow::Borrowed("number: $1");
        replacer.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, "number: 123");
    }

    #[test]
    fn test_replace_append_empty() {
        let re = Regex::new(r"(\d+)").unwrap();
        let text = "abcxyz";
        let caps = re.captures(text).unwrap();
        
        let mut dst = String::new();
        
        let mut replacer: Cow<str> = Cow::Borrowed("number: $1");
        replacer.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, "number: ");
    }

    #[test]
    fn test_replace_append_no_match() {
        let re = Regex::new(r"(\d+)").unwrap();
        let text = "abcxyz";
        let caps = re.captures(text).unwrap();
        
        let mut dst = String::new();
        
        let mut replacer: String = String::from("no match: $1");
        replacer.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, "no match: ");
    }

    #[test]
    fn test_replace_append_with_named_groups() {
        let re = Regex::new(r"(?P<digits>\d+)").unwrap();
        let text = "abc123xyz";
        let caps = re.captures(text).unwrap();
        
        let mut dst = String::new();
        
        let mut replacer: Cow<str> = Cow::Borrowed("number: ${digits}");
        replacer.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, "number: 123");
    }
}

#[cfg(test)]
mod tests_llm_16_207 {
    use super::*;

use crate::*;
    use crate::Regex;
    use std::borrow::Cow;

    #[test]
    fn test_no_expansion() {
        let mut my_string = String::from("test");
        let result: Option<Cow<str>> = my_string.no_expansion();
        assert_eq!(result, Some(Cow::Borrowed("test")));
    }
}

#[cfg(test)]
mod tests_llm_16_208 {
    use super::*;

use crate::*;
    use crate::Regex;
    use std::string::String;
    use re_unicode::Captures;
    
    #[test]
    fn test_replace_append() {
        let re = Regex::new(r"(\w+)\s(\d+)").unwrap();
        let text = "Hello 123";
        let caps = re.captures(text).unwrap();

        let mut dst = String::new();
        let mut replacer = String::from("$1$2");
        
        replacer.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, "Hello123");
    }

    #[test]
    fn test_replace_append_no_capture() {
        let re = Regex::new(r"(\w+)\s(\d+)").unwrap();
        let text = "NoMatch";
        let caps = re.captures(text).unwrap();

        let mut dst = String::new();
        let mut replacer = String::from("$1$2");
        
        replacer.replace_append(&caps, &mut dst);
        
        assert_eq!(dst, ""); // No match means nothing to append
    }
}

#[cfg(test)]
mod tests_llm_16_614 {
    use super::*;

use crate::*;
    use crate::re_unicode::Match;

    #[test]
    fn test_from() {
        let haystack = "hello world";
        let m = Match::new(haystack, 0, 5); // Match "hello"
        let result: &str = From::from(m);

        assert_eq!(result, "hello");
    }

    #[test]
    fn test_from_empty() {
        let haystack = "hello world";
        let m = Match::new(haystack, 5, 5); // Empty match
        let result: &str = From::from(m);

        assert_eq!(result, "");
    }

    #[test]
    fn test_from_partial_match() {
        let haystack = "hello world";
        let m = Match::new(haystack, 6, 11); // Match "world"
        let result: &str = From::from(m);

        assert_eq!(result, "world");
    }
}

#[cfg(test)]
mod tests_llm_16_615 {
    use super::*;

use crate::*;
    use std::ops::Range;

    #[test]
    fn test_from_match() {
        let haystack = "Hello, world!";
        let m = Match::new(haystack, 7, 12); // Matching "world"
        let range: Range<usize> = Range::from(m);
        
        assert_eq!(range.start, 7);
        assert_eq!(range.end, 12);
    }

    #[test]
    fn test_from_empty_match() {
        let haystack = "Hello, world!";
        let m = Match::new(haystack, 5, 5); // Matching an empty string
        let range: Range<usize> = Range::from(m);
        
        assert_eq!(range.start, 5);
        assert_eq!(range.end, 5);
    }
}

#[cfg(test)]
mod tests_llm_16_616 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_capture_locations_get() {
        let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let m = re.captures_read(&mut locs, "Bruce Springsteen").unwrap();

        assert_eq!(Some((0, 17)), locs.get(0)); // Entire match
        assert_eq!(Some((0, 5)), locs.get(1));  // First capture
        assert_eq!(Some((6, 17)), locs.get(2)); // Second capture
        assert_eq!(None, locs.get(3));           // Invalid capture group
    }

    #[test]
    fn test_capture_locations_get_invalid() {
        let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let _ = re.captures_read(&mut locs, "Test User").unwrap();

        assert_eq!(None, locs.get(3)); // Invalid capture group
        assert_eq!(None, locs.get(100)); // Extremely invalid capture group
    }
}

#[cfg(test)]
mod tests_llm_16_618 {
    use super::*;

use crate::*;
    use crate::Regex;
    use re_unicode::CaptureLocations;

    #[test]
    fn test_pos_valid_capture_groups() {
        let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let m = re.captures_read(&mut locs, "Bruce Springsteen").unwrap();

        assert_eq!(Some((0, 17)), locs.pos(0)); // Entire match
        assert_eq!(Some((0, 5)), locs.pos(1));  // First name
        assert_eq!(Some((6, 17)), locs.pos(2)); // Last name
    }

    #[test]
    fn test_pos_invalid_capture_groups() {
        let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let _ = re.captures_read(&mut locs, "Bruce Springsteen").unwrap();

        // Test for invalid capture group indices
        assert_eq!(None, locs.pos(3)); // Out of bounds
        assert_eq!(None, locs.pos(5)); // Out of bounds
    }

    #[test]
    fn test_pos_empty_match() {
        let re = Regex::new(r"\s+").unwrap();
        let mut locs = re.capture_locations();
        let m = re.captures_read(&mut locs, "    ").unwrap();

        assert_eq!(Some((0, 4)), locs.pos(0)); // Entire match
        assert_eq!(None, locs.pos(1)); // No additional groups
    }

    #[test]
    fn test_pos_with_non_matching_input() {
        let re = Regex::new(r"\d+").unwrap();
        let mut locs = re.capture_locations();
        let m = re.captures_read(&mut locs, "abc").unwrap();

        assert_eq!(Some((0, 3)), locs.pos(0)); // Entire match
        assert_eq!(None, locs.pos(1)); // No additional groups
    }
}

#[cfg(test)]
mod tests_llm_16_619 {
    use super::*;

use crate::*;
    use crate::Regex;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn test_expand_with_index() {
        let re = Regex::new(r"(?P<digit>\d)").unwrap();
        let text = "abc123";
        let caps = re.captures(text).unwrap();
        let mut result = String::new();
        caps.expand("$1", &mut result);
        assert_eq!(result, "1");
    }

    #[test]
    fn test_expand_with_named_group() {
        let re = Regex::new(r"(?P<digit>\d)").unwrap();
        let text = "abc123";
        let caps = re.captures(text).unwrap();
        let mut result = String::new();
        caps.expand("${digit}", &mut result);
        assert_eq!(result, "1");
    }

    #[test]
    fn test_expand_with_invalid_group() {
        let re = Regex::new(r"(?P<digit>\d)").unwrap();
        let text = "abc123";
        let caps = re.captures(text).unwrap();
        let mut result = String::new();
        caps.expand("$invalid", &mut result);
        assert_eq!(result, "");
    }

    #[test]
    fn test_expand_with_literal_dollar() {
        let re = Regex::new(r"abc").unwrap();
        let text = "abc";
        let caps = re.captures(text).unwrap();
        let mut result = String::new();
        caps.expand("$$", &mut result);
        assert_eq!(result, "$");
    }

    #[test]
    fn test_expand_with_mixed_groups() {
        let re = Regex::new(r"(?P<word>\w+)(?P<digit>\d)").unwrap();
        let text = "abc1";
        let caps = re.captures(text).unwrap();
        let mut result = String::new();
        caps.expand("${word} is number ${digit}", &mut result);
        assert_eq!(result, "abc is number 1");
    }
}

#[cfg(test)]
mod tests_llm_16_620 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_get_capture_group() {
        let re = Regex::new(r"(?P<digits>[0-9]+)(?P<letters>[a-zA-Z]+)").unwrap();
        let caps = re.captures("123abc").unwrap();
        
        let digits = caps.get(1).map(|m| m.as_str());
        let letters = caps.get(2).map(|m| m.as_str());

        assert_eq!(digits, Some("123"));
        assert_eq!(letters, Some("abc"));
    }

    #[test]
    fn test_get_non_existent_capture_group() {
        let re = Regex::new(r"(?P<digits>[0-9]+)(?P<letters>[a-zA-Z]+)").unwrap();
        let caps = re.captures("123abc").unwrap();
        
        let non_existent = caps.get(3); // Group 3 does not exist

        assert_eq!(non_existent, None);
    }

    #[test]
    fn test_get_non_matching_capture_group() {
        let re = Regex::new(r"(?P<letters>[a-zA-Z]+)(?P<digits>[0-9]+)").unwrap();
        let caps = re.captures("abc").unwrap(); // Only one group can be matched
        
        let digits = caps.get(2); // Group 2 did not match

        assert_eq!(digits, None);
    }
}

#[cfg(test)]
mod tests_llm_16_621 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_iter_captures() {
        let re = Regex::new(r"(?P<first>[a-z]+)(?P<second>[0-9]+)").unwrap();
        let text = "abc123";
        let caps = re.captures(text).unwrap();
        
        let mut iter = caps.iter();
        
        assert_eq!(iter.next(), Some(Some(Match::new(text, 0, 6)))); // whole match
        assert_eq!(iter.next(), Some(Some(Match::new(text, 0, 3)))); // first
        assert_eq!(iter.next(), Some(Some(Match::new(text, 3, 6)))); // second
        assert_eq!(iter.next(), None); // no more captures
    }

    #[test]
    fn test_iter_captures_with_none() {
        let re = Regex::new(r"(?P<first>[a-z]+)(?P<second>[0-9]+)").unwrap();
        let text = "abc";
        let caps = re.captures(text).unwrap();
        
        let mut iter = caps.iter();
        
        assert_eq!(iter.next(), Some(Some(Match::new(text, 0, 3)))); // whole match
        assert_eq!(iter.next(), Some(Some(Match::new(text, 0, 3)))); // first
        assert_eq!(iter.next(), Some(None)); // second does not match
        assert_eq!(iter.next(), None); // no more captures
    }

    #[test]
    fn test_iter_empty_captures() {
        let re = Regex::new(r"(?P<first>[a-z]+)(?P<second>[0-9]+)").unwrap();
        let text = "123";
        let caps = re.captures(text).unwrap();
        
        let mut iter = caps.iter();
        
        assert_eq!(iter.next(), Some(Some(Match::new(text, 0, 3)))); // whole match
        assert_eq!(iter.next(), Some(None)); // first does not match
        assert_eq!(iter.next(), Some(None)); // second does not match
        assert_eq!(iter.next(), None); // no more captures
    }
}

#[cfg(test)]
mod tests_llm_16_623 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_name_existing_capture_group() {
        let re = Regex::new(r"(?P<digits>\d+)").unwrap();
        let caps = re.captures("123").unwrap();
        let matched = caps.name("digits").unwrap();
        assert_eq!(matched.as_str(), "123");
    }

    #[test]
    fn test_name_non_existing_capture_group() {
        let re = Regex::new(r"(?P<digits>\d+)").unwrap();
        let caps = re.captures("123").unwrap();
        let matched = caps.name("letters");
        assert!(matched.is_none());
    }

    #[test]
    fn test_name_empty_capture_group() {
        let re = Regex::new(r"(?P<group>(?P<inner>\d+)?)").unwrap();
        let caps = re.captures("abc").unwrap();
        let matched = caps.name("inner");
        assert!(matched.is_none());
    }

    #[test]
    fn test_name_case_sensitive() {
        let re = Regex::new(r"(?P<CaseSensitive>\w+)").unwrap();
        let caps = re.captures("Hello").unwrap();
        let matched = caps.name("casesensitive");
        assert!(matched.is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_624 {
    use super::*;

use crate::*;
    use std::ops::Range;

    #[test]
    fn test_as_str() {
        let haystack = "hello world";
        let m = Match::new(haystack, 0, 5);
        assert_eq!(m.as_str(), "hello");

        let m = Match::new(haystack, 6, 11);
        assert_eq!(m.as_str(), "world");

        let m = Match::new(haystack, 0, 0);
        assert_eq!(m.as_str(), "");

        let m = Match::new(haystack, 5, 5);
        assert_eq!(m.as_str(), "");
    }
}

#[cfg(test)]
mod tests_llm_16_625 {
    use super::*;

use crate::*;
    use crate::re_unicode::Match;

    #[test]
    fn test_match_end() {
        let haystack = "hello world";
        let m = Match::new(haystack, 6, 11);
        assert_eq!(m.end(), 11);
        
        let m_empty = Match::new(haystack, 0, 0);
        assert_eq!(m_empty.end(), 0);
        
        let m_single_char = Match::new(haystack, 0, 1);
        assert_eq!(m_single_char.end(), 1);
    }
}

#[cfg(test)]
mod tests_llm_16_626 {
    use super::*;

use crate::*;
    use crate::re_unicode::Match;

    #[test]
    fn test_is_empty() {
        let haystack = "hello";
        
        let match_empty = Match::new(haystack, 5, 5);
        let match_non_empty = Match::new(haystack, 1, 3);

        assert!(match_empty.is_empty());
        assert!(!match_non_empty.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_627 {
    use super::*;

use crate::*;
    use re_unicode::Match;

    #[test]
    fn test_len() {
        let haystack = "Hello, world!";
        let match_instance = Match::new(haystack, 7, 12); // "world"
        assert_eq!(match_instance.len(), 5);
    }

    #[test]
    fn test_len_zero_length() {
        let haystack = "Hello, world!";
        let match_instance = Match::new(haystack, 5, 5); // empty match
        assert_eq!(match_instance.len(), 0);
    }

    #[test]
    fn test_len_full_length() {
        let haystack = "Hello, world!";
        let match_instance = Match::new(haystack, 0, haystack.len()); // full match
        assert_eq!(match_instance.len(), haystack.len());
    }
}

#[cfg(test)]
mod tests_llm_16_628 {
    use super::*;

use crate::*;
    use std::ops::Range;

    #[test]
    fn test_new() {
        let haystack = "hello, world";
        let start = 7;
        let end = 12;

        let m = Match::new(haystack, start, end);
        
        assert_eq!(m.start(), start);
        assert_eq!(m.end(), end);
        assert_eq!(m.as_str(), "world");
        assert_eq!(m.len(), 5);
        assert!(!m.is_empty());
        assert_eq!(m.range(), start..end);
    }

    #[test]
    fn test_new_empty_match() {
        let haystack = "hello, world";
        let start = 5;
        let end = 5;

        let m = Match::new(haystack, start, end);
        
        assert_eq!(m.start(), start);
        assert_eq!(m.end(), end);
        assert_eq!(m.as_str(), "");
        assert_eq!(m.len(), 0);
        assert!(m.is_empty());
        assert_eq!(m.range(), start..end);
    }
}

#[cfg(test)]
mod tests_llm_16_629 {
    use super::*;

use crate::*;
    use std::ops::Range;

    #[test]
    fn test_range() {
        let text = "hello world";
        let m = Match::new(text, 0, 5); // Match "hello"

        let r: Range<usize> = m.range();
        assert_eq!(r, 0..5);
    }

    #[test]
    fn test_range_empty() {
        let text = "hello world";
        let m = Match::new(text, 5, 5); // Match ""

        let r: Range<usize> = m.range();
        assert_eq!(r, 5..5);
    }

    #[test]
    fn test_range_full() {
        let text = "hello world";
        let m = Match::new(text, 0, text.len()); // Match "hello world"

        let r: Range<usize> = m.range();
        assert_eq!(r, 0..11);
    }
}

#[cfg(test)]
mod tests_llm_16_630 {
    use crate::re_unicode::Match;

    #[test]
    fn test_match_start() {
        let haystack = "hello world";
        let m = Match::new(haystack, 6, 11);
        assert_eq!(m.start(), 6);
    }

    #[test]
    fn test_match_start_empty() {
        let haystack = "hello";
        let m = Match::new(haystack, 5, 5); // empty match
        assert_eq!(m.start(), 5);
    }

    #[test]
    fn test_match_start_full_string() {
        let haystack = "full match";
        let m = Match::new(haystack, 0, haystack.len());
        assert_eq!(m.start(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_631 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_as_str() {
        let regex = Regex::new(r"\d+").unwrap();
        assert_eq!(regex.as_str(), r"\d+");
    }

    #[test]
    fn test_as_str_with_complex_regex() {
        let regex = Regex::new(r"[a-zA-Z]+").unwrap();
        assert_eq!(regex.as_str(), r"[a-zA-Z]+");
    }
    
    #[test]
    fn test_as_str_empty_regex() {
        let regex = Regex::new("").unwrap();
        assert_eq!(regex.as_str(), "");
    }
}

#[cfg(test)]
mod tests_llm_16_633 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_capture_names() {
        // Test case with named captures
        let regex = Regex::new(r"(?P<name>\w+)").unwrap();
        let names: Vec<Option<&str>> = regex.capture_names().collect();
        assert_eq!(names.len(), 2); // 1 named capture + 1 unnamed (whole match)
        assert_eq!(names[1], Some("name"));

        // Test case with no named captures
        let regex = Regex::new(r"(\w+)").unwrap();
        let names: Vec<Option<&str>> = regex.capture_names().collect();
        assert_eq!(names.len(), 2); // 1 unnamed capture (whole match) + 1 unnamed
        assert_eq!(names[1], None); // Second capture is unnamed

        // Test case with no captures
        let regex = Regex::new(r"\d+").unwrap();
        let names: Vec<Option<&str>> = regex.capture_names().collect();
        assert_eq!(names.len(), 1); // Only has the unnamed capture for the whole match
        assert_eq!(names[0], None); // The whole match is unnamed
    }
}

#[cfg(test)]
mod tests_llm_16_634 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_captures_success() {
        let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
        let text = "Not my favorite movie: 'Citizen Kane' (1941).";
        let caps = re.captures(text).unwrap();
        
        assert_eq!(caps.get(1).unwrap().as_str(), "Citizen Kane");
        assert_eq!(caps.get(2).unwrap().as_str(), "1941");
        assert_eq!(caps.get(0).unwrap().as_str(), "'Citizen Kane' (1941)");
    }

    #[test]
    fn test_captures_named_groups() {
        let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
        let text = "Not my favorite movie: 'Citizen Kane' (1941).";
        let caps = re.captures(text).unwrap();
        
        assert_eq!(caps.name("title").unwrap().as_str(), "Citizen Kane");
        assert_eq!(caps.name("year").unwrap().as_str(), "1941");
        assert_eq!(caps.get(0).unwrap().as_str(), "'Citizen Kane' (1941)");
    }

    #[test]
    fn test_captures_no_match() {
        let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
        let text = "No movie data available.";
        let caps = re.captures(text);
        
        assert!(caps.is_none());
    }

    #[test]
    fn test_captures_with_empty_input() {
        let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
        let text = "";
        let caps = re.captures(text);
        
        assert!(caps.is_none());
    }

    #[test]
    fn test_captures_with_multiple_matches() {
        let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
        let text = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939).";
        
        let caps1 = re.captures(text).unwrap();
        assert_eq!(caps1.get(1).unwrap().as_str(), "Citizen Kane");
        
        let caps2 = re.captures(&text[26..]).unwrap();
        assert_eq!(caps2.get(1).unwrap().as_str(), "The Wizard of Oz");
    }
}

#[cfg(test)]
mod tests_llm_16_635 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_captures_at() {
        let re = Regex::new(r"(?P<name>\w+)").unwrap();
        let text = "Alice and Bob";
        
        let captures = re.captures_at(text, 0).unwrap();
        assert_eq!(captures.name("name").unwrap().as_str(), "Alice");
        
        let captures = re.captures_at(text, 7).unwrap();
        assert_eq!(captures.name("name").unwrap().as_str(), "Bob");
        
        let captures = re.captures_at(text, 10);
        assert!(captures.is_none());
        
        let captures = re.captures_at(text, text.len());
        assert!(captures.is_none());
    }

    #[test]
    fn test_captures_at_with_offset() {
        let re = Regex::new(r"(\d+)").unwrap();
        let text = "12 apples and 34 oranges";

        let captures = re.captures_at(text, 0).unwrap();
        assert_eq!(captures.get(0).unwrap().as_str(), "12");

        let captures = re.captures_at(text, 15).unwrap();
        assert_eq!(captures.get(0).unwrap().as_str(), "34");

        let captures = re.captures_at(text, 20);
        assert!(captures.is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_636 {
    use super::*;

use crate::*;
    use re_unicode::Regex;
    
    #[test]
    fn test_captures_iter() {
        let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
        let text = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";
        let results: Vec<_> = re.captures_iter(text).collect();
        
        assert_eq!(results.len(), 3);
        assert_eq!(&results[0]["title"], "Citizen Kane");
        assert_eq!(&results[0]["year"], "1941");
        assert_eq!(&results[1]["title"], "The Wizard of Oz");
        assert_eq!(&results[1]["year"], "1939");
        assert_eq!(&results[2]["title"], "M");
        assert_eq!(&results[2]["year"], "1931");
    }

    #[test]
    fn test_no_captures() {
        let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
        let text = "No movies here.";
        let results: Vec<_> = re.captures_iter(text).collect();
        
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_partial_match() {
        let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
        let text = "'Movie Title' (XXXX)";
        let results: Vec<_> = re.captures_iter(text).collect();
        
        assert_eq!(results.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_637 {
    use super::*; // Adjust the import as necessary to include relevant definitions

use crate::*;
    use crate::Regex;

    #[test]
    fn test_captures_len() {
        let re = Regex::new(r"(?P<first>\w+) (?P<second>\w+)").unwrap();
        assert_eq!(re.captures_len(), 3); // Two named captures plus the entire match

        let re2 = Regex::new(r"(\d+)").unwrap();
        assert_eq!(re2.captures_len(), 2); // One captured group plus the entire match

        let re3 = Regex::new(r"(\w+)").unwrap();
        assert_eq!(re3.captures_len(), 2); // One captured group plus the entire match

        let re4 = Regex::new(r"(\w+)|(\d+)").unwrap();
        assert_eq!(re4.captures_len(), 3); // Two captured groups plus the entire match
    }
}

#[cfg(test)]
mod tests_llm_16_638 {
    use super::*;

use crate::*;
    use crate::Regex;
    use crate::Captures;
    use crate::re_unicode::CaptureLocations;

    #[test]
    fn test_captures_read() {
        let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let text = "Bruce Springsteen";

        let m = re.captures_read(&mut locs, text).unwrap();
        assert_eq!(0..17, m.range());
        assert_eq!(Some((0, 17)), locs.get(0));
        assert_eq!(Some((0, 5)), locs.get(1));
        assert_eq!(Some((6, 17)), locs.get(2));
    }

    #[test]
    fn test_captures_read_no_match() {
        let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let text = "No Match";

        let result = re.captures_read(&mut locs, text);
        assert!(result.is_none());
    }

    #[test]
    fn test_invalid_capture_group() {
        let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let text = "Bruce Springsteen";

        let m = re.captures_read(&mut locs, text).unwrap();
        assert_eq!(locs.get(3), None);
    }
}

#[cfg(test)]
mod tests_llm_16_639 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_captures_read_at() {
        let re = Regex::new(r"(?P<first>\w+)\s+(?P<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        let text = "Bruce Springsteen";

        // Test case where search begins at the start of the text
        let m = re.captures_read_at(&mut locs, text, 0).unwrap();
        assert_eq!(0..17, m.range());
        assert_eq!(Some((0, 17)), locs.get(0)); // Full match
        assert_eq!(Some((0, 5)), locs.get(1));  // First name
        assert_eq!(Some((6, 17)), locs.get(2)); // Last name

        // Test case where search begins after the first match
        let m = re.captures_read_at(&mut locs, text, 6).unwrap();
        assert_eq!(6..17, m.range());
        assert_eq!(Some((6, 17)), locs.get(0)); // Full match
        assert_eq!(None, locs.get(1));          // First name not matched
        assert_eq!(Some((6, 17)), locs.get(2)); // Last name

        // Test case where starting point exceeds the length of the text
        assert!(re.captures_read_at(&mut locs, text, 100).is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_640 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_find_valid_match() {
        let text = "I categorically deny having triskaidekaphobia.";
        let regex = Regex::new(r"\b\w{13}\b").unwrap();
        let mat = regex.find(text).unwrap();
        assert_eq!(mat.start(), 2);
        assert_eq!(mat.end(), 15);
    }

    #[test]
    fn test_find_no_match() {
        let text = "No 13-letter words here.";
        let regex = Regex::new(r"\b\w{13}\b").unwrap();
        let mat = regex.find(text);
        assert!(mat.is_none());
    }

    #[test]
    fn test_find_multiple_matches() {
        let text = "This is a test string for the regex test.";
        let regex = Regex::new(r"\b\w{4}\b").unwrap();
        let mat = regex.find(text).unwrap();
        assert_eq!(mat.start(), 10);
        assert_eq!(mat.end(), 14);
    }

    #[test]
    fn test_find_empty_string() {
        let text = "";
        let regex = Regex::new(r"\b\w{13}\b").unwrap();
        let mat = regex.find(text);
        assert!(mat.is_none());
    }

    #[test]
    fn test_find_with_unicode() {
        let text = "naïve façade résumé";
        let regex = Regex::new(r"\b\w{6}\b").unwrap(); // looking for a 6-letter unicode word
        let mat = regex.find(text).unwrap();
        assert_eq!(mat.start(), 0);
        assert_eq!(mat.end(), 6);
    }
}

#[cfg(test)]
mod tests_llm_16_641 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_find_at_match() {
        let re = Regex::new(r"\d+").unwrap();
        let text = "Order 1234 and 5678";
        let result = re.find_at(text, 6).unwrap();
        assert_eq!(result.start(), 6);
        assert_eq!(result.end(), 10);
    }

    #[test]
    fn test_find_at_no_match() {
        let re = Regex::new(r"\d+").unwrap();
        let text = "Order none";
        let result = re.find_at(text, 6);
        assert!(result.is_none());
    }

    #[test]
    fn test_find_at_offset() {
        let re = Regex::new(r"\d+").unwrap();
        let text = "Orders 23456";
        let result = re.find_at(text, 0).unwrap();
        assert_eq!(result.start(), 7);
        assert_eq!(result.end(), 12);
    }

    #[test]
    fn test_find_at_offset_beyond_length() {
        let re = Regex::new(r"\d+").unwrap();
        let text = "There are no numbers";
        let result = re.find_at(text, 100);
        assert!(result.is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_642 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_find_iter_no_matches() {
        let regex = Regex::new(r"\y\w{13}\y").unwrap();
        let text = "Short words here.";
        let matches: Vec<(usize, usize)> = regex.find_iter(text).map(|m| (m.start(), m.end())).collect();
        assert!(matches.is_empty());
    }

    #[test]
    fn test_find_iter_some_matches() {
        let regex = Regex::new(r"\b\w{13}\b").unwrap();
        let text = "Retroactively relinquishing remunerations is reprehensible.";
        let matches: Vec<(usize, usize)> = regex.find_iter(text).map(|m| (m.start(), m.end())).collect();
        assert_eq!(matches.len(), 3); // Expecting three matches for the words of length 13
        assert_eq!(matches, vec![(0, 13), (15, 28), (30, 43)]);
    }

    #[test]
    fn test_find_iter_empty_string() {
        let regex = Regex::new(r"\b\w{4}\b").unwrap();
        let text = "";
        let matches: Vec<(usize, usize)> = regex.find_iter(text).map(|m| (m.start(), m.end())).collect();
        assert!(matches.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_643 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_is_match_basic() {
        let re = Regex::new(r"\b\w{13}\b").unwrap();
        let text = "I categorically deny having triskaidekaphobia.";
        assert!(re.is_match(text));
    }

    #[test]
    fn test_is_match_no_match() {
        let re = Regex::new(r"\b\w{15}\b").unwrap();
        let text = "Short text.";
        assert!(!re.is_match(text));
    }

    #[test]
    fn test_is_match_empty_string() {
        let re = Regex::new(r"\S").unwrap(); // match any non-whitespace
        let text = "";
        assert!(!re.is_match(text));
    }

    #[test]
    fn test_is_match_with_special_characters() {
        let re = Regex::new(r"\w+").unwrap(); // match any word
        let text = "This is a test!";
        assert!(re.is_match(text));
    }

    #[test]
    fn test_is_match_unicode() {
        let re = Regex::new(r"\b\w{2,}\b").unwrap(); // at least 2 unicode word characters
        let text = "こんにちは、元気ですか？"; // "Hello, how are you?"
        assert!(re.is_match(text));
    }

    #[test]
    fn test_is_match_with_no_unicode_match() {
        let re = Regex::new(r"\b\w{5,}\b").unwrap(); // match words with at least 5 characters
        let text = "あ"; // a single unicode character
        assert!(!re.is_match(text));
    }
}

#[cfg(test)]
mod tests_llm_16_644 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_is_match_at() {
        let re = Regex::new(r"\d+").unwrap();

        assert!(re.is_match_at("123abc", 0));  // Match starts at 0
        assert!(re.is_match_at("abc123", 3));  // Match starts at 3
        assert!(!re.is_match_at("abc", 0));     // No match
        assert!(!re.is_match_at("123abc", 4));  // No match at start 4
        assert!(!re.is_match_at("", 0));         // No match in empty string
        assert!(!re.is_match_at("abc123", 0));  // No match at start 0
        assert!(re.is_match_at("xyz 456", 4));  // Match at start 4
        assert!(!re.is_match_at("12", 3));       // Out of bounds start
        assert!(re.is_match_at("00test", 0));    // Match at start 0
        assert!(!re.is_match_at("numm3rs", 0));  // No match at start 0
    }
}

#[cfg(test)]
mod tests_llm_16_645 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_locations() {
        let re = Regex::new(r"(\w+) (\w+)").unwrap();
        let locs = re.locations();
        
        assert_eq!(locs.len(), 3); // 0: full match, 1: first word, 2: second word
        assert_eq!(locs.get(0), Some((0, 13))); // match for full pattern
        assert_eq!(locs.get(1), Some((0, 5))); // match for first word
        assert_eq!(locs.get(2), Some((6, 11))); // match for second word

        // Testing invalid capture group
        assert_eq!(locs.get(3), None);
    }
}

#[cfg(test)]
mod tests_llm_16_646 {
    use super::*;

use crate::*;
    use crate::Regex; // Make sure to import the Regex struct
    use crate::Error; // Import the Error type for testing

    #[test]
    fn test_new_valid_regex() {
        let result = re_unicode::Regex::new(r"^a.*b$");
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_invalid_regex() {
        let result = re_unicode::Regex::new(r"(");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_new_empty_regex() {
        let result = re_unicode::Regex::new("");
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod tests_llm_16_647 {
    use super::*;

use crate::*;
    use crate::Regex;
    use re_unicode::CaptureLocations;

    #[test]
    fn test_read_captures_at() {
        let re = Regex::new(r"(?P<first>\w+)\s+(?P<last>\w+)").unwrap();
        let mut locs = re.capture_locations();
        
        let text = "Bruce Springsteen";
        let match_result = re.read_captures_at(&mut locs, text, 0);
        
        assert!(match_result.is_some());
        let m = match_result.unwrap();
        assert_eq!(m.start(), 0);
        assert_eq!(m.end(), 17);
        
        assert_eq!(locs.get(0), Some((0, 17))); // total match
        assert_eq!(locs.get(1), Some((0, 5)));  // first name
        assert_eq!(locs.get(2), Some((6, 17))); // last name
    }
}

#[cfg(test)]
mod tests_llm_16_648 {
    use super::*;

use crate::*;
    use crate::Regex;
    use crate::Captures;
    use std::borrow::Cow;

    #[test]
    fn test_replace_with_string() {
        let re = Regex::new(r"(\w+)\s+(\w+)").unwrap();
        let result = re.replace("Hello World", "$2 $1");
        assert_eq!(result, "World Hello");
    }

    #[test]
    fn test_replace_with_function() {
        let re = Regex::new(r"(\w+)\s+(\w+)").unwrap();
        let result = re.replace("Hello World", |caps: &Captures| {
            format!("{} {}", &caps[2], &caps[1])
        });
        assert_eq!(result, "World Hello");
    }

    #[test]
    fn test_replace_no_match() {
        let re = Regex::new(r"(\d+)").unwrap();
        let result = re.replace("Hello World", "Match");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_replace_with_non_existent_capture() {
        let re = Regex::new(r"(\w+)").unwrap();
        let result = re.replace("Hello", "$2");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_replace_with_empty_string() {
        let re = Regex::new(r"(\w+)").unwrap();
        let result = re.replace("Hello", "");
        assert_eq!(result, "");
    }

    #[test]
    fn test_replace_with_no_expand() {
        let re = Regex::new(r"(\w+)").unwrap();
        use crate::NoExpand;
        let result = re.replace("Hello", NoExpand("$1"));
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_replace_with_braces() {
        let re = Regex::new(r"(?P<first>\w+)\s+(?P<second>\w+)").unwrap();
        let result = re.replace("Hello World", "${second}_${first}");
        assert_eq!(result, "World_Hello");
    }
}

#[cfg(test)]
mod tests_llm_16_649 {
    use super::*;

use crate::*;
    use crate::Regex;
    use crate::Captures;
    use std::borrow::Cow;

    #[test]
    fn test_replace_all_with_string() {
        let re = Regex::new(r"\d+").unwrap();
        let text = "There are 12 apples and 24 oranges.";
        let result = re.replace_all(text, "fruit");
        assert_eq!(result, "There are fruit apples and fruit oranges.");
    }

    #[test]
    fn test_replace_all_with_closure() {
        let re = Regex::new(r"(\d+)").unwrap();
        let text = "There are 12 apples and 24 oranges.";
        let result = re.replace_all(text, |caps: &Captures| {
            let num: u32 = caps[1].parse().unwrap();
            (num * 2).to_string()
        });
        assert_eq!(result, "There are 24 apples and 48 oranges.");
    }

    #[test]
    fn test_replace_all_with_no_matches() {
        let re = Regex::new(r"xyz").unwrap();
        let text = "There are 12 apples.";
        let result = re.replace_all(text, "fruit");
        assert_eq!(result, "There are 12 apples.");
    }

    #[test]
    fn test_replace_all_with_empty_string() {
        let re = Regex::new(r"\s+").unwrap();
        let text = "This  is   a test.";
        let result = re.replace_all(text, " ");
        assert_eq!(result, "This is a test.");
    }

    #[test]
    fn test_replace_all_with_captures() {
        let re = Regex::new(r"(?P<fruit>\w+)").unwrap();
        let text = "I have apples and oranges.";
        let result = re.replace_all(text, |caps: &Captures| {
            format!("I have a {}", &caps["fruit"])
        });
        assert_eq!(result, "I have a apples and I have a oranges.");
    }
}

#[cfg(test)]
mod tests_llm_16_651 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_shortest_match_basic() {
        let re = Regex::new(r"a+").unwrap();
        let pos = re.shortest_match("aaaaa");
        assert_eq!(pos, Some(1));
    }

    #[test]
    fn test_shortest_match_no_match() {
        let re = Regex::new(r"b+").unwrap();
        let pos = re.shortest_match("aaaaa");
        assert_eq!(pos, None);
    }

    #[test]
    fn test_shortest_match_with_different_inputs() {
        let re = Regex::new(r"[aeiou]+").unwrap();
        assert_eq!(re.shortest_match("hello"), Some(1));
        assert_eq!(re.shortest_match("rhythm"), None);
        assert_eq!(re.shortest_match("a quick fox"), Some(0));
    }

    #[test]
    fn test_shortest_match_with_empty_string() {
        let re = Regex::new(r"a+").unwrap();
        let pos = re.shortest_match("");
        assert_eq!(pos, None);
    }

    #[test]
    fn test_shortest_match_at_different_positions() {
        let re = Regex::new(r"a+").unwrap();
        assert_eq!(re.shortest_match_at("aaaaa", 0), Some(1));
        assert_eq!(re.shortest_match_at("aaaaa", 1), Some(2));
        assert_eq!(re.shortest_match_at("aaaaa", 2), Some(3));
        assert_eq!(re.shortest_match_at("aaaaa", 3), Some(4));
        assert_eq!(re.shortest_match_at("aaaaa", 4), None);
    }
}

#[cfg(test)]
mod tests_llm_16_652 {
    use super::*;

use crate::*;
    
    #[test]
    fn test_shortest_match_at() {
        let re = Regex::new(r"a+").unwrap();
        assert_eq!(re.shortest_match_at("aaaaa", 0), Some(1));
        assert_eq!(re.shortest_match_at("aaaaa", 1), Some(1));
        assert_eq!(re.shortest_match_at("bbbaaa", 0), Some(4));
        assert_eq!(re.shortest_match_at("bbbaaa", 1), Some(4));
        assert_eq!(re.shortest_match_at("bbbaaa", 4), Some(4));
        assert_eq!(re.shortest_match_at("bbbb", 0), None);
        assert_eq!(re.shortest_match_at("", 0), None);
    }
}

#[cfg(test)]
mod tests_llm_16_653 {
    use super::*;

use crate::*;
    use re_unicode::Regex;

    #[test]
    fn test_split_basic() {
        let re = Regex::new(r"[ \t]+").unwrap();
        let fields: Vec<&str> = re.split("a b \t  c\td    e").collect();
        assert_eq!(fields, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn test_split_multiple_spaces() {
        let re = Regex::new(r"[ \t]+").unwrap();
        let fields: Vec<&str> = re.split("hello    world").collect();
        assert_eq!(fields, vec!["hello", "world"]);
    }

    #[test]
    fn test_split_leading_trailing_spaces() {
        let re = Regex::new(r"[ \t]+").unwrap();
        let fields: Vec<&str> = re.split("   leading and trailing    ").collect();
        assert_eq!(fields, vec!["leading", "and", "trailing"]);
    }

    #[test]
    fn test_split_empty_string() {
        let re = Regex::new(r"[ \t]+").unwrap();
        let fields: Vec<&str> = re.split("").collect();
        assert_eq!(fields, vec![""]);
    }

    #[test]
    fn test_split_single_delimiter() {
        let re = Regex::new(r"[ \t]+").unwrap();
        let fields: Vec<&str> = re.split("single").collect();
        assert_eq!(fields, vec!["single"]);
    }

    #[test]
    fn test_split_multiple_delimiters() {
        let re = Regex::new(r"[ \t]+").unwrap();
        let fields: Vec<&str> = re.split("a \t b \t c \t d").collect();
        assert_eq!(fields, vec!["a", "b", "c", "d"]);
    }
}

#[cfg(test)]
mod tests_llm_16_654 {
    use super::*;

use crate::*;
    use crate::Regex;

    #[test]
    fn test_splitn_basic() {
        let re = Regex::new(r"\W+").unwrap();
        let result: Vec<&str> = re.splitn("Hey! How are you?", 3).collect();
        assert_eq!(result, vec!["Hey", "How", "are you?"]);
    }

    #[test]
    fn test_splitn_with_zero_limit() {
        let re = Regex::new(r"\W+").unwrap();
        let result: Vec<&str> = re.splitn("Hey! How are you?", 0).collect();
        assert_eq!(result, Vec::<&str>::new());
    }

    #[test]
    fn test_splitn_with_large_limit() {
        let re = Regex::new(r"\W+").unwrap();
        let result: Vec<&str> = re.splitn("Hey! How are you?", 10).collect();
        assert_eq!(result, vec!["Hey", "How", "are", "you?"]);
    }

    #[test]
    fn test_splitn_no_matches() {
        let re = Regex::new(r"\W+").unwrap();
        let result: Vec<&str> = re.splitn("HeyHeyHey", 2).collect();
        assert_eq!(result, vec!["HeyHeyHey"]);
    }

    #[test]
    fn test_splitn_with_empty_string() {
        let re = Regex::new(r"\W+").unwrap();
        let result: Vec<&str> = re.splitn("", 3).collect();
        assert_eq!(result, vec![""]);
    }

    #[test]
    fn test_splitn_single_word() {
        let re = Regex::new(r"\W+").unwrap();
        let result: Vec<&str> = re.splitn("Hello", 3).collect();
        assert_eq!(result, vec!["Hello"]);
    }
}

#[cfg(test)]
mod tests_llm_16_655 {
    use super::*;

use crate::*;
    use crate::Regex;

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

#[cfg(test)]
mod tests_llm_16_656 {
    use super::*;

use crate::*;
    use crate::{Regex, Replacer, Captures};

    struct TestReplacer {
        count: usize,
    }

    impl Replacer for TestReplacer {
        fn replace_append(&mut self, _: &Captures<'_>, dst: &mut String) {
            self.count += 1;
            dst.push_str("replacement");
        }

        fn no_expansion(&mut self) -> Option<Cow<'_, str>> {
            None
        }
    }

    #[test]
    fn test_by_ref() {
        let re = Regex::new(r"\d+").unwrap();
        let mut replacer = TestReplacer { count: 0 };
        
        let result = re.replace_all("1 2 3", replacer.by_ref());
        let result = re.replace_all(&result, replacer.by_ref());

        assert_eq!(result, "replacement replacement replacement");
        assert_eq!(replacer.count, 3);
    }
}

#[cfg(test)]
mod tests_llm_16_658 {
    use crate::re_unicode::escape;

    #[test]
    fn test_escape() {
        assert_eq!(escape("."), "\\.");
        assert_eq!(escape("*"), "\\*");
        assert_eq!(escape("+"), "\\+");
        assert_eq!(escape("?"), "\\?");
        assert_eq!(escape("^"), "\\^");
        assert_eq!(escape("$"), "\\$");
        assert_eq!(escape("|"), "\\|");
        assert_eq!(escape("()[]{}"), "\\(\\)\\[\\]\\{\\}");
        assert_eq!(escape("a.b*c+d?e^f$g|h"), "a\\.b\\*c\\+d\\?e\\^f\\$g\\|h");
    }
}

#[cfg(test)]
mod tests_llm_16_659 {
    use super::*;

use crate::*;
    use std::borrow::Cow;

    #[test]
    fn test_no_expansion_with_dollar_sign() {
        let input = "Hello $World";
        let result = no_expansion(&input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_no_expansion_without_dollar_sign() {
        let input = "Hello World";
        let result = no_expansion(&input);
        assert_eq!(result, Some(Cow::Borrowed("Hello World")));
    }

    #[test]
    fn test_no_expansion_empty_string() {
        let input = "";
        let result = no_expansion(&input);
        assert_eq!(result, Some(Cow::Borrowed("")));
    }

    #[test]
    fn test_no_expansion_only_dollar_sign() {
        let input = "$";
        let result = no_expansion(&input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_no_expansion_multiple_dollar_signs() {
        let input = "Hello $World $!";
        let result = no_expansion(&input);
        assert_eq!(result, None);
    }
}
