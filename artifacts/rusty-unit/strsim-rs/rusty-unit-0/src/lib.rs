//! This library implements string similarity metrics.

#![forbid(unsafe_code)]
#![allow(
    // these casts are sometimes needed. They restrict the length of input iterators
    // but there isn't really any way around this except for always working with
    // 128 bit types
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    // not practical
    clippy::needless_pass_by_value,
    clippy::similar_names,
    // noisy
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    // todo https://github.com/rapidfuzz/strsim-rs/issues/59
    clippy::range_plus_one
)]

extern crate ntest;
extern crate redis;

use std::char;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;
use std::mem;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum StrSimError {
    DifferentLengthArgs,
}

impl Display for StrSimError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        let text = match self {
            StrSimError::DifferentLengthArgs => "Differing length arguments provided",
        };

        write!(fmt, "{}", text)
    }
}

impl Error for StrSimError {}

pub type HammingResult = Result<usize, StrSimError>;

/// Calculates the number of positions in the two sequences where the elements
/// differ. Returns an error if the sequences have different lengths.
pub fn generic_hamming<Iter1, Iter2, Elem1, Elem2>(a: Iter1, b: Iter2) -> HammingResult
where
    Iter1: IntoIterator<Item = Elem1>,
    Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>,
{
    let (mut ita, mut itb) = (a.into_iter(), b.into_iter());
    let mut count = 0;
    loop {
        match (ita.next(), itb.next()) {
            (Some(x), Some(y)) => {
                if x != y {
                    count += 1;
                }
            }
            (None, None) => return Ok(count),
            _ => return Err(StrSimError::DifferentLengthArgs),
        }
    }
}

/// Calculates the number of positions in the two strings where the characters
/// differ. Returns an error if the strings have different lengths.
///
/// ```
/// use strsim::{hamming, StrSimError::DifferentLengthArgs};
///
/// assert_eq!(Ok(3), hamming("hamming", "hammers"));
///
/// assert_eq!(Err(DifferentLengthArgs), hamming("hamming", "ham"));
/// ```
pub fn hamming(a: &str, b: &str) -> HammingResult {
    generic_hamming(a.chars(), b.chars())
}

/// Calculates the Jaro similarity between two sequences. The returned value
/// is between 0.0 and 1.0 (higher value means more similar).
pub fn generic_jaro<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> f64
where
    &'a Iter1: IntoIterator<Item = Elem1>,
    &'b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>,
{
    let a_len = a.into_iter().count();
    let b_len = b.into_iter().count();

    if a_len == 0 && b_len == 0 {
        return 1.0;
    } else if a_len == 0 || b_len == 0 {
        return 0.0;
    }

    let mut search_range = max(a_len, b_len) / 2;
    search_range = search_range.saturating_sub(1);

    // combine memory allocations to reduce runtime
    let mut flags_memory = vec![false; a_len + b_len];
    let (a_flags, b_flags) = flags_memory.split_at_mut(a_len);

    let mut matches = 0_usize;

    for (i, a_elem) in a.into_iter().enumerate() {
        // prevent integer wrapping
        let min_bound = if i > search_range {
            i - search_range
        } else {
            0
        };

        let max_bound = min(b_len, i + search_range + 1);

        for (j, b_elem) in b.into_iter().enumerate().take(max_bound) {
            if min_bound <= j && a_elem == b_elem && !b_flags[j] {
                a_flags[i] = true;
                b_flags[j] = true;
                matches += 1;
                break;
            }
        }
    }

    let mut transpositions = 0_usize;
    if matches != 0 {
        let mut b_iter = b_flags.iter().zip(b);
        for (a_flag, ch1) in a_flags.iter().zip(a) {
            if *a_flag {
                loop {
                    if let Some((b_flag, ch2)) = b_iter.next() {
                        if !*b_flag {
                            continue;
                        }

                        if ch1 != ch2 {
                            transpositions += 1;
                        }
                        break;
                    }
                }
            }
        }
    }
    transpositions /= 2;

    if matches == 0 {
        0.0
    } else {
        ((matches as f64 / a_len as f64)
            + (matches as f64 / b_len as f64)
            + ((matches - transpositions) as f64 / matches as f64))
            / 3.0
    }
}

pub struct StringWrapper<'a>(&'a str);

impl<'a, 'b> IntoIterator for &'a StringWrapper<'b> {
    type Item = char;
    type IntoIter = Chars<'b>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.chars()
    }
}

/// Calculates the Jaro similarity between two strings. The returned value
/// is between 0.0 and 1.0 (higher value means more similar).
///
/// ```
/// use strsim::jaro;
///
/// assert!((0.392 - jaro("Friedrich Nietzsche", "Jean-Paul Sartre")).abs() <
///         0.001);
/// ```
pub fn jaro(a: &str, b: &str) -> f64 {
    generic_jaro(&StringWrapper(a), &StringWrapper(b))
}

/// Like Jaro but gives a boost to sequences that have a common prefix.
pub fn generic_jaro_winkler<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> f64
where
    &'a Iter1: IntoIterator<Item = Elem1>,
    &'b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>,
{
    let sim = generic_jaro(a, b);

    if sim > 0.7 {
        let prefix_length = a
            .into_iter()
            .take(4)
            .zip(b)
            .take_while(|(a_elem, b_elem)| a_elem == b_elem)
            .count();

        sim + 0.1 * prefix_length as f64 * (1.0 - sim)
    } else {
        sim
    }
}

/// Like Jaro but gives a boost to strings that have a common prefix.
///
/// ```
/// use strsim::jaro_winkler;
///
/// assert!((0.866 - jaro_winkler("cheeseburger", "cheese fries")).abs() <
///         0.001);
/// ```
pub fn jaro_winkler(a: &str, b: &str) -> f64 {
    generic_jaro_winkler(&StringWrapper(a), &StringWrapper(b))
}

/// Calculates the minimum number of insertions, deletions, and substitutions
/// required to change one sequence into the other.
///
/// ```
/// use strsim::generic_levenshtein;
///
/// assert_eq!(3, generic_levenshtein(&[1,2,3], &[1,2,3,4,5,6]));
/// ```
pub fn generic_levenshtein<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> usize
where
    &'a Iter1: IntoIterator<Item = Elem1>,
    &'b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>,
{
    let b_len = b.into_iter().count();

    let mut cache: Vec<usize> = (1..b_len + 1).collect();

    let mut result = b_len;

    for (i, a_elem) in a.into_iter().enumerate() {
        result = i + 1;
        let mut distance_b = i;

        for (j, b_elem) in b.into_iter().enumerate() {
            let cost = usize::from(a_elem != b_elem);
            let distance_a = distance_b + cost;
            distance_b = cache[j];
            result = min(result + 1, min(distance_a, distance_b + 1));
            cache[j] = result;
        }
    }

    result
}

/// Calculates the minimum number of insertions, deletions, and substitutions
/// required to change one string into the other.
///
/// ```
/// use strsim::levenshtein;
///
/// assert_eq!(3, levenshtein("kitten", "sitting"));
/// ```
pub fn levenshtein(a: &str, b: &str) -> usize {
    generic_levenshtein(&StringWrapper(a), &StringWrapper(b))
}

/// Calculates a normalized score of the Levenshtein algorithm between 0.0 and
/// 1.0 (inclusive), where 1.0 means the strings are the same.
///
/// ```
/// use strsim::normalized_levenshtein;
///
/// assert!((normalized_levenshtein("kitten", "sitting") - 0.57142).abs() < 0.00001);
/// assert!((normalized_levenshtein("", "") - 1.0).abs() < 0.00001);
/// assert!(normalized_levenshtein("", "second").abs() < 0.00001);
/// assert!(normalized_levenshtein("first", "").abs() < 0.00001);
/// assert!((normalized_levenshtein("string", "string") - 1.0).abs() < 0.00001);
/// ```
pub fn normalized_levenshtein(a: &str, b: &str) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    1.0 - (levenshtein(a, b) as f64) / (a.chars().count().max(b.chars().count()) as f64)
}

/// Like Levenshtein but allows for adjacent transpositions. Each substring can
/// only be edited once.
///
/// ```
/// use strsim::osa_distance;
///
/// assert_eq!(3, osa_distance("ab", "bca"));
/// ```
pub fn osa_distance(a: &str, b: &str) -> usize {
    let b_len = b.chars().count();
    // 0..=b_len behaves like 0..b_len.saturating_add(1) which could be a different size
    // this leads to significantly worse code gen when swapping the vectors below
    let mut prev_two_distances: Vec<usize> = (0..b_len + 1).collect();
    let mut prev_distances: Vec<usize> = (0..b_len + 1).collect();
    let mut curr_distances: Vec<usize> = vec![0; b_len + 1];

    let mut prev_a_char = char::MAX;
    let mut prev_b_char = char::MAX;

    for (i, a_char) in a.chars().enumerate() {
        curr_distances[0] = i + 1;

        for (j, b_char) in b.chars().enumerate() {
            let cost = usize::from(a_char != b_char);
            curr_distances[j + 1] = min(
                curr_distances[j] + 1,
                min(prev_distances[j + 1] + 1, prev_distances[j] + cost),
            );
            if i > 0 && j > 0 && a_char != b_char && a_char == prev_b_char && b_char == prev_a_char
            {
                curr_distances[j + 1] = min(curr_distances[j + 1], prev_two_distances[j - 1] + 1);
            }

            prev_b_char = b_char;
        }

        mem::swap(&mut prev_two_distances, &mut prev_distances);
        mem::swap(&mut prev_distances, &mut curr_distances);
        prev_a_char = a_char;
    }

    // access prev_distances instead of curr_distances since we swapped
    // them above. In case a is empty this would still contain the correct value
    // from initializing the last element to b_len
    prev_distances[b_len]
}

/* Returns the final index for a value in a single vector that represents a fixed
2d grid */
fn flat_index(i: usize, j: usize, width: usize) -> usize {
    j * width + i
}

