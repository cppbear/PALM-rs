use std::fmt;
use std::iter::FusedIterator;

/// Slot is a single saved capture location. Note that there are two slots for
/// every capture in a regular expression (one slot each for the start and end
/// of the capture).
pub type Slot = Option<usize>;

/// Locations represents the offsets of each capturing group in a regex for
/// a single match.
///
/// Unlike `Captures`, a `Locations` value only stores offsets.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Locations(Vec<Slot>);

impl Locations {
    /// Returns the start and end positions of the Nth capture group. Returns
    /// `None` if `i` is not a valid capture group or if the capture group did
    /// not match anything. The positions returned are *always* byte indices
    /// with respect to the original string matched.
    pub fn pos(&self, i: usize) -> Option<(usize, usize)> {
        let (s, e) = (i.checked_mul(2)?, i.checked_mul(2)?.checked_add(1)?);
        match (self.0.get(s), self.0.get(e)) {
            (Some(&Some(s)), Some(&Some(e))) => Some((s, e)),
            _ => None,
        }
    }

    /// Creates an iterator of all the capture group positions in order of
    /// appearance in the regular expression. Positions are byte indices
    /// in terms of the original string matched.
    pub fn iter(&self) -> SubCapturesPosIter<'_> {
        SubCapturesPosIter { idx: 0, locs: self }
    }

    /// Returns the total number of capturing groups.
    ///
    /// This is always at least `1` since every regex has at least `1`
    /// capturing group that corresponds to the entire match.
    pub fn len(&self) -> usize {
        self.0.len() / 2
    }

    /// Return the individual slots as a slice.
    pub(crate) fn as_slots(&mut self) -> &mut [Slot] {
        &mut self.0
    }
}

/// An iterator over capture group positions for a particular match of a
/// regular expression.
///
/// Positions are byte indices in terms of the original string matched.
///
/// `'c` is the lifetime of the captures.
#[derive(Clone, Debug)]
pub struct SubCapturesPosIter<'c> {
    idx: usize,
    locs: &'c Locations,
}

impl<'c> Iterator for SubCapturesPosIter<'c> {
    type Item = Option<(usize, usize)>;

    fn next(&mut self) -> Option<Option<(usize, usize)>> {
        if self.idx >= self.locs.len() {
            return None;
        }
        let x = match self.locs.pos(self.idx) {
            None => Some(None),
            Some((s, e)) => Some(Some((s, e))),
        };
        self.idx += 1;
        x
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.locs.len() - self.idx;
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.len()
    }
}

impl<'c> ExactSizeIterator for SubCapturesPosIter<'c> {}

impl<'c> FusedIterator for SubCapturesPosIter<'c> {}

/// `RegularExpression` describes types that can implement regex searching.
///
/// This trait is my attempt at reducing code duplication and to standardize
/// the internal API. Specific duplication that is avoided are the `find`
/// and `capture` iterators, which are slightly tricky.
///
/// It's not clear whether this trait is worth it, and it also isn't
/// clear whether it's useful as a public trait or not. Methods like
/// `next_after_empty` reak of bad design, but the rest of the methods seem
/// somewhat reasonable. One particular thing this trait would expose would be
/// the ability to start the search of a regex anywhere in a haystack, which
/// isn't possible in the current public API.
pub trait RegularExpression: Sized + fmt::Debug {
    /// The type of the haystack.
    type Text: ?Sized + fmt::Debug;

    /// The number of capture slots in the compiled regular expression. This is
    /// always two times the number of capture groups (two slots per group).
    fn slots_len(&self) -> usize;

    /// Allocates fresh space for all capturing groups in this regex.
    fn locations(&self) -> Locations {
        Locations(vec![None; self.slots_len()])
    }

    /// Returns the position of the next character after `i`.
    ///
    /// For example, a haystack with type `&[u8]` probably returns `i+1`,
    /// whereas a haystack with type `&str` probably returns `i` plus the
    /// length of the next UTF-8 sequence.
    fn next_after_empty(&self, text: &Self::Text, i: usize) -> usize;

    /// Returns the location of the shortest match.
    fn shortest_match_at(
        &self,
        text: &Self::Text,
        start: usize,
    ) -> Option<usize>;

    /// Returns whether the regex matches the text given.
    fn is_match_at(&self, text: &Self::Text, start: usize) -> bool;

    /// Returns the leftmost-first match location if one exists.
    fn find_at(
        &self,
        text: &Self::Text,
        start: usize,
    ) -> Option<(usize, usize)>;

    /// Returns the leftmost-first match location if one exists, and also
    /// fills in any matching capture slot locations.
    fn captures_read_at(
        &self,
        locs: &mut Locations,
        text: &Self::Text,
        start: usize,
    ) -> Option<(usize, usize)>;

