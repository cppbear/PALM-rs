// Answer 0

#[test]
fn test_new_with_valid_inputs_complete_set() {
    let literals = Literals::from(vec![vec![b'a'], vec![b'b']]);
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 26],
        dense: vec![b'a', b'b'],
        complete: true,
        all_ascii: true,
    };
    Matcher::new(&literals, single_byte_set);
}

#[test]
fn test_new_with_multiple_literals_complete_set() {
    let literals = Literals::from(vec![vec![b'a'], vec![b'b'], vec![b'c'], vec![b'd']]);
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 26],
        dense: vec![b'a', b'b', b'c', b'd'],
        complete: true,
        all_ascii: true,
    };
    Matcher::new(&literals, single_byte_set);
}

#[test]
fn test_new_with_small_lits_complete_set() {
    let literals = Literals::from(vec![vec![b'a']]);
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 26],
        dense: vec![b'a'],
        complete: true,
        all_ascii: true,
    };
    Matcher::new(&literals, single_byte_set);
}