/// Like optimal string alignment, but substrings can be edited an unlimited
/// number of times, and the triangle inequality holds.
///
/// ```
/// use strsim::generic_damerau_levenshtein;
///
/// assert_eq!(2, generic_damerau_levenshtein(&[1,2], &[2,3,1]));
/// ```
pub fn generic_damerau_levenshtein<Elem>(a_elems: &[Elem], b_elems: &[Elem]) -> usize
where
    Elem: Eq + Hash + Clone,
{
    let a_len = a_elems.len();
    let b_len = b_elems.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let width = a_len + 2;
    let mut distances = vec![0; (a_len + 2) * (b_len + 2)];
    let max_distance = a_len + b_len;
    distances[0] = max_distance;

    for i in 0..(a_len + 1) {
        distances[flat_index(i + 1, 0, width)] = max_distance;
        distances[flat_index(i + 1, 1, width)] = i;
    }

    for j in 0..(b_len + 1) {
        distances[flat_index(0, j + 1, width)] = max_distance;
        distances[flat_index(1, j + 1, width)] = j;
    }

    let mut elems: HashMap<Elem, usize> = HashMap::with_capacity(64);

    for i in 1..(a_len + 1) {
        let mut db = 0;

        for j in 1..(b_len + 1) {
            let k = match elems.get(&b_elems[j - 1]) {
                Some(&value) => value,
                None => 0,
            };

            let insertion_cost = distances[flat_index(i, j + 1, width)] + 1;
            let deletion_cost = distances[flat_index(i + 1, j, width)] + 1;
            let transposition_cost =
                distances[flat_index(k, db, width)] + (i - k - 1) + 1 + (j - db - 1);

            let mut substitution_cost = distances[flat_index(i, j, width)] + 1;
            if a_elems[i - 1] == b_elems[j - 1] {
                db = j;
                substitution_cost -= 1;
            }

            distances[flat_index(i + 1, j + 1, width)] = min(
                substitution_cost,
                min(insertion_cost, min(deletion_cost, transposition_cost)),
            );
        }

        elems.insert(a_elems[i - 1].clone(), i);
    }

    distances[flat_index(a_len + 1, b_len + 1, width)]
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RowId {
    val: isize,
}

impl Default for RowId {
    fn default() -> Self {
        Self { val: -1 }
    }
}

#[derive(Default, Clone)]
pub struct GrowingHashmapMapElemChar<ValueType> {
    key: u32,
    value: ValueType,
}

/// specialized hashmap to store user provided types
/// this implementation relies on a couple of base assumptions in order to simplify the implementation
/// - the hashmap does not have an upper limit of included items
/// - the default value for the `ValueType` can be used as a dummy value to indicate an empty cell
/// - elements can't be removed
/// - only allocates memory on first write access.
///   This improves performance for hashmaps that are never written to
pub struct GrowingHashmapChar<ValueType> {
    used: i32,
    fill: i32,
    mask: i32,
    map: Option<Vec<GrowingHashmapMapElemChar<ValueType>>>,
}

impl<ValueType> Default for GrowingHashmapChar<ValueType>
where
    ValueType: Default + Clone + Eq,
{
    fn default() -> Self {
        Self {
            used: 0,
            fill: 0,
            mask: -1,
            map: None,
        }
    }
}

impl<ValueType> GrowingHashmapChar<ValueType>
where
    ValueType: Default + Clone + Eq + Copy,
{
    fn get(&self, key: u32) -> ValueType {
        self.map
            .as_ref()
            .map_or_else(|| Default::default(), |map| map[self.lookup(key)].value)
    }

    fn get_mut(&mut self, key: u32) -> &mut ValueType {
        if self.map.is_none() {
            self.allocate();
        }

        let mut i = self.lookup(key);
        if self
            .map
            .as_ref()
            .expect("map should have been created above")[i]
            .value
            == Default::default()
        {
            self.fill += 1;
            // resize when 2/3 full
            if self.fill * 3 >= (self.mask + 1) * 2 {
                self.grow((self.used + 1) * 2);
                i = self.lookup(key);
            }

            self.used += 1;
        }

        let elem = &mut self
            .map
            .as_mut()
            .expect("map should have been created above")[i];
        elem.key = key;
        &mut elem.value
    }

    fn allocate(&mut self) {
        self.mask = 8 - 1;
        self.map = Some(vec![GrowingHashmapMapElemChar::default(); 8]);
    }

    /// lookup key inside the hashmap using a similar collision resolution
    /// strategy to `CPython` and `Ruby`
    fn lookup(&self, key: u32) -> usize {
        let hash = key;
        let mut i = hash as usize & self.mask as usize;

        let map = self
            .map
            .as_ref()
            .expect("callers have to ensure map is allocated");

        if map[i].value == Default::default() || map[i].key == key {
            return i;
        }

        let mut perturb = key;
        loop {
            i = (i * 5 + perturb as usize + 1) & self.mask as usize;

            if map[i].value == Default::default() || map[i].key == key {
                return i;
            }

            perturb >>= 5;
        }
    }

    fn grow(&mut self, min_used: i32) {
        let mut new_size = self.mask + 1;
        while new_size <= min_used {
            new_size <<= 1;
        }

        self.fill = self.used;
        self.mask = new_size - 1;

        let old_map = std::mem::replace(
            self.map
                .as_mut()
                .expect("callers have to ensure map is allocated"),
            vec![GrowingHashmapMapElemChar::<ValueType>::default(); new_size as usize],
        );

        for elem in old_map {
            if elem.value != Default::default() {
                let j = self.lookup(elem.key);
                let new_elem = &mut self.map.as_mut().expect("map created above")[j];
                new_elem.key = elem.key;
                new_elem.value = elem.value;
                self.used -= 1;
                if self.used == 0 {
                    break;
                }
            }
        }

        self.used = self.fill;
    }
}

struct HybridGrowingHashmapChar<ValueType> {
    map: GrowingHashmapChar<ValueType>,
    extended_ascii: [ValueType; 256],
}

impl<ValueType> HybridGrowingHashmapChar<ValueType>
where
    ValueType: Default + Clone + Copy + Eq,
{
    fn get(&self, key: char) -> ValueType {
        let value = key as u32;
        if value <= 255 {
            let val_u8 = u8::try_from(value).expect("we check the bounds above");
            self.extended_ascii[usize::from(val_u8)]
        } else {
            self.map.get(value)
        }
    }

    fn get_mut(&mut self, key: char) -> &mut ValueType {
        let value = key as u32;
        if value <= 255 {
            let val_u8 = u8::try_from(value).expect("we check the bounds above");
            &mut self.extended_ascii[usize::from(val_u8)]
        } else {
            self.map.get_mut(value)
        }
    }
}

impl<ValueType> Default for HybridGrowingHashmapChar<ValueType>
where
    ValueType: Default + Clone + Copy + Eq,
{
    fn default() -> Self {
        HybridGrowingHashmapChar {
            map: GrowingHashmapChar::default(),
            extended_ascii: [Default::default(); 256],
        }
    }
}

pub fn damerau_levenshtein_impl<Iter1, Iter2>(s1: Iter1, len1: usize, s2: Iter2, len2: usize) -> usize
where
    Iter1: Iterator<Item = char> + Clone,
    Iter2: Iterator<Item = char> + Clone,
{
    // The implementations is based on the paper
    // `Linear space string correction algorithm using the Damerau-Levenshtein distance`
    // from Chunchun Zhao and Sartaj Sahni
    //
    // It has a runtime complexity of `O(N*M)` and a memory usage of `O(N+M)`.
    let max_val = max(len1, len2) as isize + 1;

    let mut last_row_id = HybridGrowingHashmapChar::<RowId>::default();

    let size = len2 + 2;
    let mut fr = vec![max_val; size];
    let mut r1 = vec![max_val; size];
    let mut r: Vec<isize> = (max_val..max_val + 1)
        .chain(0..(size - 1) as isize)
        .collect();

    for (i, ch1) in s1.enumerate().map(|(i, ch1)| (i + 1, ch1)) {
        mem::swap(&mut r, &mut r1);
        let mut last_col_id: isize = -1;
        let mut last_i2l1 = r[1];
        r[1] = i as isize;
        let mut t = max_val;

        for (j, ch2) in s2.clone().enumerate().map(|(j, ch2)| (j + 1, ch2)) {
            let diag = r1[j] + isize::from(ch1 != ch2);
            let left = r[j] + 1;
            let up = r1[j + 1] + 1;
            let mut temp = min(diag, min(left, up));

            if ch1 == ch2 {
                last_col_id = j as isize; // last occurence of s1_i
                fr[j + 1] = r1[j - 1]; // save H_k-1,j-2
                t = last_i2l1; // save H_i-2,l-1
            } else {
                let k = last_row_id.get(ch2).val;
                let l = last_col_id;

                if j as isize - l == 1 {
                    let transpose = fr[j + 1] + (i as isize - k);
                    temp = min(temp, transpose);
                } else if i as isize - k == 1 {
                    let transpose = t + (j as isize - l);
                    temp = min(temp, transpose);
                }
            }

            last_i2l1 = r[j + 1];
            r[j + 1] = temp;
        }
        last_row_id.get_mut(ch1).val = i as isize;
    }

    r[len2 + 1] as usize
}

/// Like optimal string alignment, but substrings can be edited an unlimited
/// number of times, and the triangle inequality holds.
///
/// ```
/// use strsim::damerau_levenshtein;
///
/// assert_eq!(2, damerau_levenshtein("ab", "bca"));
/// ```
pub fn damerau_levenshtein(a: &str, b: &str) -> usize {
    damerau_levenshtein_impl(a.chars(), a.chars().count(), b.chars(), b.chars().count())
}

/// Calculates a normalized score of the Damerau–Levenshtein algorithm between
/// 0.0 and 1.0 (inclusive), where 1.0 means the strings are the same.
///
/// ```
/// use strsim::normalized_damerau_levenshtein;
///
/// assert!((normalized_damerau_levenshtein("levenshtein", "löwenbräu") - 0.27272).abs() < 0.00001);
/// assert!((normalized_damerau_levenshtein("", "") - 1.0).abs() < 0.00001);
/// assert!(normalized_damerau_levenshtein("", "flower").abs() < 0.00001);
/// assert!(normalized_damerau_levenshtein("tree", "").abs() < 0.00001);
/// assert!((normalized_damerau_levenshtein("sunglasses", "sunglasses") - 1.0).abs() < 0.00001);
/// ```
pub fn normalized_damerau_levenshtein(a: &str, b: &str) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }

    let len1 = a.chars().count();
    let len2 = b.chars().count();
    let dist = damerau_levenshtein_impl(a.chars(), len1, b.chars(), len2);
    1.0 - (dist as f64) / (max(len1, len2) as f64)
}

/// Returns an Iterator of char tuples.
fn bigrams(s: &str) -> impl Iterator<Item = (char, char)> + '_ {
    s.chars().zip(s.chars().skip(1))
}

/// Calculates a Sørensen-Dice similarity distance using bigrams.
/// See <https://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient>.
///
/// ```
/// use strsim::sorensen_dice;
///
/// assert_eq!(1.0, sorensen_dice("", ""));
/// assert_eq!(0.0, sorensen_dice("", "a"));
/// assert_eq!(0.0, sorensen_dice("french", "quebec"));
/// assert_eq!(1.0, sorensen_dice("ferris", "ferris"));
/// assert_eq!(0.8888888888888888, sorensen_dice("feris", "ferris"));
/// ```
pub fn sorensen_dice(a: &str, b: &str) -> f64 {
    // implementation guided by
    // https://github.com/aceakash/string-similarity/blob/f83ba3cd7bae874c20c429774e911ae8cff8bced/src/index.js#L6

    let a: String = a.chars().filter(|&x| !char::is_whitespace(x)).collect();
    let b: String = b.chars().filter(|&x| !char::is_whitespace(x)).collect();

    if a == b {
        return 1.0;
    }

    if a.len() < 2 || b.len() < 2 {
        return 0.0;
    }

    let mut a_bigrams: HashMap<(char, char), usize> = HashMap::new();

    for bigram in bigrams(&a) {
        *a_bigrams.entry(bigram).or_insert(0) += 1;
    }

    let mut intersection_size = 0_usize;

    for bigram in bigrams(&b) {
        a_bigrams.entry(bigram).and_modify(|bi| {
            if *bi > 0 {
                *bi -= 1;
                intersection_size += 1;
            }
        });
    }

    (2 * intersection_size) as f64 / (a.len() + b.len() - 2) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_delta {
        ($x:expr, $y:expr) => {
            assert_delta!($x, $y, 1e-5);
        };
        ($x:expr, $y:expr, $d:expr) => {
            if ($x - $y).abs() > $d {
                panic!(
                    "assertion failed: actual: `{}`, expected: `{}`: \
                    actual not within < {} of expected",
                    $x, $y, $d
                );
            }
        };
    }

    #[test]
    fn bigrams_iterator() {
        let mut bi = bigrams("abcde");

        assert_eq!(Some(('a', 'b')), bi.next());
        assert_eq!(Some(('b', 'c')), bi.next());
        assert_eq!(Some(('c', 'd')), bi.next());
        assert_eq!(Some(('d', 'e')), bi.next());
        assert_eq!(None, bi.next());
    }

    fn assert_hamming_dist(dist: usize, str1: &str, str2: &str) {
        assert_eq!(Ok(dist), hamming(str1, str2));
    }

    #[test]
    fn hamming_empty() {
        assert_hamming_dist(0, "", "")
    }

    #[test]
    fn hamming_same() {
        assert_hamming_dist(0, "hamming", "hamming")
    }

    #[test]
    fn hamming_numbers() {
        assert_eq!(Ok(1), generic_hamming(&[1, 2, 4], &[1, 2, 3]));
    }

    #[test]
    fn hamming_diff() {
        assert_hamming_dist(3, "hamming", "hammers")
    }

    #[test]
    fn hamming_diff_multibyte() {
        assert_hamming_dist(2, "hamming", "h香mmüng");
    }

    #[test]
    fn hamming_unequal_length() {
        assert_eq!(
            Err(StrSimError::DifferentLengthArgs),
            generic_hamming("ham".chars(), "hamming".chars())
        );
    }

    #[test]
    fn hamming_names() {
        assert_hamming_dist(14, "Friedrich Nietzs", "Jean-Paul Sartre")
    }

    #[test]
    fn jaro_both_empty() {
        assert_eq!(1.0, jaro("", ""));
    }

    #[test]
    fn jaro_first_empty() {
        assert_eq!(0.0, jaro("", "jaro"));
    }

    #[test]
    fn jaro_second_empty() {
        assert_eq!(0.0, jaro("distance", ""));
    }

    #[test]
    fn jaro_same() {
        assert_eq!(1.0, jaro("jaro", "jaro"));
    }

    #[test]
    fn jaro_multibyte() {
        assert_delta!(0.818, jaro("testabctest", "testöঙ香test"), 0.001);
        assert_delta!(0.818, jaro("testöঙ香test", "testabctest"), 0.001);
    }

    #[test]
    fn jaro_diff_short() {
        assert_delta!(0.767, jaro("dixon", "dicksonx"), 0.001);
    }

    #[test]
    fn jaro_diff_one_character() {
        assert_eq!(0.0, jaro("a", "b"));
    }

    #[test]
    fn jaro_same_one_character() {
        assert_eq!(1.0, jaro("a", "a"));
    }

    #[test]
    fn generic_jaro_diff() {
        assert_eq!(0.0, generic_jaro(&[1, 2], &[3, 4]));
    }

    #[test]
    fn jaro_diff_one_and_two() {
        assert_delta!(0.83, jaro("a", "ab"), 0.01);
    }

    #[test]
    fn jaro_diff_two_and_one() {
        assert_delta!(0.83, jaro("ab", "a"), 0.01);
    }

    #[test]
    fn jaro_diff_no_transposition() {
        assert_delta!(0.822, jaro("dwayne", "duane"), 0.001);
    }

    #[test]
    fn jaro_diff_with_transposition() {
        assert_delta!(0.944, jaro("martha", "marhta"), 0.001);
        assert_delta!(0.6, jaro("a jke", "jane a k"), 0.001);
    }

    #[test]
    fn jaro_names() {
        assert_delta!(
            0.392,
            jaro("Friedrich Nietzsche", "Jean-Paul Sartre"),
            0.001
        );
    }

    #[test]
    fn jaro_winkler_both_empty() {
        assert_eq!(1.0, jaro_winkler("", ""));
    }

    #[test]
    fn jaro_winkler_first_empty() {
        assert_eq!(0.0, jaro_winkler("", "jaro-winkler"));
    }

    #[test]
    fn jaro_winkler_second_empty() {
        assert_eq!(0.0, jaro_winkler("distance", ""));
    }

    #[test]
    fn jaro_winkler_same() {
        assert_eq!(1.0, jaro_winkler("Jaro-Winkler", "Jaro-Winkler"));
    }

    #[test]
    fn jaro_winkler_multibyte() {
        assert_delta!(0.89, jaro_winkler("testabctest", "testöঙ香test"), 0.001);
        assert_delta!(0.89, jaro_winkler("testöঙ香test", "testabctest"), 0.001);
    }

    #[test]
    fn jaro_winkler_diff_short() {
        assert_delta!(0.813, jaro_winkler("dixon", "dicksonx"), 0.001);
        assert_delta!(0.813, jaro_winkler("dicksonx", "dixon"), 0.001);
    }

    #[test]
    fn jaro_winkler_diff_one_character() {
        assert_eq!(0.0, jaro_winkler("a", "b"));
    }

    #[test]
    fn jaro_winkler_same_one_character() {
        assert_eq!(1.0, jaro_winkler("a", "a"));
    }

    #[test]
    fn jaro_winkler_diff_no_transposition() {
        assert_delta!(0.84, jaro_winkler("dwayne", "duane"), 0.001);
    }

    #[test]
    fn jaro_winkler_diff_with_transposition() {
        assert_delta!(0.961, jaro_winkler("martha", "marhta"), 0.001);
        assert_delta!(0.6, jaro_winkler("a jke", "jane a k"), 0.001);
    }

    #[test]
    fn jaro_winkler_names() {
        assert_delta!(
            0.452,
            jaro_winkler("Friedrich Nietzsche", "Fran-Paul Sartre"),
            0.001
        );
    }

    #[test]
    fn jaro_winkler_long_prefix() {
        assert_delta!(0.866, jaro_winkler("cheeseburger", "cheese fries"), 0.001);
    }

    #[test]
    fn jaro_winkler_more_names() {
        assert_delta!(0.868, jaro_winkler("Thorkel", "Thorgier"), 0.001);
    }

    #[test]
    fn jaro_winkler_length_of_one() {
        assert_delta!(0.738, jaro_winkler("Dinsdale", "D"), 0.001);
    }

    #[test]
    fn jaro_winkler_very_long_prefix() {
        assert_delta!(
            0.98519,
            jaro_winkler("thequickbrownfoxjumpedoverx", "thequickbrownfoxjumpedovery")
        );
    }

    #[test]
    fn levenshtein_empty() {
        assert_eq!(0, levenshtein("", ""));
    }

    #[test]
    fn levenshtein_same() {
        assert_eq!(0, levenshtein("levenshtein", "levenshtein"));
    }

    #[test]
    fn levenshtein_diff_short() {
        assert_eq!(3, levenshtein("kitten", "sitting"));
    }

    #[test]
    fn levenshtein_diff_with_space() {
        assert_eq!(5, levenshtein("hello, world", "bye, world"));
    }

    #[test]
    fn levenshtein_diff_multibyte() {
        assert_eq!(3, levenshtein("öঙ香", "abc"));
        assert_eq!(3, levenshtein("abc", "öঙ香"));
    }

    #[test]
    fn levenshtein_diff_longer() {
        let a = "The quick brown fox jumped over the angry dog.";
        let b = "Lorem ipsum dolor sit amet, dicta latine an eam.";
        assert_eq!(37, levenshtein(a, b));
    }

    #[test]
    fn levenshtein_first_empty() {
        assert_eq!(7, levenshtein("", "sitting"));
    }

    #[test]
    fn levenshtein_second_empty() {
        assert_eq!(6, levenshtein("kitten", ""));
    }

    #[test]
    fn normalized_levenshtein_diff_short() {
        assert_delta!(0.57142, normalized_levenshtein("kitten", "sitting"));
    }

    #[test]
    fn normalized_levenshtein_for_empty_strings() {
        assert_delta!(1.0, normalized_levenshtein("", ""));
    }

    #[test]
    fn normalized_levenshtein_first_empty() {
        assert_delta!(0.0, normalized_levenshtein("", "second"));
    }

    #[test]
    fn normalized_levenshtein_second_empty() {
        assert_delta!(0.0, normalized_levenshtein("first", ""));
    }

    #[test]
    fn normalized_levenshtein_identical_strings() {
        assert_delta!(1.0, normalized_levenshtein("identical", "identical"));
    }

    #[test]
    fn osa_distance_empty() {
        assert_eq!(0, osa_distance("", ""));
    }

    #[test]
    fn osa_distance_same() {
        assert_eq!(0, osa_distance("damerau", "damerau"));
    }

    #[test]
    fn osa_distance_first_empty() {
        assert_eq!(7, osa_distance("", "damerau"));
    }

    #[test]
    fn osa_distance_second_empty() {
        assert_eq!(7, osa_distance("damerau", ""));
    }

    #[test]
    fn osa_distance_diff() {
        assert_eq!(3, osa_distance("ca", "abc"));
    }

    #[test]
    fn osa_distance_diff_short() {
        assert_eq!(3, osa_distance("damerau", "aderua"));
    }

    #[test]
    fn osa_distance_diff_reversed() {
        assert_eq!(3, osa_distance("aderua", "damerau"));
    }

    #[test]
    fn osa_distance_diff_multibyte() {
        assert_eq!(3, osa_distance("öঙ香", "abc"));
        assert_eq!(3, osa_distance("abc", "öঙ香"));
    }

    #[test]
    fn osa_distance_diff_unequal_length() {
        assert_eq!(6, osa_distance("damerau", "aderuaxyz"));
    }

    #[test]
    fn osa_distance_diff_unequal_length_reversed() {
        assert_eq!(6, osa_distance("aderuaxyz", "damerau"));
    }

    #[test]
    fn osa_distance_diff_comedians() {
        assert_eq!(5, osa_distance("Stewart", "Colbert"));
    }

    #[test]
    fn osa_distance_many_transpositions() {
        assert_eq!(4, osa_distance("abcdefghijkl", "bacedfgihjlk"));
    }

    #[test]
    fn osa_distance_diff_longer() {
        let a = "The quick brown fox jumped over the angry dog.";
        let b = "Lehem ipsum dolor sit amet, dicta latine an eam.";
        assert_eq!(36, osa_distance(a, b));
    }

    #[test]
    fn osa_distance_beginning_transposition() {
        assert_eq!(1, osa_distance("foobar", "ofobar"));
    }

    #[test]
    fn osa_distance_end_transposition() {
        assert_eq!(1, osa_distance("specter", "spectre"));
    }

    #[test]
    fn osa_distance_restricted_edit() {
        assert_eq!(4, osa_distance("a cat", "an abct"));
    }

    #[test]
    fn damerau_levenshtein_empty() {
        assert_eq!(0, damerau_levenshtein("", ""));
    }

    #[test]
    fn damerau_levenshtein_same() {
        assert_eq!(0, damerau_levenshtein("damerau", "damerau"));
    }

    #[test]
    fn damerau_levenshtein_first_empty() {
        assert_eq!(7, damerau_levenshtein("", "damerau"));
    }

    #[test]
    fn damerau_levenshtein_second_empty() {
        assert_eq!(7, damerau_levenshtein("damerau", ""));
    }

    #[test]
    fn damerau_levenshtein_diff() {
        assert_eq!(2, damerau_levenshtein("ca", "abc"));
    }

    #[test]
    fn damerau_levenshtein_diff_short() {
        assert_eq!(3, damerau_levenshtein("damerau", "aderua"));
    }

    #[test]
    fn damerau_levenshtein_diff_reversed() {
        assert_eq!(3, damerau_levenshtein("aderua", "damerau"));
    }

    #[test]
    fn damerau_levenshtein_diff_multibyte() {
        assert_eq!(3, damerau_levenshtein("öঙ香", "abc"));
        assert_eq!(3, damerau_levenshtein("abc", "öঙ香"));
    }

    #[test]
    fn damerau_levenshtein_diff_unequal_length() {
        assert_eq!(6, damerau_levenshtein("damerau", "aderuaxyz"));
    }

    #[test]
    fn damerau_levenshtein_diff_unequal_length_reversed() {
        assert_eq!(6, damerau_levenshtein("aderuaxyz", "damerau"));
    }

    #[test]
    fn damerau_levenshtein_diff_comedians() {
        assert_eq!(5, damerau_levenshtein("Stewart", "Colbert"));
    }

    #[test]
    fn damerau_levenshtein_many_transpositions() {
        assert_eq!(4, damerau_levenshtein("abcdefghijkl", "bacedfgihjlk"));
    }

    #[test]
    fn damerau_levenshtein_diff_longer() {
        let a = "The quick brown fox jumped over the angry dog.";
        let b = "Lehem ipsum dolor sit amet, dicta latine an eam.";
        assert_eq!(36, damerau_levenshtein(a, b));
    }

    #[test]
    fn damerau_levenshtein_beginning_transposition() {
        assert_eq!(1, damerau_levenshtein("foobar", "ofobar"));
    }

    #[test]
    fn damerau_levenshtein_end_transposition() {
        assert_eq!(1, damerau_levenshtein("specter", "spectre"));
    }

    #[test]
    fn damerau_levenshtein_unrestricted_edit() {
        assert_eq!(3, damerau_levenshtein("a cat", "an abct"));
    }

    #[test]
    fn normalized_damerau_levenshtein_diff_short() {
        assert_delta!(
            0.27272,
            normalized_damerau_levenshtein("levenshtein", "löwenbräu")
        );
    }

    #[test]
    fn normalized_damerau_levenshtein_for_empty_strings() {
        assert_delta!(1.0, normalized_damerau_levenshtein("", ""));
    }

    #[test]
    fn normalized_damerau_levenshtein_first_empty() {
        assert_delta!(0.0, normalized_damerau_levenshtein("", "flower"));
    }

    #[test]
    fn normalized_damerau_levenshtein_second_empty() {
        assert_delta!(0.0, normalized_damerau_levenshtein("tree", ""));
    }

    #[test]
    fn normalized_damerau_levenshtein_identical_strings() {
        assert_delta!(
            1.0,
            normalized_damerau_levenshtein("sunglasses", "sunglasses")
        );
    }

    #[test]
    fn sorensen_dice_all() {
        // test cases taken from
        // https://github.com/aceakash/string-similarity/blob/f83ba3cd7bae874c20c429774e911ae8cff8bced/src/spec/index.spec.js#L11

        assert_delta!(1.0, sorensen_dice("a", "a"));
        assert_delta!(0.0, sorensen_dice("a", "b"));
        assert_delta!(1.0, sorensen_dice("", ""));
        assert_delta!(0.0, sorensen_dice("a", ""));
        assert_delta!(0.0, sorensen_dice("", "a"));
        assert_delta!(1.0, sorensen_dice("apple event", "apple    event"));
        assert_delta!(0.90909, sorensen_dice("iphone", "iphone x"));
        assert_delta!(0.0, sorensen_dice("french", "quebec"));
        assert_delta!(1.0, sorensen_dice("france", "france"));
        assert_delta!(0.2, sorensen_dice("fRaNce", "france"));
        assert_delta!(0.8, sorensen_dice("healed", "sealed"));
        assert_delta!(
            0.78788,
            sorensen_dice("web applications", "applications of the web")
        );
        assert_delta!(
            0.92,
            sorensen_dice(
                "this will have a typo somewhere",
                "this will huve a typo somewhere"
            )
        );
        assert_delta!(
            0.60606,
            sorensen_dice(
                "Olive-green table for sale, in extremely good condition.",
                "For sale: table in very good  condition, olive green in colour."
            )
        );
        assert_delta!(
            0.25581,
            sorensen_dice(
                "Olive-green table for sale, in extremely good condition.",
                "For sale: green Subaru Impreza, 210,000 miles"
            )
        );
        assert_delta!(
            0.14118,
            sorensen_dice(
                "Olive-green table for sale, in extremely good condition.",
                "Wanted: mountain bike with at least 21 gears."
            )
        );
        assert_delta!(
            0.77419,
            sorensen_dice("this has one extra word", "this has one word")
        );
    }
}