    /// Returns an iterator over all non-overlapping successive leftmost-first
    /// matches.
    fn find_iter(self, text: &Self::Text) -> Matches<'_, Self> {
        Matches { re: self, text, last_end: 0, last_match: None }
    }

    /// Returns an iterator over all non-overlapping successive leftmost-first
    /// matches with captures.
    fn captures_iter(self, text: &Self::Text) -> CaptureMatches<'_, Self> {
        CaptureMatches(self.find_iter(text))
    }
}

/// An iterator over all non-overlapping successive leftmost-first matches.
#[derive(Debug)]
pub struct Matches<'t, R>
where
    R: RegularExpression,
    R::Text: 't,
{
    re: R,
    text: &'t R::Text,
    last_end: usize,
    last_match: Option<usize>,
}

impl<'t, R> Matches<'t, R>
where
    R: RegularExpression,
    R::Text: 't,
{
    /// Return the text being searched.
    pub fn text(&self) -> &'t R::Text {
        self.text
    }

    /// Return the underlying regex.
    pub fn regex(&self) -> &R {
        &self.re
    }
}

impl<'t, R> Iterator for Matches<'t, R>
where
    R: RegularExpression,
    R::Text: 't + AsRef<[u8]>,
{
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if self.last_end > self.text.as_ref().len() {
            return None;
        }
        let (s, e) = match self.re.find_at(self.text, self.last_end) {
            None => return None,
            Some((s, e)) => (s, e),
        };
        if s == e {
            // This is an empty match. To ensure we make progress, start
            // the next search at the smallest possible starting position
            // of the next match following this one.
            self.last_end = self.re.next_after_empty(self.text, e);
            // Don't accept empty matches immediately following a match.
            // Just move on to the next match.
            if Some(e) == self.last_match {
                return self.next();
            }
        } else {
            self.last_end = e;
        }
        self.last_match = Some(e);
        Some((s, e))
    }
}

impl<'t, R> FusedIterator for Matches<'t, R>
where
    R: RegularExpression,
    R::Text: 't + AsRef<[u8]>,
{
}

/// An iterator over all non-overlapping successive leftmost-first matches with
/// captures.
#[derive(Debug)]
pub struct CaptureMatches<'t, R>(Matches<'t, R>)
where
    R: RegularExpression,
    R::Text: 't;

impl<'t, R> CaptureMatches<'t, R>
where
    R: RegularExpression,
    R::Text: 't,
{
    /// Return the text being searched.
    pub fn text(&self) -> &'t R::Text {
        self.0.text()
    }

    /// Return the underlying regex.
    pub fn regex(&self) -> &R {
        self.0.regex()
    }
}

impl<'t, R> Iterator for CaptureMatches<'t, R>
where
    R: RegularExpression,
    R::Text: 't + AsRef<[u8]>,
{
    type Item = Locations;

    fn next(&mut self) -> Option<Locations> {
        if self.0.last_end > self.0.text.as_ref().len() {
            return None;
        }
        let mut locs = self.0.re.locations();
        let (s, e) = match self.0.re.captures_read_at(
            &mut locs,
            self.0.text,
            self.0.last_end,
        ) {
            None => return None,
            Some((s, e)) => (s, e),
        };
        if s == e {
            self.0.last_end = self.0.re.next_after_empty(self.0.text, e);
            if Some(e) == self.0.last_match {
                return self.next();
            }
        } else {
            self.0.last_end = e;
        }
        self.0.last_match = Some(e);
        Some(locs)
    }
}

impl<'t, R> FusedIterator for CaptureMatches<'t, R>
where
    R: RegularExpression,
    R::Text: 't + AsRef<[u8]>,
{
}

#[cfg(test)]
mod tests_llm_16_174 {
    use super::*;

use crate::*;
    use crate::re_trait::{Locations, SubCapturesPosIter};

    #[test]
    fn test_count() {
        let locs = Locations(vec![Some(0), Some(5), Some(10), Some(15)]);
        let iter: SubCapturesPosIter = locs.iter();
        assert_eq!(iter.count(), 2); // There are 2 capturing groups based on the vec size
    }

    #[test]
    fn test_count_with_empty_locations() {
        let locs = Locations(vec![]);
        let iter: SubCapturesPosIter = locs.iter();
        assert_eq!(iter.count(), 0); // No capturing groups
    }

    #[test]
    fn test_count_with_one_capture() {
        let locs = Locations(vec![Some(0), Some(5)]);
        let iter: SubCapturesPosIter = locs.iter();
        assert_eq!(iter.count(), 1); // One capturing group
    }

    #[test]
    fn test_count_with_none() {
        let locs = Locations(vec![None, None, Some(10), Some(15)]);
        let iter: SubCapturesPosIter = locs.iter();
        assert_eq!(iter.count(), 2); // There are 2 capturing groups (10, 15)
    }
}

#[cfg(test)]
mod tests_llm_16_175 {
    use super::*; // import everything from the parent module

use crate::*;
    use crate::re_trait::{Locations, SubCapturesPosIter};

