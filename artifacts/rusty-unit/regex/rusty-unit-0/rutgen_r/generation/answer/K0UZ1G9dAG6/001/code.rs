// Answer 0


struct Literals {
    data: Vec<&'static str>,
}

struct SingleByteSet;

impl SingleByteSet {
    fn prefixes(lits: &Literals) -> Vec<&'static str> {
        lits.data.iter().map(|s| &s[..1]).collect()
    }
}

struct Matcher<'a> {
    literals: &'a Literals,
    single_byte_set: Vec<&'static str>,
}

impl<'a> Matcher<'a> {
    fn new(lits: &'a Literals, sset: Vec<&'static str>) -> Self {
        Matcher {
            literals: lits,
            single_byte_set: sset,
        }
    }
}

#[test]
fn test_prefixes_with_empty_literals() {
    let lits = Literals { data: vec![] };
    let result = Matcher::new(&lits, SingleByteSet::prefixes(&lits));
    assert_eq!(result.single_byte_set.len(), 0);
}

#[test]
fn test_prefixes_with_single_literal() {
    let lits = Literals { data: vec!["a"] };
    let result = Matcher::new(&lits, SingleByteSet::prefixes(&lits));
    assert_eq!(result.single_byte_set, vec!["a"]);
}

#[test]
fn test_prefixes_with_multiple_literals() {
    let lits = Literals { data: vec!["apple", "banana", "cherry"] };
    let result = Matcher::new(&lits, SingleByteSet::prefixes(&lits));
    assert_eq!(result.single_byte_set, vec!["a", "b", "c"]);
}

#[test]
fn test_prefixes_with_literals_of_different_lengths() {
    let lits = Literals { data: vec!["ant", "bear", "cat"] };
    let result = Matcher::new(&lits, SingleByteSet::prefixes(&lits));
    assert_eq!(result.single_byte_set, vec!["a", "b", "c"]);
}

#[should_panic]
fn test_prefixes_with_long_strings() {
    let lits = Literals { data: vec!["this_is_a_long_string"] };
    let result = Matcher::new(&lits, SingleByteSet::prefixes(&lits));
    assert_eq!(result.single_byte_set, vec!["t"]);  // Just testing that panic doesn't occur
}