pub use ntest::timeout;
pub mod rusty_monitor;

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_0() {
    rusty_monitor::set_test_id(0);
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<isize>>> = std::option::Option::None;
    let mut i32_0: i32 = -17830i32;
    let mut i32_1: i32 = 13796i32;
    let mut i32_2: i32 = -3551i32;
    let mut i32_3: i32 = 13672i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<usize> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<usize> = &mut growinghashmapchar_0;
    let mut u32_0: u32 = 5865u32;
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<u128> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_1_ref_0: &crate::GrowingHashmapChar<u128> = &mut growinghashmapchar_1;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut isize_0: isize = -13692isize;
    let mut rowid_1: crate::RowId = crate::RowId {val: isize_0};
    let mut rowid_1_ref_0: &crate::RowId = &mut rowid_1;
    let mut str_0: &str = "hovmlyaTO";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "smy";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut char_0: char = 'B';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<u8> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &crate::HybridGrowingHashmapChar<u8> = &mut hybridgrowinghashmapchar_0;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut u8_0: u8 = crate::HybridGrowingHashmapChar::get(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut f64_0: f64 = crate::normalized_levenshtein(str_1_ref_0, str_0_ref_0);
    let mut bool_0: bool = crate::RowId::eq(rowid_1_ref_0, rowid_0_ref_0);
    let mut u128_0: u128 = crate::GrowingHashmapChar::get(growinghashmapchar_1_ref_0, u32_0);
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<i8> = crate::GrowingHashmapMapElemChar::default();
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    crate::GrowingHashmapChar::grow(growinghashmapchar_0_ref_0, i32_3);
    let mut growinghashmapchar_2: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1() {
    rusty_monitor::set_test_id(1);
    let mut str_0: &str = "u1SMzpW29WN9iNw2e";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "tSwjJRGL3wOe";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "juB2BEpQxcCL9EsNqmW";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "RxxdaknONHQHJebF10y";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<f32> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<f32> = &mut growinghashmapmapelemchar_0;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut str_4: &str = "p";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "GrnU4PiutA";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<isize>>> = std::option::Option::None;
    let mut i32_0: i32 = -11883i32;
    let mut i32_1: i32 = -2400i32;
    let mut i32_2: i32 = -9792i32;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut strsimerror_2: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_2_ref_0: &StrSimError = &mut strsimerror_2;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    crate::hamming(str_5_ref_0, str_4_ref_0);
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<f32> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    crate::hamming(str_3_ref_0, str_2_ref_0);
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<u8> = crate::HybridGrowingHashmapChar::default();
    let mut growinghashmapmapelemchar_2: crate::GrowingHashmapMapElemChar<f32> = crate::GrowingHashmapMapElemChar::default();
    let mut f64_0: f64 = crate::jaro_winkler(str_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2() {
    rusty_monitor::set_test_id(2);
    let mut u32_0: u32 = 9056u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<usize>>> = std::option::Option::None;
    let mut i32_0: i32 = -13976i32;
    let mut i32_1: i32 = 10074i32;
    let mut i32_2: i32 = -6013i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<usize> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &crate::GrowingHashmapChar<usize> = &mut growinghashmapchar_0;
    let mut u32_1: u32 = 5642u32;
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<i64> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_1_ref_0: &crate::GrowingHashmapChar<i64> = &mut growinghashmapchar_1;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut strsimerror_2: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_2_ref_0: &StrSimError = &mut strsimerror_2;
    let mut str_0: &str = "IUlp";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut strsimerror_3: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_3_ref_0: &StrSimError = &mut strsimerror_3;
    let mut str_2: &str = "X9b8EcXLPN6K";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "axmixu1Nd71AsrgO";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut usize_0: usize = crate::osa_distance(str_3_ref_0, str_2_ref_0);
    let mut f64_0: f64 = crate::jaro_winkler(str_1_ref_0, str_0_ref_0);
    let mut i64_0: i64 = crate::GrowingHashmapChar::get(growinghashmapchar_1_ref_0, u32_1);
    let mut usize_1: usize = crate::GrowingHashmapChar::get(growinghashmapchar_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3() {
    rusty_monitor::set_test_id(3);
    let mut isize_0: isize = -7690isize;
    let mut rowid_0: crate::RowId = crate::RowId {val: isize_0};
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut rowid_1: crate::RowId = crate::RowId::default();
    let mut rowid_1_ref_0: &crate::RowId = &mut rowid_1;
    let mut str_0: &str = "oWmfnV98JO";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Ep6V";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u32_0: u32 = 6003u32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u32> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_0_ref_0: &crate::GrowingHashmapChar<u32> = &mut growinghashmapchar_0;
    let mut isize_1: isize = 12574isize;
    let mut u32_1: u32 = 350u32;
    let mut isize_2: isize = -2992isize;
    let mut rowid_2: crate::RowId = crate::RowId {val: isize_2};
    let mut rowid_2_ref_0: &crate::RowId = &mut rowid_2;
    let mut str_2: &str = "cF";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "ZO52e3gthCy6ATu";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut rowid_3: crate::RowId = crate::RowId::default();
    let mut usize_0: usize = crate::osa_distance(str_3_ref_0, str_2_ref_0);
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<u64> = crate::GrowingHashmapMapElemChar::default();
    let mut tuple_0: () = crate::RowId::assert_receiver_is_total_eq(rowid_2_ref_0);
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<isize> = crate::GrowingHashmapMapElemChar {key: u32_1, value: isize_1};
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut usize_1: usize = crate::GrowingHashmapChar::lookup(growinghashmapchar_0_ref_0, u32_0);
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<i128> = crate::HybridGrowingHashmapChar::default();
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut f64_0: f64 = crate::sorensen_dice(str_1_ref_0, str_0_ref_0);
    let mut bool_0: bool = crate::RowId::eq(rowid_1_ref_0, rowid_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4() {
    rusty_monitor::set_test_id(4);
    let mut usize_0: usize = 7611usize;
    let mut usize_1: usize = 2623usize;
    let mut usize_2: usize = 8218usize;
    let mut str_0: &str = "XOPnXdp";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "FshBpDz";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut str_2: &str = "rWtv9xPeZEZZZ";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "F4axfzkd2a";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "Yh4Jg8qzhA";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "E";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut isize_0: isize = 10736isize;
    let mut rowid_1: crate::RowId = crate::RowId {val: isize_0};
    let mut rowid_1_ref_0: &crate::RowId = &mut rowid_1;
    let mut rowid_2: crate::RowId = crate::RowId::default();
    let mut rowid_2_ref_0: &crate::RowId = &mut rowid_2;
    let mut bool_0: bool = crate::RowId::eq(rowid_2_ref_0, rowid_1_ref_0);
    let mut f64_0: f64 = crate::jaro(str_5_ref_0, str_4_ref_0);
    let mut usize_3: usize = crate::osa_distance(str_3_ref_0, str_2_ref_0);
    let mut rowid_3: crate::RowId = crate::RowId::clone(rowid_0_ref_0);
    let mut usize_4: usize = crate::levenshtein(str_1_ref_0, str_0_ref_0);
    let mut usize_5: usize = crate::flat_index(usize_2, usize_1, usize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5() {
    rusty_monitor::set_test_id(5);
    let mut str_0: &str = "44useqri6O";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "XhzasS";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "1hjZ";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "KnFR5zS";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "L5ssyB";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "gSpIds5Hcw0bZO";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut char_0: char = 'J';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<i8> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &mut crate::HybridGrowingHashmapChar<i8> = &mut hybridgrowinghashmapchar_0;
    let mut str_6: &str = "";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "o2r";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut bool_0: bool = crate::StrSimError::eq(strsimerror_1_ref_0, strsimerror_0_ref_0);
    let mut usize_0: usize = crate::osa_distance(str_7_ref_0, str_6_ref_0);
    let mut i8_0: &mut i8 = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<usize> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<usize> = crate::GrowingHashmapMapElemChar::default();
    let mut f64_0: f64 = crate::normalized_damerau_levenshtein(str_5_ref_0, str_4_ref_0);
    let mut f64_1: f64 = crate::normalized_damerau_levenshtein(str_3_ref_0, str_2_ref_0);
    let mut hybridgrowinghashmapchar_1: crate::HybridGrowingHashmapChar<usize> = crate::HybridGrowingHashmapChar::default();
    let mut usize_1: usize = crate::osa_distance(str_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6() {
    rusty_monitor::set_test_id(6);
    let mut str_0: &str = "Kiv95rccxozE2DvOq";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut char_0: char = '%';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<i64> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &mut crate::HybridGrowingHashmapChar<i64> = &mut hybridgrowinghashmapchar_0;
    let mut isize_0: isize = -11278isize;
    let mut rowid_0: crate::RowId = crate::RowId {val: isize_0};
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut isize_1: isize = 6isize;
    let mut rowid_1: crate::RowId = crate::RowId {val: isize_1};
    let mut rowid_1_ref_0: &crate::RowId = &mut rowid_1;
    let mut str_1: &str = "H4";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "e0KPS";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<u16> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<u16> = &mut growinghashmapmapelemchar_0;
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<u16> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    let mut growinghashmapmapelemchar_1_ref_0: &crate::GrowingHashmapMapElemChar<u16> = &mut growinghashmapmapelemchar_1;
    let mut growinghashmapmapelemchar_2: crate::GrowingHashmapMapElemChar<u16> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_1_ref_0);
    let mut usize_0: usize = crate::damerau_levenshtein(str_2_ref_0, str_1_ref_0);
    let mut bool_0: bool = crate::RowId::eq(rowid_1_ref_0, rowid_0_ref_0);
    let mut i64_0: &mut i64 = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut growinghashmapmapelemchar_2_ref_0: &crate::GrowingHashmapMapElemChar<u16> = &mut growinghashmapmapelemchar_2;
    let mut growinghashmapmapelemchar_3: crate::GrowingHashmapMapElemChar<u16> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_2_ref_0);
    let mut growinghashmapmapelemchar_3_ref_0: &crate::GrowingHashmapMapElemChar<u16> = &mut growinghashmapmapelemchar_3;
    let mut growinghashmapmapelemchar_4: crate::GrowingHashmapMapElemChar<u16> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_3_ref_0);
    crate::bigrams(str_0_ref_0);
    let mut growinghashmapmapelemchar_4_ref_0: &crate::GrowingHashmapMapElemChar<u16> = &mut growinghashmapmapelemchar_4;
    let mut growinghashmapmapelemchar_5: crate::GrowingHashmapMapElemChar<u16> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_4_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7() {
    rusty_monitor::set_test_id(7);
    let mut usize_0: usize = 2655usize;
    let mut usize_1: usize = 8549usize;
    let mut usize_2: usize = 8960usize;
    let mut isize_0: isize = -15488isize;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut str_0: &str = "nruoTrmoC";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Vp0E";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut char_0: char = '"';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<isize> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &mut crate::HybridGrowingHashmapChar<isize> = &mut hybridgrowinghashmapchar_0;
    let mut isize_1: isize = 12385isize;
    let mut isize_2: isize = 5113isize;
    let mut str_2: &str = "hgXtCDFiclW";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "3btjQgV5e";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut f64_0: f64 = crate::normalized_levenshtein(str_3_ref_0, str_2_ref_0);
    let mut rowid_1: crate::RowId = crate::RowId {val: isize_2};
    let mut rowid_2: crate::RowId = crate::RowId {val: isize_1};
    let mut isize_3: &mut isize = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut rowid_3: crate::RowId = crate::RowId::clone(rowid_0_ref_0);
    let mut rowid_3_ref_0: &crate::RowId = &mut rowid_3;
    let mut tuple_0: () = crate::RowId::assert_receiver_is_total_eq(rowid_3_ref_0);
    let mut rowid_4: crate::RowId = crate::RowId::default();
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<i64> = crate::GrowingHashmapMapElemChar::default();
    let mut f64_1: f64 = crate::jaro(str_1_ref_0, str_0_ref_0);
    let mut rowid_5: crate::RowId = crate::RowId {val: isize_0};
    let mut usize_3: usize = crate::flat_index(usize_2, usize_1, usize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8() {
    rusty_monitor::set_test_id(8);
    let mut u32_0: u32 = 7002u32;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut char_0: char = ',';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<i128> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &mut crate::HybridGrowingHashmapChar<i128> = &mut hybridgrowinghashmapchar_0;
    let mut u32_1: u32 = 8473u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<i64>>> = std::option::Option::None;
    let mut i32_0: i32 = -4014i32;
    let mut i32_1: i32 = -9400i32;
    let mut i32_2: i32 = -240i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<i64> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<i64> = &mut growinghashmapchar_0;
    let mut option_1: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<isize>>> = std::option::Option::None;
    let mut i32_3: i32 = -11829i32;
    let mut i32_4: i32 = -10301i32;
    let mut i32_5: i32 = 8807i32;
    let mut i128_0: i128 = 13120i128;
    let mut u32_2: u32 = 7793u32;
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<i128> = crate::GrowingHashmapMapElemChar {key: u32_2, value: i128_0};
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<i128> = &mut growinghashmapmapelemchar_0;
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<i128> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar {used: i32_5, fill: i32_4, mask: i32_3, map: option_1};
    let mut i64_0: &mut i64 = crate::GrowingHashmapChar::get_mut(growinghashmapchar_0_ref_0, u32_1);
    let mut growinghashmapmapelemchar_1_ref_0: &crate::GrowingHashmapMapElemChar<i128> = &mut growinghashmapmapelemchar_1;
    let mut growinghashmapmapelemchar_2: crate::GrowingHashmapMapElemChar<i128> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_1_ref_0);
    let mut i128_1: &mut i128 = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut tuple_0: () = crate::RowId::assert_receiver_is_total_eq(rowid_0_ref_0);
    let mut growinghashmapchar_1_ref_0: &mut crate::GrowingHashmapChar<isize> = &mut growinghashmapchar_1;
    let mut isize_0: &mut isize = crate::GrowingHashmapChar::get_mut(growinghashmapchar_1_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_9() {
    rusty_monitor::set_test_id(9);
    let mut isize_0: isize = -1710isize;
    let mut rowid_0: crate::RowId = crate::RowId {val: isize_0};
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut u64_0: u64 = 1175u64;
    let mut u32_0: u32 = 3574u32;
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<u64> = crate::GrowingHashmapMapElemChar {key: u32_0, value: u64_0};
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<u64> = &mut growinghashmapmapelemchar_0;
    let mut str_0: &str = "imEZzRvuxGVhvHQ7h5";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "nNK";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "QJGSN9cXdySBK0v7xy";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "ESCWi0zRzv24";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut i32_0: i32 = -9317i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<usize> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<usize> = &mut growinghashmapchar_0;
    let mut rowid_1: crate::RowId = crate::RowId::default();
    let mut rowid_1_ref_0: &crate::RowId = &mut rowid_1;
    let mut char_0: char = 'u';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<usize> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &mut crate::HybridGrowingHashmapChar<usize> = &mut hybridgrowinghashmapchar_0;
    let mut usize_0: &mut usize = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut rowid_2: crate::RowId = crate::RowId::clone(rowid_1_ref_0);
    crate::GrowingHashmapChar::grow(growinghashmapchar_0_ref_0, i32_0);
    let mut f64_0: f64 = crate::jaro_winkler(str_3_ref_0, str_2_ref_0);
    let mut f64_1: f64 = crate::jaro(str_1_ref_0, str_0_ref_0);
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<u64> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    let mut rowid_2_ref_0: &crate::RowId = &mut rowid_2;
    let mut bool_0: bool = crate::RowId::eq(rowid_2_ref_0, rowid_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_10() {
    rusty_monitor::set_test_id(10);
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut str_0: &str = "tVwdeXSD2EFx7MCr";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "ATIiQDn";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_0: isize = 11757isize;
    let mut u32_0: u32 = 2571u32;
    let mut u32_1: u32 = 1948u32;
    let mut str_2: &str = "RELdVg18EY";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "XE72iw3MqpR";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut char_0: char = 'w';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<i32> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &mut crate::HybridGrowingHashmapChar<i32> = &mut hybridgrowinghashmapchar_0;
    let mut i32_0: &mut i32 = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut f64_0: f64 = crate::sorensen_dice(str_3_ref_0, str_2_ref_0);
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u128> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_0_ref_0: &crate::GrowingHashmapChar<u128> = &mut growinghashmapchar_0;
    let mut usize_0: usize = crate::GrowingHashmapChar::lookup(growinghashmapchar_0_ref_0, u32_1);
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<isize> = crate::GrowingHashmapMapElemChar {key: u32_0, value: isize_0};
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<isize> = &mut growinghashmapmapelemchar_0;
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<isize> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    crate::hamming(str_1_ref_0, str_0_ref_0);
    let mut tuple_0: () = crate::RowId::assert_receiver_is_total_eq(rowid_0_ref_0);
    let mut growinghashmapmapelemchar_1_ref_0: &crate::GrowingHashmapMapElemChar<isize> = &mut growinghashmapmapelemchar_1;
    let mut growinghashmapmapelemchar_2: crate::GrowingHashmapMapElemChar<isize> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_1_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_11() {
    rusty_monitor::set_test_id(11);
    let mut str_0: &str = "O8Lt0";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut str_1: &str = "58vJsZVVnRfBS";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "sKxSN";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_0: usize = 1311usize;
    let mut usize_1: usize = 7559usize;
    let mut usize_2: usize = 3738usize;
    let mut strsimerror_2: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_2_ref_0: &StrSimError = &mut strsimerror_2;
    let mut str_3: &str = "IVmEqIPuBuO6o";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut isize_0: isize = 15864isize;
    let mut rowid_0: crate::RowId = crate::RowId {val: isize_0};
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut tuple_0: () = crate::RowId::assert_receiver_is_total_eq(rowid_0_ref_0);
    let mut strsimerror_3: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut f64_0: f64 = crate::jaro(str_4_ref_0, str_3_ref_0);
    let mut strsimerror_3_ref_0: &StrSimError = &mut strsimerror_3;
    let mut bool_0: bool = crate::StrSimError::eq(strsimerror_3_ref_0, strsimerror_2_ref_0);
    let mut usize_3: usize = crate::flat_index(usize_2, usize_1, usize_0);
    let mut usize_4: usize = crate::damerau_levenshtein(str_2_ref_0, str_1_ref_0);
    let mut bool_1: bool = crate::StrSimError::eq(strsimerror_1_ref_0, strsimerror_0_ref_0);
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<u128> = crate::GrowingHashmapMapElemChar::default();
    crate::bigrams(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_12() {
    rusty_monitor::set_test_id(12);
    let mut str_0: &str = "vFFKH6W";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Yg";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u32_0: u32 = 942u32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u16> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_0_ref_0: &crate::GrowingHashmapChar<u16> = &mut growinghashmapchar_0;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<f32> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<f32> = &mut growinghashmapmapelemchar_0;
    let mut str_2: &str = "mgZt2WEbhboJQD";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "lU4MXuV4";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "ZRYutlf27";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "go3nwAHYXTN";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut f64_0: f64 = crate::jaro(str_5_ref_0, str_4_ref_0);
    let mut usize_0: usize = crate::osa_distance(str_3_ref_0, str_2_ref_0);
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<f32> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    let mut growinghashmapmapelemchar_1_ref_0: &crate::GrowingHashmapMapElemChar<f32> = &mut growinghashmapmapelemchar_1;
    let mut growinghashmapmapelemchar_2: crate::GrowingHashmapMapElemChar<f32> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_1_ref_0);
    let mut growinghashmapmapelemchar_2_ref_0: &crate::GrowingHashmapMapElemChar<f32> = &mut growinghashmapmapelemchar_2;
    let mut growinghashmapmapelemchar_3: crate::GrowingHashmapMapElemChar<f32> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_2_ref_0);
    let mut u16_0: u16 = crate::GrowingHashmapChar::get(growinghashmapchar_0_ref_0, u32_0);
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<bool> = crate::HybridGrowingHashmapChar::default();
    let mut growinghashmapmapelemchar_4: crate::GrowingHashmapMapElemChar<u8> = crate::GrowingHashmapMapElemChar::default();
    let mut f64_1: f64 = crate::sorensen_dice(str_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_13() {
    rusty_monitor::set_test_id(13);
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut u32_0: u32 = 5124u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<u32>>> = std::option::Option::None;
    let mut i32_0: i32 = -6928i32;
    let mut i32_1: i32 = -6661i32;
    let mut i32_2: i32 = -4927i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u32> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &crate::GrowingHashmapChar<u32> = &mut growinghashmapchar_0;
    let mut option_1: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<isize>>> = std::option::Option::None;
    let mut i32_3: i32 = 16018i32;
    let mut i32_4: i32 = -790i32;
    let mut i32_5: i32 = 4750i32;
    let mut char_0: char = 'w';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<i64> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &mut crate::HybridGrowingHashmapChar<i64> = &mut hybridgrowinghashmapchar_0;
    let mut usize_0: usize = 8889usize;
    let mut usize_1: usize = 2174usize;
    let mut usize_2: usize = 998usize;
    let mut str_0: &str = "1fSqJg3CR";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "5SH4tsQ3Y44EU";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut hybridgrowinghashmapchar_1: crate::HybridGrowingHashmapChar<i64> = crate::HybridGrowingHashmapChar::default();
    let mut usize_3: usize = crate::osa_distance(str_1_ref_0, str_0_ref_0);
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<i128> = crate::GrowingHashmapChar::default();
    let mut usize_4: usize = crate::flat_index(usize_2, usize_1, usize_0);
    let mut i64_0: &mut i64 = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut growinghashmapchar_2: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar {used: i32_5, fill: i32_4, mask: i32_3, map: option_1};
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut u32_1: u32 = crate::GrowingHashmapChar::get(growinghashmapchar_0_ref_0, u32_0);
    let mut bool_0: bool = crate::StrSimError::eq(strsimerror_1_ref_0, strsimerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_14() {
    rusty_monitor::set_test_id(14);
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<i64>>> = std::option::Option::None;
    let mut i32_0: i32 = -2598i32;
    let mut i32_1: i32 = -2311i32;
    let mut i32_2: i32 = -11897i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<i64> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<i64> = &mut growinghashmapchar_0;
    let mut str_0: &str = "iV0";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "iEK5oxoYyUqr3";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "RBa2r2iC4VcRktC";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Zou19neen";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u32_0: u32 = 4466u32;
    let mut option_1: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<u16>>> = std::option::Option::None;
    let mut i32_3: i32 = 14854i32;
    let mut i32_4: i32 = 2525i32;
    let mut i32_5: i32 = 10704i32;
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<u16> = crate::GrowingHashmapChar {used: i32_5, fill: i32_4, mask: i32_3, map: option_1};
    let mut growinghashmapchar_1_ref_0: &crate::GrowingHashmapChar<u16> = &mut growinghashmapchar_1;
    let mut str_4: &str = "XtpJOy";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "c1u4eKt";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "XnOL4GcD4m6ffIPV";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "gQTyhJbDpanJ9WlaabW";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut usize_0: usize = crate::damerau_levenshtein(str_7_ref_0, str_6_ref_0);
    crate::hamming(str_5_ref_0, str_4_ref_0);
    let mut usize_1: usize = crate::GrowingHashmapChar::lookup(growinghashmapchar_1_ref_0, u32_0);
    let mut usize_2: usize = crate::levenshtein(str_3_ref_0, str_2_ref_0);
    let mut usize_3: usize = crate::osa_distance(str_1_ref_0, str_0_ref_0);
    crate::GrowingHashmapChar::allocate(growinghashmapchar_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_15() {
    rusty_monitor::set_test_id(15);
    let mut u32_0: u32 = 3567u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<u128>>> = std::option::Option::None;
    let mut i32_0: i32 = -1007i32;
    let mut i32_1: i32 = -4234i32;
    let mut i32_2: i32 = 4571i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u128> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<u128> = &mut growinghashmapchar_0;
    let mut usize_0: usize = 9293usize;
    let mut usize_1: usize = 3564usize;
    let mut usize_2: usize = 5556usize;
    let mut str_0: &str = "7JMXYfPYxYS";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "bC5k3QjLWFh58";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut u32_1: u32 = 9883u32;
    let mut option_1: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<i32>>> = std::option::Option::None;
    let mut i32_3: i32 = 6195i32;
    let mut i32_4: i32 = 25135i32;
    let mut i32_5: i32 = 11689i32;
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<i32> = crate::GrowingHashmapChar {used: i32_5, fill: i32_4, mask: i32_3, map: option_1};
    let mut growinghashmapchar_1_ref_0: &mut crate::GrowingHashmapChar<i32> = &mut growinghashmapchar_1;
    let mut i32_6: i32 = 26648i32;
    let mut growinghashmapchar_2: crate::GrowingHashmapChar<u16> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_2_ref_0: &mut crate::GrowingHashmapChar<u16> = &mut growinghashmapchar_2;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    crate::GrowingHashmapChar::grow(growinghashmapchar_2_ref_0, i32_6);
    let mut i32_7: &mut i32 = crate::GrowingHashmapChar::get_mut(growinghashmapchar_1_ref_0, u32_1);
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut bool_0: bool = crate::StrSimError::eq(strsimerror_1_ref_0, strsimerror_0_ref_0);
    let mut usize_3: usize = crate::levenshtein(str_1_ref_0, str_0_ref_0);
    let mut usize_4: usize = crate::flat_index(usize_2, usize_1, usize_0);
    let mut u128_0: &mut u128 = crate::GrowingHashmapChar::get_mut(growinghashmapchar_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_16() {
    rusty_monitor::set_test_id(16);
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<u64> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<u64> = &mut growinghashmapmapelemchar_0;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut rowid_1: crate::RowId = crate::RowId::default();
    let mut rowid_1_ref_0: &crate::RowId = &mut rowid_1;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut usize_0: usize = 5654usize;
    let mut usize_1: usize = 9982usize;
    let mut usize_2: usize = 5294usize;
    let mut strsimerror_2: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_2_ref_0: &StrSimError = &mut strsimerror_2;
    let mut str_0: &str = "RIKiJoM1X";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "m";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u32_0: u32 = 8466u32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_0_ref_0: &crate::GrowingHashmapChar<isize> = &mut growinghashmapchar_0;
    let mut isize_0: isize = crate::GrowingHashmapChar::get(growinghashmapchar_0_ref_0, u32_0);
    let mut usize_3: usize = crate::damerau_levenshtein(str_1_ref_0, str_0_ref_0);
    let mut usize_4: usize = crate::flat_index(usize_2, usize_1, usize_0);
    let mut bool_0: bool = crate::RowId::eq(rowid_1_ref_0, rowid_0_ref_0);
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<u64> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    let mut rowid_2: crate::RowId = crate::RowId::default();
    let mut rowid_2_ref_0: &crate::RowId = &mut rowid_2;
    let mut tuple_0: () = crate::RowId::assert_receiver_is_total_eq(rowid_2_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_17() {
    rusty_monitor::set_test_id(17);
    let mut str_0: &str = "dsmevkriyvL1x";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "oM6eMdIty1yYkrRFJGU";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u32_0: u32 = 1585u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<i16>>> = std::option::Option::None;
    let mut i32_0: i32 = -20769i32;
    let mut i32_1: i32 = -8289i32;
    let mut i32_2: i32 = 7731i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<i16> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<i16> = &mut growinghashmapchar_0;
    let mut str_2: &str = "JKo9POlWSmcvCwirSk";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "h4";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "UoHRTmFyaobLqko4u";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "RUhyNB2s";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut isize_0: isize = -9497isize;
    let mut char_0: char = 'T';
    let mut str_6: &str = "jtYnN5KVx";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "KsVWRIjG";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<bool> = crate::HybridGrowingHashmapChar::default();
    let mut usize_0: usize = crate::damerau_levenshtein(str_7_ref_0, str_6_ref_0);
    let mut hybridgrowinghashmapchar_0_ref_0: &mut crate::HybridGrowingHashmapChar<bool> = &mut hybridgrowinghashmapchar_0;
    let mut bool_0: &mut bool = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut rowid_0: crate::RowId = crate::RowId {val: isize_0};
    let mut usize_1: usize = crate::levenshtein(str_5_ref_0, str_4_ref_0);
    let mut f64_0: f64 = crate::sorensen_dice(str_3_ref_0, str_2_ref_0);
    let mut i16_0: &mut i16 = crate::GrowingHashmapChar::get_mut(growinghashmapchar_0_ref_0, u32_0);
    let mut f64_1: f64 = crate::normalized_levenshtein(str_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_18() {
    rusty_monitor::set_test_id(18);
    let mut u32_0: u32 = 2019u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<usize>>> = std::option::Option::None;
    let mut i32_0: i32 = 4289i32;
    let mut i32_1: i32 = 8122i32;
    let mut i32_2: i32 = 10824i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<usize> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<usize> = &mut growinghashmapchar_0;
    let mut char_0: char = 'Z';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<usize> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &mut crate::HybridGrowingHashmapChar<usize> = &mut hybridgrowinghashmapchar_0;
    let mut char_1: char = 'q';
    let mut hybridgrowinghashmapchar_1: crate::HybridGrowingHashmapChar<isize> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_1_ref_0: &mut crate::HybridGrowingHashmapChar<isize> = &mut hybridgrowinghashmapchar_1;
    let mut str_0: &str = "Q588m2gd4u";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Mr6tbe9niTol2ju7YAh";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut strsimerror_2: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_2_ref_0: &StrSimError = &mut strsimerror_2;
    let mut strsimerror_3: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_3_ref_0: &StrSimError = &mut strsimerror_3;
    let mut strsimerror_4: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_4_ref_0: &StrSimError = &mut strsimerror_4;
    let mut u32_1: u32 = 15u32;
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<i32> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_1_ref_0: &crate::GrowingHashmapChar<i32> = &mut growinghashmapchar_1;
    let mut usize_0: usize = crate::GrowingHashmapChar::lookup(growinghashmapchar_1_ref_0, u32_1);
    let mut bool_0: bool = crate::StrSimError::eq(strsimerror_3_ref_0, strsimerror_2_ref_0);
    let mut bool_1: bool = crate::StrSimError::eq(strsimerror_1_ref_0, strsimerror_0_ref_0);
    let mut f64_0: f64 = crate::normalized_levenshtein(str_1_ref_0, str_0_ref_0);
    let mut isize_0: &mut isize = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_1_ref_0, char_1);
    let mut usize_1: &mut usize = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut usize_2: &mut usize = crate::GrowingHashmapChar::get_mut(growinghashmapchar_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_19() {
    rusty_monitor::set_test_id(19);
    let mut char_0: char = 'O';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<isize> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &mut crate::HybridGrowingHashmapChar<isize> = &mut hybridgrowinghashmapchar_0;
    let mut u32_0: u32 = 405u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<u8>>> = std::option::Option::None;
    let mut i32_0: i32 = -11052i32;
    let mut i32_1: i32 = 860i32;
    let mut i32_2: i32 = -5731i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u8> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &crate::GrowingHashmapChar<u8> = &mut growinghashmapchar_0;
    let mut str_0: &str = "UW79Nf5ozHhnzf5sZNO";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Dk4Wuj8CZSMXZFnV";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut char_1: char = ':';
    let mut hybridgrowinghashmapchar_1: crate::HybridGrowingHashmapChar<u16> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_1_ref_0: &mut crate::HybridGrowingHashmapChar<u16> = &mut hybridgrowinghashmapchar_1;
    let mut usize_0: usize = 9585usize;
    let mut usize_1: usize = 7676usize;
    let mut usize_2: usize = 7674usize;
    let mut str_2: &str = "gqAbS3K3bYTiOXzWqQ";
    let mut str_2_ref_0: &str = &mut str_2;
    crate::bigrams(str_2_ref_0);
    let mut usize_3: usize = crate::flat_index(usize_2, usize_1, usize_0);
    let mut u16_0: &mut u16 = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_1_ref_0, char_1);
    let mut usize_4: usize = crate::osa_distance(str_1_ref_0, str_0_ref_0);
    let mut u8_0: u8 = crate::GrowingHashmapChar::get(growinghashmapchar_0_ref_0, u32_0);
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut rowid_1: crate::RowId = crate::RowId::clone(rowid_0_ref_0);
    let mut isize_0: &mut isize = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_0_ref_0, char_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_20() {
    rusty_monitor::set_test_id(20);
    let mut str_0: &str = "N9nKSzVvNVGwBx";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "dH2tV";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u16> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<u16> = &mut growinghashmapchar_0;
    let mut str_2: &str = "vDWIC6bSh3c29eyMqP";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Y4iDzUICju2Wv";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut char_0: char = '#';
    let mut str_4: &str = "vzgOGRi";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "53pF9Uwn8jF93Btzhos";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut i32_0: i32 = -3624i32;
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<i64> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_1_ref_0: &mut crate::GrowingHashmapChar<i64> = &mut growinghashmapchar_1;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<usize> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<usize> = &mut growinghashmapmapelemchar_0;
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<usize> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    let mut tuple_0: () = crate::RowId::assert_receiver_is_total_eq(rowid_0_ref_0);
    crate::GrowingHashmapChar::grow(growinghashmapchar_1_ref_0, i32_0);
    let mut usize_0: usize = crate::damerau_levenshtein(str_5_ref_0, str_4_ref_0);
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<i64> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &crate::HybridGrowingHashmapChar<i64> = &mut hybridgrowinghashmapchar_0;
    let mut i64_0: i64 = crate::HybridGrowingHashmapChar::get(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut usize_1: usize = crate::levenshtein(str_3_ref_0, str_2_ref_0);
    let mut growinghashmapmapelemchar_1_ref_0: &crate::GrowingHashmapMapElemChar<usize> = &mut growinghashmapmapelemchar_1;
    let mut growinghashmapmapelemchar_2: crate::GrowingHashmapMapElemChar<usize> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_1_ref_0);
    crate::GrowingHashmapChar::allocate(growinghashmapchar_0_ref_0);
    let mut f64_0: f64 = crate::jaro(str_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_21() {
    rusty_monitor::set_test_id(21);
    let mut u32_0: u32 = 937u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<u64>>> = std::option::Option::None;
    let mut i32_0: i32 = -8102i32;
    let mut i32_1: i32 = 6379i32;
    let mut i32_2: i32 = 2732i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u64> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &crate::GrowingHashmapChar<u64> = &mut growinghashmapchar_0;
    let mut str_0: &str = "7bdvEv3fd";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "W9JQkau5";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<u128> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<u128> = &mut growinghashmapmapelemchar_0;
    let mut str_2: &str = "KOUuZvemYFH";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "U";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_0: usize = crate::osa_distance(str_3_ref_0, str_2_ref_0);
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<u128> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    let mut growinghashmapmapelemchar_1_ref_0: &crate::GrowingHashmapMapElemChar<u128> = &mut growinghashmapmapelemchar_1;
    let mut growinghashmapmapelemchar_2: crate::GrowingHashmapMapElemChar<u128> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_1_ref_0);
    let mut usize_1: usize = crate::levenshtein(str_1_ref_0, str_0_ref_0);
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<bool> = crate::GrowingHashmapChar::default();
    let mut growinghashmapmapelemchar_2_ref_0: &crate::GrowingHashmapMapElemChar<u128> = &mut growinghashmapmapelemchar_2;
    let mut growinghashmapmapelemchar_3: crate::GrowingHashmapMapElemChar<u128> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_2_ref_0);
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut usize_2: usize = crate::GrowingHashmapChar::lookup(growinghashmapchar_0_ref_0, u32_0);
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut tuple_0: () = crate::RowId::assert_receiver_is_total_eq(rowid_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_22() {
    rusty_monitor::set_test_id(22);
    let mut u32_0: u32 = 8298u32;
    let mut str_0: &str = "o4vGJM37GVyuezSGlE";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Aipq";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut str_2: &str = "iljM2QKgZca8RBGY";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "sXYVSL5";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut isize_0: isize = -11894isize;
    let mut str_5: &str = "NMoo";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "H3V";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut f64_0: f64 = crate::jaro(str_6_ref_0, str_5_ref_0);
    let mut rowid_1: crate::RowId = crate::RowId {val: isize_0};
    let mut rowid_1_ref_0: &crate::RowId = &mut rowid_1;
    let mut bool_0: bool = crate::RowId::eq(rowid_1_ref_0, rowid_0_ref_0);
    crate::bigrams(str_4_ref_0);
    crate::hamming(str_3_ref_0, str_2_ref_0);
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<i32> = crate::GrowingHashmapChar::default();
    let mut usize_0: usize = crate::osa_distance(str_1_ref_0, str_0_ref_0);
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<i32> = &mut growinghashmapchar_0;
    let mut i32_0: &mut i32 = crate::GrowingHashmapChar::get_mut(growinghashmapchar_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_23() {
    rusty_monitor::set_test_id(23);
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut u32_0: u32 = 4937u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<u16>>> = std::option::Option::None;
    let mut i32_0: i32 = -5433i32;
    let mut i32_1: i32 = 2058i32;
    let mut i32_2: i32 = 6082i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u16> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &crate::GrowingHashmapChar<u16> = &mut growinghashmapchar_0;
    let mut isize_0: isize = 263isize;
    let mut u32_1: u32 = 9229u32;
    let mut str_0: &str = "p6oxuZXQgVFfYwjgh4q";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "FPELHPjm";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Ua";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_0: usize = 3470usize;
    let mut usize_1: usize = 1747usize;
    let mut usize_2: usize = 1788usize;
    let mut usize_3: usize = crate::flat_index(usize_2, usize_1, usize_0);
    let mut usize_4: usize = crate::levenshtein(str_3_ref_0, str_2_ref_0);
    let mut usize_5: usize = crate::osa_distance(str_1_ref_0, str_0_ref_0);
    let mut strsimerror_2: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_2_ref_0: &StrSimError = &mut strsimerror_2;
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<isize> = crate::GrowingHashmapMapElemChar {key: u32_1, value: isize_0};
    let mut usize_6: usize = crate::GrowingHashmapChar::lookup(growinghashmapchar_0_ref_0, u32_0);
    let mut bool_0: bool = crate::StrSimError::eq(strsimerror_1_ref_0, strsimerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_24() {
    rusty_monitor::set_test_id(24);
    let mut u32_0: u32 = 2202u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<i64>>> = std::option::Option::None;
    let mut i32_0: i32 = 3827i32;
    let mut i32_1: i32 = 5064i32;
    let mut i32_2: i32 = 2199i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<i64> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<i64> = &mut growinghashmapchar_0;
    let mut isize_0: isize = 13998isize;
    let mut u32_1: u32 = 4546u32;
    let mut str_0: &str = "ccO7qoJ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "11Jz5rXmxgvH";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "gmOPlsN";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "K4nDIvq";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut str_4: &str = "SE";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "hz9GMbnwKi9";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "mSAzZdgBoqYxneM";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "n7LR53Sf3";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut f64_0: f64 = crate::sorensen_dice(str_7_ref_0, str_6_ref_0);
    let mut usize_0: usize = crate::osa_distance(str_5_ref_0, str_4_ref_0);
    let mut usize_1: usize = crate::levenshtein(str_3_ref_0, str_2_ref_0);
    crate::hamming(str_1_ref_0, str_0_ref_0);
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<isize> = crate::GrowingHashmapMapElemChar {key: u32_1, value: isize_0};
    let mut i64_0: &mut i64 = crate::GrowingHashmapChar::get_mut(growinghashmapchar_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_25() {
    rusty_monitor::set_test_id(25);
    let mut char_0: char = 'f';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<u64> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &crate::HybridGrowingHashmapChar<u64> = &mut hybridgrowinghashmapchar_0;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut isize_0: isize = 9109isize;
    let mut rowid_0: crate::RowId = crate::RowId {val: isize_0};
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut str_0: &str = "Xau4rEDPCFxot";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "FauP4Uj3Dt";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "EklwejT";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "tMJgUN";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "5L1HqlBMuGj";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "folU22jezC4yxGw";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut strsimerror_2: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_2_ref_0: &StrSimError = &mut strsimerror_2;
    let mut rowid_1: crate::RowId = crate::RowId::default();
    let mut bool_0: bool = crate::StrSimError::eq(strsimerror_2_ref_0, strsimerror_1_ref_0);
    let mut rowid_1_ref_0: &crate::RowId = &mut rowid_1;
    let mut rowid_2: crate::RowId = crate::RowId::clone(rowid_1_ref_0);
    crate::hamming(str_5_ref_0, str_4_ref_0);
    let mut f64_0: f64 = crate::jaro(str_3_ref_0, str_2_ref_0);
    let mut f64_1: f64 = crate::normalized_damerau_levenshtein(str_1_ref_0, str_0_ref_0);
    let mut tuple_0: () = crate::RowId::assert_receiver_is_total_eq(rowid_0_ref_0);
    let mut u64_0: u64 = crate::HybridGrowingHashmapChar::get(hybridgrowinghashmapchar_0_ref_0, char_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_26() {
    rusty_monitor::set_test_id(26);
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut isize_0: isize = 3501isize;
    let mut rowid_1: crate::RowId = crate::RowId {val: isize_0};
    let mut rowid_1_ref_0: &crate::RowId = &mut rowid_1;
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<f32> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<f32> = &mut growinghashmapmapelemchar_0;
    let mut isize_1: isize = -16777isize;
    let mut u32_0: u32 = 3967u32;
    let mut i32_0: i32 = 7007i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u16> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<u16> = &mut growinghashmapchar_0;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    crate::GrowingHashmapChar::grow(growinghashmapchar_0_ref_0, i32_0);
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<isize> = crate::GrowingHashmapMapElemChar {key: u32_0, value: isize_1};
    let mut rowid_2: crate::RowId = crate::RowId::default();
    let mut growinghashmapmapelemchar_2: crate::GrowingHashmapMapElemChar<f32> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    let mut bool_0: bool = crate::RowId::eq(rowid_1_ref_0, rowid_0_ref_0);
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<u32> = crate::HybridGrowingHashmapChar::default();
    let mut growinghashmapmapelemchar_3: crate::GrowingHashmapMapElemChar<u32> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapmapelemchar_4: crate::GrowingHashmapMapElemChar<usize> = crate::GrowingHashmapMapElemChar::default();
    let mut rowid_2_ref_0: &crate::RowId = &mut rowid_2;
    let mut tuple_0: () = crate::RowId::assert_receiver_is_total_eq(rowid_2_ref_0);
    let mut growinghashmapmapelemchar_5: crate::GrowingHashmapMapElemChar<f64> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapmapelemchar_1_ref_0: &crate::GrowingHashmapMapElemChar<isize> = &mut growinghashmapmapelemchar_1;
    let mut growinghashmapmapelemchar_6: crate::GrowingHashmapMapElemChar<isize> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_1_ref_0);
    let mut rowid_3: crate::RowId = crate::RowId::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_27() {
    rusty_monitor::set_test_id(27);
    let mut char_0: char = 'u';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<i64> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &mut crate::HybridGrowingHashmapChar<i64> = &mut hybridgrowinghashmapchar_0;
    let mut u32_0: u32 = 4211u32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<i128> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_0_ref_0: &crate::GrowingHashmapChar<i128> = &mut growinghashmapchar_0;
    let mut usize_0: usize = 7118usize;
    let mut usize_1: usize = 6617usize;
    let mut usize_2: usize = 9802usize;
    let mut str_0: &str = "3BJ99IgdZ3UdOF";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Dv2WEGb9S4IzYdhA";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut char_1: char = ')';
    let mut hybridgrowinghashmapchar_1: crate::HybridGrowingHashmapChar<u64> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_1_ref_0: &crate::HybridGrowingHashmapChar<u64> = &mut hybridgrowinghashmapchar_1;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "IMWjrulPgLic4ufN";
    let mut str_3_ref_0: &str = &mut str_3;
    crate::hamming(str_3_ref_0, str_2_ref_0);
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<u16> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<u16> = &mut growinghashmapmapelemchar_0;
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<u16> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    let mut u64_0: u64 = crate::HybridGrowingHashmapChar::get(hybridgrowinghashmapchar_1_ref_0, char_1);
    let mut usize_3: usize = crate::levenshtein(str_1_ref_0, str_0_ref_0);
    let mut growinghashmapmapelemchar_1_ref_0: &crate::GrowingHashmapMapElemChar<u16> = &mut growinghashmapmapelemchar_1;
    let mut growinghashmapmapelemchar_2: crate::GrowingHashmapMapElemChar<u16> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_1_ref_0);
    let mut usize_4: usize = crate::flat_index(usize_2, usize_1, usize_0);
    let mut i128_0: i128 = crate::GrowingHashmapChar::get(growinghashmapchar_0_ref_0, u32_0);
    let mut i64_0: &mut i64 = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_0_ref_0, char_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_28() {
    rusty_monitor::set_test_id(28);
    let mut str_0: &str = "esPJ2St94VO";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "GRaCQhfVfUfW3fgS8";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u32_0: u32 = 1580u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<u64>>> = std::option::Option::None;
    let mut i32_0: i32 = 7044i32;
    let mut i32_1: i32 = -8986i32;
    let mut i32_2: i32 = 3687i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u64> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &crate::GrowingHashmapChar<u64> = &mut growinghashmapchar_0;
    let mut str_2: &str = "kuKCm86ERX";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "TgzOWhJ52NJa3lHzyL";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut rowid_1: crate::RowId = crate::RowId::default();
    let mut rowid_1_ref_0: &crate::RowId = &mut rowid_1;
    let mut str_4: &str = "Kd92B";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "YLZ";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut f64_0: f64 = crate::normalized_levenshtein(str_5_ref_0, str_4_ref_0);
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut bool_0: bool = crate::RowId::eq(rowid_1_ref_0, rowid_0_ref_0);
    let mut usize_0: usize = crate::levenshtein(str_3_ref_0, str_2_ref_0);
    let mut usize_1: usize = crate::GrowingHashmapChar::lookup(growinghashmapchar_0_ref_0, u32_0);
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut f64_1: f64 = crate::jaro(str_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_29() {
    rusty_monitor::set_test_id(29);
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<i64> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<i64> = &mut growinghashmapmapelemchar_0;
    let mut u32_0: u32 = 6585u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<u16>>> = std::option::Option::None;
    let mut i32_0: i32 = 5799i32;
    let mut i32_1: i32 = 2198i32;
    let mut i32_2: i32 = 6150i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u16> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &crate::GrowingHashmapChar<u16> = &mut growinghashmapchar_0;
    let mut str_0: &str = "XIGmTvI8WYw";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "evYIE1JhxcJ6w0CY";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u32_1: u32 = 2730u32;
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_1_ref_0: &crate::GrowingHashmapChar<isize> = &mut growinghashmapchar_1;
    let mut u32_2: u32 = 464u32;
    let mut growinghashmapchar_2: crate::GrowingHashmapChar<u128> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_2_ref_0: &crate::GrowingHashmapChar<u128> = &mut growinghashmapchar_2;
    let mut str_2: &str = "lAAOqXjnOTBRaQcrnUk";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "eJZMxZe8HBxP";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut f64_0: f64 = crate::normalized_damerau_levenshtein(str_3_ref_0, str_2_ref_0);
    let mut usize_0: usize = crate::GrowingHashmapChar::lookup(growinghashmapchar_2_ref_0, u32_2);
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut tuple_0: () = crate::RowId::assert_receiver_is_total_eq(rowid_0_ref_0);
    let mut isize_0: isize = crate::GrowingHashmapChar::get(growinghashmapchar_1_ref_0, u32_1);
    let mut f64_1: f64 = crate::normalized_damerau_levenshtein(str_1_ref_0, str_0_ref_0);
    let mut usize_1: usize = crate::GrowingHashmapChar::lookup(growinghashmapchar_0_ref_0, u32_0);
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<i64> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_30() {
    rusty_monitor::set_test_id(30);
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<u64>>> = std::option::Option::None;
    let mut i32_0: i32 = 7088i32;
    let mut i32_1: i32 = 489i32;
    let mut i32_2: i32 = 7966i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u64> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<u64> = &mut growinghashmapchar_0;
    let mut str_0: &str = "lLmxQ1Yi7Ahl";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "xHCSGt";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut char_0: char = 'K';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<u8> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &crate::HybridGrowingHashmapChar<u8> = &mut hybridgrowinghashmapchar_0;
    let mut u32_0: u32 = 3115u32;
    let mut option_1: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<bool>>> = std::option::Option::None;
    let mut i32_3: i32 = -29467i32;
    let mut i32_4: i32 = -1056i32;
    let mut i32_5: i32 = -20695i32;
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<bool> = crate::GrowingHashmapChar {used: i32_5, fill: i32_4, mask: i32_3, map: option_1};
    let mut growinghashmapchar_1_ref_0: &crate::GrowingHashmapChar<bool> = &mut growinghashmapchar_1;
    let mut isize_0: isize = -17487isize;
    let mut str_2: &str = "OeZ";
    let mut str_2_ref_0: &str = &mut str_2;
    crate::bigrams(str_2_ref_0);
    let mut rowid_0: crate::RowId = crate::RowId {val: isize_0};
    let mut usize_0: usize = crate::GrowingHashmapChar::lookup(growinghashmapchar_1_ref_0, u32_0);
    let mut u8_0: u8 = crate::HybridGrowingHashmapChar::get(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut usize_1: usize = crate::levenshtein(str_1_ref_0, str_0_ref_0);
    crate::GrowingHashmapChar::allocate(growinghashmapchar_0_ref_0);
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut rowid_1: crate::RowId = crate::RowId::clone(rowid_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_31() {
    rusty_monitor::set_test_id(31);
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<usize> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<usize> = &mut growinghashmapmapelemchar_0;
    let mut u32_0: u32 = 5042u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<isize>>> = std::option::Option::None;
    let mut i32_0: i32 = -7572i32;
    let mut i32_1: i32 = 6674i32;
    let mut i32_2: i32 = 8519i32;
    let mut char_0: char = 'H';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<u16> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &crate::HybridGrowingHashmapChar<u16> = &mut hybridgrowinghashmapchar_0;
    let mut str_0: &str = "gHdxGNjVA";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "6b1hN7QjlTfJoInKt";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<f64> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapmapelemchar_1_ref_0: &crate::GrowingHashmapMapElemChar<f64> = &mut growinghashmapmapelemchar_1;
    let mut growinghashmapmapelemchar_2: crate::GrowingHashmapMapElemChar<f64> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_1_ref_0);
    let mut growinghashmapmapelemchar_2_ref_0: &crate::GrowingHashmapMapElemChar<f64> = &mut growinghashmapmapelemchar_2;
    let mut growinghashmapmapelemchar_3: crate::GrowingHashmapMapElemChar<f64> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_2_ref_0);
    let mut f64_0: f64 = crate::sorensen_dice(str_1_ref_0, str_0_ref_0);
    let mut u16_0: u16 = crate::HybridGrowingHashmapChar::get(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut growinghashmapmapelemchar_3_ref_0: &crate::GrowingHashmapMapElemChar<f64> = &mut growinghashmapmapelemchar_3;
    let mut growinghashmapmapelemchar_4: crate::GrowingHashmapMapElemChar<f64> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_3_ref_0);
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut growinghashmapchar_0_ref_0: &crate::GrowingHashmapChar<isize> = &mut growinghashmapchar_0;
    let mut usize_0: usize = crate::GrowingHashmapChar::lookup(growinghashmapchar_0_ref_0, u32_0);
    let mut growinghashmapmapelemchar_4_ref_0: &crate::GrowingHashmapMapElemChar<f64> = &mut growinghashmapmapelemchar_4;
    let mut growinghashmapmapelemchar_5: crate::GrowingHashmapMapElemChar<f64> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_4_ref_0);
    let mut growinghashmapmapelemchar_6: crate::GrowingHashmapMapElemChar<usize> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_32() {
    rusty_monitor::set_test_id(32);
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<isize>>> = std::option::Option::None;
    let mut i32_0: i32 = -390i32;
    let mut i32_1: i32 = 407i32;
    let mut i32_2: i32 = 7217i32;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut char_0: char = 'i';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<u64> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &crate::HybridGrowingHashmapChar<u64> = &mut hybridgrowinghashmapchar_0;
    let mut str_0: &str = "Aw55LD2DhR";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "HBYQOZmyD1mbQj";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut isize_0: isize = 903isize;
    let mut rowid_1: crate::RowId = crate::RowId {val: isize_0};
    let mut rowid_1_ref_0: &crate::RowId = &mut rowid_1;
    let mut str_2: &str = "8Dk7iyMtSUxfX";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Nnrq";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut char_1: char = 'e';
    let mut hybridgrowinghashmapchar_1: crate::HybridGrowingHashmapChar<bool> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_1_ref_0: &crate::HybridGrowingHashmapChar<bool> = &mut hybridgrowinghashmapchar_1;
    let mut bool_0: bool = crate::HybridGrowingHashmapChar::get(hybridgrowinghashmapchar_1_ref_0, char_1);
    let mut f64_0: f64 = crate::normalized_damerau_levenshtein(str_3_ref_0, str_2_ref_0);
    let mut bool_1: bool = crate::RowId::eq(rowid_1_ref_0, rowid_0_ref_0);
    let mut f64_1: f64 = crate::normalized_damerau_levenshtein(str_1_ref_0, str_0_ref_0);
    let mut u64_0: u64 = crate::HybridGrowingHashmapChar::get(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut bool_2: bool = crate::StrSimError::eq(strsimerror_1_ref_0, strsimerror_0_ref_0);
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_33() {
    rusty_monitor::set_test_id(33);
    let mut str_0: &str = "8HJj1r57";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "kVU";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "cpo";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "2wG2";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut isize_0: isize = 9369isize;
    let mut isize_1: isize = -12443isize;
    let mut rowid_0: crate::RowId = crate::RowId {val: isize_1};
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut str_4: &str = "YQqfaZK7lULoBG";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "9J";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "2nqBJfX4xp8gBFm4";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut bool_0: bool = crate::StrSimError::eq(strsimerror_1_ref_0, strsimerror_0_ref_0);
    let mut strsimerror_2: StrSimError = crate::StrSimError::DifferentLengthArgs;
    crate::bigrams(str_6_ref_0);
    let mut f64_0: f64 = crate::normalized_damerau_levenshtein(str_5_ref_0, str_4_ref_0);
    let mut strsimerror_2_ref_0: &StrSimError = &mut strsimerror_2;
    let mut tuple_0: () = crate::RowId::assert_receiver_is_total_eq(rowid_0_ref_0);
    let mut rowid_1: crate::RowId = crate::RowId {val: isize_0};
    let mut rowid_1_ref_0: &crate::RowId = &mut rowid_1;
    let mut tuple_1: () = crate::RowId::assert_receiver_is_total_eq(rowid_1_ref_0);
    let mut usize_0: usize = crate::levenshtein(str_3_ref_0, str_2_ref_0);
    let mut f64_1: f64 = crate::normalized_levenshtein(str_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_34() {
    rusty_monitor::set_test_id(34);
    let mut u32_0: u32 = 3659u32;
    let mut i32_0: i32 = -15441i32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<u32>>> = std::option::Option::None;
    let mut i32_1: i32 = -12949i32;
    let mut i32_2: i32 = -3042i32;
    let mut i32_3: i32 = 5380i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u32> = crate::GrowingHashmapChar {used: i32_3, fill: i32_2, mask: i32_1, map: option_0};
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<u32> = &mut growinghashmapchar_0;
    let mut char_0: char = 'i';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<i16> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &crate::HybridGrowingHashmapChar<i16> = &mut hybridgrowinghashmapchar_0;
    let mut option_1: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<isize>>> = std::option::Option::None;
    let mut i32_4: i32 = -5067i32;
    let mut i32_5: i32 = -31140i32;
    let mut i32_6: i32 = 1228i32;
    let mut str_0: &str = "tMrWET";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "z5On6CrGE2ih4bL";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u32_1: u32 = 4426u32;
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<i128> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_1_ref_0: &mut crate::GrowingHashmapChar<i128> = &mut growinghashmapchar_1;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut i128_0: &mut i128 = crate::GrowingHashmapChar::get_mut(growinghashmapchar_1_ref_0, u32_1);
    let mut f64_0: f64 = crate::normalized_damerau_levenshtein(str_1_ref_0, str_0_ref_0);
    let mut growinghashmapchar_2: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar {used: i32_6, fill: i32_5, mask: i32_4, map: option_1};
    let mut i16_0: i16 = crate::HybridGrowingHashmapChar::get(hybridgrowinghashmapchar_0_ref_0, char_0);
    crate::GrowingHashmapChar::grow(growinghashmapchar_0_ref_0, i32_0);
    let mut hybridgrowinghashmapchar_1: crate::HybridGrowingHashmapChar<u128> = crate::HybridGrowingHashmapChar::default();
    let mut growinghashmapchar_2_ref_0: &crate::GrowingHashmapChar<isize> = &mut growinghashmapchar_2;
    let mut isize_0: isize = crate::GrowingHashmapChar::get(growinghashmapchar_2_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_35() {
    rusty_monitor::set_test_id(35);
    let mut str_0: &str = "QeFZD7OOXOdz73OoY";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "2se";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut char_0: char = '.';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<isize> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &crate::HybridGrowingHashmapChar<isize> = &mut hybridgrowinghashmapchar_0;
    let mut usize_0: usize = 872usize;
    let mut usize_1: usize = 239usize;
    let mut usize_2: usize = 9434usize;
    let mut char_1: char = '=';
    let mut hybridgrowinghashmapchar_1: crate::HybridGrowingHashmapChar<i16> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_1_ref_0: &crate::HybridGrowingHashmapChar<i16> = &mut hybridgrowinghashmapchar_1;
    let mut u32_0: u32 = 3661u32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<bool> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_0_ref_0: &crate::GrowingHashmapChar<bool> = &mut growinghashmapchar_0;
    let mut usize_3: usize = 2330usize;
    let mut usize_4: usize = 3892usize;
    let mut usize_5: usize = 761usize;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut usize_6: usize = crate::flat_index(usize_5, usize_4, usize_3);
    let mut bool_0: bool = crate::GrowingHashmapChar::get(growinghashmapchar_0_ref_0, u32_0);
    let mut i16_0: i16 = crate::HybridGrowingHashmapChar::get(hybridgrowinghashmapchar_1_ref_0, char_1);
    let mut usize_7: usize = crate::flat_index(usize_2, usize_1, usize_0);
    let mut isize_0: isize = crate::HybridGrowingHashmapChar::get(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut tuple_0: () = crate::RowId::assert_receiver_is_total_eq(rowid_0_ref_0);
    let mut f64_0: f64 = crate::sorensen_dice(str_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_36() {
    rusty_monitor::set_test_id(36);
    let mut u32_0: u32 = 2476u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<i128>>> = std::option::Option::None;
    let mut i32_0: i32 = 16991i32;
    let mut i32_1: i32 = -9800i32;
    let mut i32_2: i32 = 3285i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<i128> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &crate::GrowingHashmapChar<i128> = &mut growinghashmapchar_0;
    let mut str_0: &str = "kOPlcJU55qZ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "mxtC7AxQugm";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut char_0: char = ';';
    let mut str_2: &str = "DNwJV";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "uNplxH";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "HnZdX9XUdoheCIZm";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "MFL7w8IPg3Fl2hCE";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<i128> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_1: crate::HybridGrowingHashmapChar<u128> = crate::HybridGrowingHashmapChar::default();
    let mut usize_0: usize = crate::levenshtein(str_5_ref_0, str_4_ref_0);
    let mut usize_1: usize = crate::osa_distance(str_3_ref_0, str_2_ref_0);
    let mut hybridgrowinghashmapchar_1_ref_0: &crate::HybridGrowingHashmapChar<u128> = &mut hybridgrowinghashmapchar_1;
    let mut u128_0: u128 = crate::HybridGrowingHashmapChar::get(hybridgrowinghashmapchar_1_ref_0, char_0);
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<i128> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<u32> = crate::GrowingHashmapChar::default();
    let mut f64_0: f64 = crate::sorensen_dice(str_1_ref_0, str_0_ref_0);
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<i32> = crate::GrowingHashmapMapElemChar::default();
    let mut i128_0: i128 = crate::GrowingHashmapChar::get(growinghashmapchar_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_37() {
    rusty_monitor::set_test_id(37);
    let mut char_0: char = ']';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<i128> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &mut crate::HybridGrowingHashmapChar<i128> = &mut hybridgrowinghashmapchar_0;
    let mut str_0: &str = "W9THVW81XpeHGVhxo";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "ud9Qc7yaavuH";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u16> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<u16> = &mut growinghashmapchar_0;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "6AJiJ8yhZSOJ0jRqMx9";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<i128> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_1_ref_0: &mut crate::GrowingHashmapChar<i128> = &mut growinghashmapchar_1;
    let mut u32_0: u32 = 779u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<isize>>> = std::option::Option::None;
    let mut i32_0: i32 = 5117i32;
    let mut i32_1: i32 = -10671i32;
    let mut i32_2: i32 = -16629i32;
    let mut growinghashmapchar_2: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_2_ref_0: &crate::GrowingHashmapChar<isize> = &mut growinghashmapchar_2;
    let mut str_4: &str = "cWjNMq7WS1";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "YDRnt6yA";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut f64_0: f64 = crate::jaro(str_5_ref_0, str_4_ref_0);
    let mut usize_0: usize = crate::GrowingHashmapChar::lookup(growinghashmapchar_2_ref_0, u32_0);
    crate::GrowingHashmapChar::allocate(growinghashmapchar_1_ref_0);
    let mut f64_1: f64 = crate::jaro(str_3_ref_0, str_2_ref_0);
    crate::GrowingHashmapChar::allocate(growinghashmapchar_0_ref_0);
    let mut f64_2: f64 = crate::jaro(str_1_ref_0, str_0_ref_0);
    let mut i128_0: &mut i128 = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_0_ref_0, char_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_38() {
    rusty_monitor::set_test_id(38);
    let mut str_0: &str = "wKdOtH1wBfvpD";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_0: isize = -4787isize;
    let mut u32_0: u32 = 5272u32;
    let mut isize_1: isize = -12119isize;
    let mut rowid_0: crate::RowId = crate::RowId {val: isize_1};
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut isize_2: isize = 12470isize;
    let mut rowid_1: crate::RowId = crate::RowId {val: isize_2};
    let mut rowid_1_ref_0: &crate::RowId = &mut rowid_1;
    let mut isize_3: isize = -19420isize;
    let mut rowid_2: crate::RowId = crate::RowId {val: isize_3};
    let mut rowid_2_ref_0: &crate::RowId = &mut rowid_2;
    let mut isize_4: isize = -2229isize;
    let mut rowid_3: crate::RowId = crate::RowId {val: isize_4};
    let mut rowid_3_ref_0: &crate::RowId = &mut rowid_3;
    let mut u32_1: u32 = 1322u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<i32>>> = std::option::Option::None;
    let mut i32_0: i32 = -6787i32;
    let mut i32_1: i32 = -5457i32;
    let mut i32_2: i32 = -1564i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<i32> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<i32> = &mut growinghashmapchar_0;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut i32_3: &mut i32 = crate::GrowingHashmapChar::get_mut(growinghashmapchar_0_ref_0, u32_1);
    let mut bool_0: bool = crate::RowId::eq(rowid_3_ref_0, rowid_2_ref_0);
    let mut rowid_4: crate::RowId = crate::RowId::clone(rowid_1_ref_0);
    let mut tuple_0: () = crate::RowId::assert_receiver_is_total_eq(rowid_0_ref_0);
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<isize> = crate::GrowingHashmapMapElemChar {key: u32_0, value: isize_0};
    crate::bigrams(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_39() {
    rusty_monitor::set_test_id(39);
    let mut u32_0: u32 = 6605u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<usize>>> = std::option::Option::None;
    let mut i32_0: i32 = 8163i32;
    let mut i32_1: i32 = -7864i32;
    let mut i32_2: i32 = -7820i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<usize> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<usize> = &mut growinghashmapchar_0;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut isize_0: isize = 16474isize;
    let mut rowid_1: crate::RowId = crate::RowId {val: isize_0};
    let mut rowid_1_ref_0: &crate::RowId = &mut rowid_1;
    let mut str_0: &str = "xUKZQho1cE9";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "YmZJhSWbMYztXMH";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut char_0: char = '\t';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<u8> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &mut crate::HybridGrowingHashmapChar<u8> = &mut hybridgrowinghashmapchar_0;
    let mut option_1: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<u8>>> = std::option::Option::None;
    let mut i32_3: i32 = -7277i32;
    let mut i32_4: i32 = -337i32;
    let mut i32_5: i32 = 3495i32;
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<u8> = crate::GrowingHashmapChar {used: i32_5, fill: i32_4, mask: i32_3, map: option_1};
    let mut growinghashmapchar_1_ref_0: &mut crate::GrowingHashmapChar<u8> = &mut growinghashmapchar_1;
    let mut option_2: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<isize>>> = std::option::Option::None;
    let mut i32_6: i32 = 5094i32;
    let mut i32_7: i32 = 542i32;
    let mut i32_8: i32 = -6108i32;
    let mut growinghashmapchar_2: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar {used: i32_8, fill: i32_7, mask: i32_6, map: option_2};
    crate::GrowingHashmapChar::allocate(growinghashmapchar_1_ref_0);
    let mut u8_0: &mut u8 = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut f64_0: f64 = crate::sorensen_dice(str_1_ref_0, str_0_ref_0);
    let mut hybridgrowinghashmapchar_1: crate::HybridGrowingHashmapChar<u128> = crate::HybridGrowingHashmapChar::default();
    let mut bool_0: bool = crate::RowId::eq(rowid_1_ref_0, rowid_0_ref_0);
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<u8> = crate::GrowingHashmapMapElemChar::default();
    let mut usize_0: &mut usize = crate::GrowingHashmapChar::get_mut(growinghashmapchar_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_40() {
    rusty_monitor::set_test_id(40);
    let mut str_0: &str = "veFCt2vV9E";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "oVHJXoasq";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "mpr9afsZB";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut char_0: char = 'U';
    let mut str_3: &str = "JF54et2IQff";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u32_0: u32 = 1954u32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<i128> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<i128> = &mut growinghashmapchar_0;
    let mut u32_1: u32 = 3581u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<usize>>> = std::option::Option::None;
    let mut i32_0: i32 = 5339i32;
    let mut i32_1: i32 = -9156i32;
    let mut i32_2: i32 = 13315i32;
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<usize> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_1_ref_0: &mut crate::GrowingHashmapChar<usize> = &mut growinghashmapchar_1;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut bool_0: bool = crate::StrSimError::eq(strsimerror_1_ref_0, strsimerror_0_ref_0);
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<bool> = crate::HybridGrowingHashmapChar::default();
    let mut usize_0: &mut usize = crate::GrowingHashmapChar::get_mut(growinghashmapchar_1_ref_0, u32_1);
    let mut i128_0: &mut i128 = crate::GrowingHashmapChar::get_mut(growinghashmapchar_0_ref_0, u32_0);
    crate::bigrams(str_3_ref_0);
    let mut hybridgrowinghashmapchar_0_ref_0: &crate::HybridGrowingHashmapChar<bool> = &mut hybridgrowinghashmapchar_0;
    let mut bool_1: bool = crate::HybridGrowingHashmapChar::get(hybridgrowinghashmapchar_0_ref_0, char_0);
    crate::bigrams(str_2_ref_0);
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<i128> = crate::GrowingHashmapMapElemChar::default();
    crate::hamming(str_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_41() {
    rusty_monitor::set_test_id(41);
    let mut str_0: &str = "6NqTu8T";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "G1";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "05kdvNFWdNxjB";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "5qC4Hu2o05Fm";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut char_0: char = 'U';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<u32> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &mut crate::HybridGrowingHashmapChar<u32> = &mut hybridgrowinghashmapchar_0;
    let mut i64_0: i64 = -2244i64;
    let mut u32_0: u32 = 9072u32;
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<i64> = crate::GrowingHashmapMapElemChar {key: u32_0, value: i64_0};
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<i64> = &mut growinghashmapmapelemchar_0;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut str_4: &str = "lYuvaEv1j";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "2nobbPHGuaA5gmFyKLk";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "FUW";
    let mut str_6_ref_0: &str = &mut str_6;
    crate::hamming(str_6_ref_0, str_5_ref_0);
    crate::bigrams(str_4_ref_0);
    let mut bool_0: bool = crate::StrSimError::eq(strsimerror_1_ref_0, strsimerror_0_ref_0);
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<i64> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    let mut u32_1: &mut u32 = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut f64_0: f64 = crate::jaro_winkler(str_3_ref_0, str_2_ref_0);
    let mut f64_1: f64 = crate::jaro(str_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_42() {
    rusty_monitor::set_test_id(42);
    let mut isize_0: isize = 11936isize;
    let mut rowid_0: crate::RowId = crate::RowId {val: isize_0};
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut u32_0: u32 = 7372u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<usize>>> = std::option::Option::None;
    let mut i32_0: i32 = -3034i32;
    let mut i32_1: i32 = -7036i32;
    let mut i32_2: i32 = 13938i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<usize> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<usize> = &mut growinghashmapchar_0;
    let mut usize_0: usize = 6201usize;
    let mut usize_1: usize = 5086usize;
    let mut usize_2: usize = 2636usize;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "paeIjWRqywO8XIbE9QH";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "UQQRD0Jmp5nNPKb";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "45pHj9wEn5JT85fg2RG";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut f64_0: f64 = crate::normalized_levenshtein(str_3_ref_0, str_2_ref_0);
    crate::hamming(str_1_ref_0, str_0_ref_0);
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<i128> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_2: crate::GrowingHashmapChar<u128> = crate::GrowingHashmapChar::default();
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut usize_3: usize = crate::flat_index(usize_2, usize_1, usize_0);
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<u128> = crate::HybridGrowingHashmapChar::default();
    let mut usize_4: &mut usize = crate::GrowingHashmapChar::get_mut(growinghashmapchar_0_ref_0, u32_0);
    let mut tuple_0: () = crate::RowId::assert_receiver_is_total_eq(rowid_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_43() {
    rusty_monitor::set_test_id(43);
    let mut str_0: &str = "Em6Y";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "HOFX";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Y";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut isize_0: isize = 2561isize;
    let mut str_4: &str = "DBs0nH10s9uS4";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "SDQ";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut char_0: char = 'm';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<bool> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &crate::HybridGrowingHashmapChar<bool> = &mut hybridgrowinghashmapchar_0;
    let mut u32_0: u32 = 5047u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<i64>>> = std::option::Option::None;
    let mut i32_0: i32 = -831i32;
    let mut i32_1: i32 = 10246i32;
    let mut i32_2: i32 = 3720i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<i64> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<i64> = &mut growinghashmapchar_0;
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<usize> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<usize> = &mut growinghashmapmapelemchar_0;
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<u32> = crate::GrowingHashmapChar::default();
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<usize> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    let mut i64_0: &mut i64 = crate::GrowingHashmapChar::get_mut(growinghashmapchar_0_ref_0, u32_0);
    let mut bool_0: bool = crate::HybridGrowingHashmapChar::get(hybridgrowinghashmapchar_0_ref_0, char_0);
    crate::hamming(str_5_ref_0, str_4_ref_0);
    let mut rowid_0: crate::RowId = crate::RowId {val: isize_0};
    let mut f64_0: f64 = crate::jaro(str_3_ref_0, str_2_ref_0);
    let mut usize_0: usize = crate::levenshtein(str_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<isize>>> = std::option::Option::None;
    let mut i32_0: i32 = -12059i32;
    let mut i32_1: i32 = -4391i32;
    let mut i32_2: i32 = -11142i32;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut rowid_1: crate::RowId = crate::RowId::default();
    let mut rowid_1_ref_0: &crate::RowId = &mut rowid_1;
    let mut char_0: char = '\r';
    let mut hybridgrowinghashmapchar_0: crate::HybridGrowingHashmapChar<u32> = crate::HybridGrowingHashmapChar::default();
    let mut hybridgrowinghashmapchar_0_ref_0: &mut crate::HybridGrowingHashmapChar<u32> = &mut hybridgrowinghashmapchar_0;
    let mut str_0: &str = "HaW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "htc8yY";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_0: usize = 6559usize;
    let mut usize_1: usize = 7969usize;
    let mut usize_2: usize = 5239usize;
    let mut str_2: &str = "iCcH2";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "SjdGILI6DpnlpvUq";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut f64_0: f64 = crate::sorensen_dice(str_3_ref_0, str_2_ref_0);
    let mut usize_3: usize = crate::flat_index(usize_2, usize_1, usize_0);
    let mut f64_1: f64 = crate::jaro(str_1_ref_0, str_0_ref_0);
    let mut u32_0: &mut u32 = crate::HybridGrowingHashmapChar::get_mut(hybridgrowinghashmapchar_0_ref_0, char_0);
    let mut bool_0: bool = crate::RowId::eq(rowid_1_ref_0, rowid_0_ref_0);
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<bool> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<bool> = &mut growinghashmapmapelemchar_0;
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<bool> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_45() {
    rusty_monitor::set_test_id(45);
    let mut i32_0: i32 = -5748i32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<usize>>> = std::option::Option::None;
    let mut i32_1: i32 = -9094i32;
    let mut i32_2: i32 = 16110i32;
    let mut i32_3: i32 = 251i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<usize> = crate::GrowingHashmapChar {used: i32_3, fill: i32_2, mask: i32_1, map: option_0};
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<usize> = &mut growinghashmapchar_0;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut str_0: &str = "Jh8vQzF8aA";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "DFaUaUoRYW8yhkdR";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut str_2: &str = "mU";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "YsvjcjAWmINm";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "gizuLhlMPgf6wyAVs1W";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut u32_0: u32 = 9213u32;
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<u64> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_1_ref_0: &crate::GrowingHashmapChar<u64> = &mut growinghashmapchar_1;
    let mut usize_0: usize = crate::GrowingHashmapChar::lookup(growinghashmapchar_1_ref_0, u32_0);
    let mut usize_1: usize = crate::osa_distance(str_5_ref_0, str_4_ref_0);
    crate::bigrams(str_3_ref_0);
    crate::bigrams(str_2_ref_0);
    let mut usize_2: usize = crate::damerau_levenshtein(str_1_ref_0, str_0_ref_0);
    crate::GrowingHashmapChar::grow(growinghashmapchar_0_ref_0, i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_46() {
    rusty_monitor::set_test_id(46);
    let mut str_0: &str = "9lOPZvppZ92HcNSKQ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "gHg0dAKd";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "Dt6nVwBM9hanW";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "1PHZ4dS6et18";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<isize>>> = std::option::Option::None;
    let mut i32_0: i32 = 518i32;
    let mut i32_1: i32 = -5860i32;
    let mut i32_2: i32 = -3059i32;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut isize_0: isize = 6233isize;
    let mut rowid_0: crate::RowId = crate::RowId {val: isize_0};
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut str_4: &str = "y";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "GZhQf";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut isize_1: isize = 7724isize;
    let mut rowid_1: crate::RowId = crate::RowId {val: isize_1};
    let mut usize_0: usize = crate::damerau_levenshtein(str_5_ref_0, str_4_ref_0);
    let mut rowid_2: crate::RowId = crate::RowId::clone(rowid_0_ref_0);
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u16> = crate::GrowingHashmapChar::default();
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<u8> = crate::GrowingHashmapMapElemChar::default();
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut usize_1: usize = crate::levenshtein(str_3_ref_0, str_2_ref_0);
    let mut growinghashmapchar_2: crate::GrowingHashmapChar<bool> = crate::GrowingHashmapChar::default();
    crate::hamming(str_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_47() {
    rusty_monitor::set_test_id(47);
    let mut str_0: &str = "8IvMWzfBVmU9RgbPA6";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "H44vS48pTk9PWDGcJ8O";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "qcTA";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "tuZyVF";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<isize>>> = std::option::Option::None;
    let mut i32_0: i32 = -14357i32;
    let mut i32_1: i32 = 1113i32;
    let mut i32_2: i32 = -24i32;
    let mut isize_0: isize = -6027isize;
    let mut strsimerror_0: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_0_ref_0: &StrSimError = &mut strsimerror_0;
    let mut strsimerror_1: StrSimError = crate::StrSimError::DifferentLengthArgs;
    let mut strsimerror_1_ref_0: &StrSimError = &mut strsimerror_1;
    let mut str_4: &str = "mpwCx";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "KG";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "BkvMSCvD18we9";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut f64_0: f64 = crate::jaro(str_6_ref_0, str_5_ref_0);
    crate::bigrams(str_4_ref_0);
    let mut bool_0: bool = crate::StrSimError::eq(strsimerror_1_ref_0, strsimerror_0_ref_0);
    let mut rowid_0: crate::RowId = crate::RowId {val: isize_0};
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut usize_0: usize = crate::levenshtein(str_3_ref_0, str_2_ref_0);
    let mut usize_1: usize = crate::osa_distance(str_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_48() {
    rusty_monitor::set_test_id(48);
    let mut str_0: &str = "phZipt3nVwy";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "5Ms42n2YL1mGdV";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut rowid_0: crate::RowId = crate::RowId::default();
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut isize_0: isize = 8504isize;
    let mut str_2: &str = "lKnKzWir8SnE";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "ai";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut i32_0: i32 = -15259i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<u16> = crate::GrowingHashmapChar::default();
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<u16> = &mut growinghashmapchar_0;
    let mut u32_0: u32 = 4286u32;
    let mut str_4: &str = "V3x";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "0zoqpaeLhJfs8Px5AcJ";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut vec_0: std::vec::Vec<crate::GrowingHashmapMapElemChar<isize>> = std::vec::Vec::new();
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<isize>>> = std::option::Option::Some(vec_0);
    let mut i32_1: i32 = 9292i32;
    let mut i32_2: i32 = -3925i32;
    let mut i32_3: i32 = 4935i32;
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar {used: i32_3, fill: i32_2, mask: i32_1, map: option_0};
    let mut f64_0: f64 = crate::jaro_winkler(str_5_ref_0, str_4_ref_0);
    let mut growinghashmapchar_1_ref_0: &crate::GrowingHashmapChar<isize> = &mut growinghashmapchar_1;
    let mut isize_1: isize = crate::GrowingHashmapChar::get(growinghashmapchar_1_ref_0, u32_0);
    crate::GrowingHashmapChar::grow(growinghashmapchar_0_ref_0, i32_0);
    let mut f64_1: f64 = crate::sorensen_dice(str_3_ref_0, str_2_ref_0);
    let mut rowid_1: crate::RowId = crate::RowId {val: isize_0};
    let mut rowid_2: crate::RowId = crate::RowId::clone(rowid_0_ref_0);
    let mut usize_0: usize = crate::levenshtein(str_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_49() {
    rusty_monitor::set_test_id(49);
    let mut isize_0: isize = -13994isize;
    let mut rowid_0: crate::RowId = crate::RowId {val: isize_0};
    let mut rowid_0_ref_0: &crate::RowId = &mut rowid_0;
    let mut str_0: &str = "GXbEZnjSKXNoOHo54";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u32_0: u32 = 8059u32;
    let mut option_0: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<isize>>> = std::option::Option::None;
    let mut i32_0: i32 = -2573i32;
    let mut i32_1: i32 = 11213i32;
    let mut i32_2: i32 = -5453i32;
    let mut growinghashmapchar_0: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar {used: i32_2, fill: i32_1, mask: i32_0, map: option_0};
    let mut growinghashmapchar_0_ref_0: &mut crate::GrowingHashmapChar<isize> = &mut growinghashmapchar_0;
    let mut option_1: std::option::Option<std::vec::Vec<crate::GrowingHashmapMapElemChar<isize>>> = std::option::Option::None;
    let mut i32_3: i32 = 22865i32;
    let mut i32_4: i32 = -18568i32;
    let mut i32_5: i32 = -4594i32;
    let mut str_1: &str = "JpiqPHx5M41QL4o4HEw";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "pKbnBgzz0V0k";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut isize_1: isize = 3500isize;
    let mut u32_1: u32 = 7377u32;
    let mut growinghashmapmapelemchar_0: crate::GrowingHashmapMapElemChar<isize> = crate::GrowingHashmapMapElemChar {key: u32_1, value: isize_1};
    let mut f64_0: f64 = crate::normalized_levenshtein(str_2_ref_0, str_1_ref_0);
    let mut growinghashmapchar_1: crate::GrowingHashmapChar<isize> = crate::GrowingHashmapChar {used: i32_5, fill: i32_4, mask: i32_3, map: option_1};
    let mut growinghashmapmapelemchar_0_ref_0: &crate::GrowingHashmapMapElemChar<isize> = &mut growinghashmapmapelemchar_0;
    let mut growinghashmapmapelemchar_1: crate::GrowingHashmapMapElemChar<isize> = crate::GrowingHashmapMapElemChar::clone(growinghashmapmapelemchar_0_ref_0);
    let mut rowid_1: crate::RowId = crate::RowId::default();
    let mut isize_2: &mut isize = crate::GrowingHashmapChar::get_mut(growinghashmapchar_0_ref_0, u32_0);
    let mut rowid_1_ref_0: &crate::RowId = &mut rowid_1;
    let mut rowid_2: crate::RowId = crate::RowId::clone(rowid_1_ref_0);
    crate::bigrams(str_0_ref_0);
    let mut tuple_0: () = crate::RowId::assert_receiver_is_total_eq(rowid_0_ref_0);
    panic!("From RustyUnit with love");
}
}