    #[test]
    fn test_next() {
        let slots = vec![Some(0), Some(5), Some(6), Some(12), Some(13), Some(20)];
        let locations = Locations(slots);
        let mut iter = locations.iter();

        // First capture group
        assert_eq!(iter.next(), Some(Some((0, 5))));
        // Second capture group
        assert_eq!(iter.next(), Some(Some((6, 12))));
        // Third capture group
        assert_eq!(iter.next(), Some(Some((13, 20))));
        // No more capture groups
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_next_with_no_match() {
        let slots = vec![Some(0), None, None, None];
        let locations = Locations(slots);
        let mut iter = locations.iter();

        // First capture group
        assert_eq!(iter.next(), Some(Some((0, 0))));
        // No match for second capture group
        assert_eq!(iter.next(), Some(None));
        // No match for third capture group
        assert_eq!(iter.next(), Some(None));
        // No more capture groups
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_next_boundary() {
        let slots = vec![Some(0), Some(3)];
        let locations = Locations(slots);
        let mut iter = locations.iter();

        // First capture group
        assert_eq!(iter.next(), Some(Some((0, 3))));
        // No more capture groups
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_next_empty() {
        let slots: Vec<Option<usize>> = vec![];
        let locations = Locations(slots);
        let mut iter = locations.iter();

        // No capture groups
        assert_eq!(iter.next(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_176 {
    use super::*;

use crate::*;
    use crate::re_trait::{Locations, SubCapturesPosIter};

    #[test]
    fn test_size_hint() {
        let locations = Locations(vec![Some(0), Some(1), Some(2), Some(3), None, None]);
        let mut iter = locations.iter();

        // Test size_hint when there are some locations left
        let (len, upper) = iter.size_hint();
        assert_eq!(len, 3); // 3 valid capturing groups left
        assert_eq!(upper, Some(3));

        // Consume one item
        iter.next();
        
        // Test size_hint after consuming one item
        let (len, upper) = iter.size_hint();
        assert_eq!(len, 2); // 2 valid capturing groups left
        assert_eq!(upper, Some(2));

        // Consume remaining items
        iter.next();
        iter.next();

        // Test size_hint when no items are left
        let (len, upper) = iter.size_hint();
        assert_eq!(len, 0); // No valid capturing groups left
        assert_eq!(upper, Some(0));
    }
}

#[cfg(test)]
mod tests_llm_16_605 {
    use super::*;

use crate::*;
    use std::vec::Vec;

    #[test]
    fn test_as_slots() {
        let mut locations = Locations(vec![Some(0), Some(5), Some(10), Some(15)]);
        let slots = locations.as_slots();
        
        assert_eq!(slots.len(), 4);
        assert_eq!(slots[0], Some(0));
        assert_eq!(slots[1], Some(5));
        assert_eq!(slots[2], Some(10));
        assert_eq!(slots[3], Some(15));
        
        slots[0] = Some(1);
        assert_eq!(locations.as_slots()[0], Some(1));
    }
}

#[cfg(test)]
mod tests_llm_16_607 {
    use super::*;

use crate::*;
    use re_trait::Locations;

    #[test]
    fn test_len_with_multiple_groups() {
        let locations = Locations(vec![Some(0), Some(5), Some(10), Some(15)]);
        assert_eq!(locations.len(), 2);
    }

    #[test]
    fn test_len_with_single_group() {
        let locations = Locations(vec![Some(0), Some(5)]);
        assert_eq!(locations.len(), 1);
    }

    #[test]
    fn test_len_with_no_groups() {
        let locations = Locations(vec![]);
        assert_eq!(locations.len(), 0);
    }

    #[test]
    fn test_len_with_odd_elements() {
        let locations = Locations(vec![Some(0), Some(5), Some(10)]);
        assert_eq!(locations.len(), 1);
    }
}

#[cfg(test)]
mod tests_llm_16_608 {
    use super::*;

use crate::*;
    use std::vec::Vec;

    #[test]
    fn test_pos_valid_capture_group() {
        let locations = Locations(vec![Some(0), Some(5), Some(6), Some(10)]);
        assert_eq!(locations.pos(1), Some((0, 5)));
    }

    #[test]
    fn test_pos_valid_capture_group_alternate() {
        let locations = Locations(vec![Some(0), Some(4), Some(5), Some(9)]);
        assert_eq!(locations.pos(1), Some((0, 4)));
    }

    #[test]
    fn test_pos_invalid_capture_group() {
        let locations = Locations(vec![Some(0), Some(5)]);
        assert_eq!(locations.pos(2), None);
    }

    #[test]
    fn test_pos_empty_capture_group() {
        let locations = Locations(vec![]);
        assert_eq!(locations.pos(0), None);
    }

    #[test]
    fn test_pos_out_of_bounds_capture_group() {
        let locations = Locations(vec![Some(0), Some(5), Some(6), Some(10)]);
        assert_eq!(locations.pos(10), None);
    }

    #[test]
    fn test_pos_capture_group_not_matched() {
        let locations = Locations(vec![Some(0), None, Some(6), None]);
        assert_eq!(locations.pos(1), None);
    }
}
