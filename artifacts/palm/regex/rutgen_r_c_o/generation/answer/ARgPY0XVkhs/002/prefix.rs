// Answer 0

#[test]
fn test_new_with_empty_lits() {
    let lits = Literals::new(vec![]);
    let sset = SingleByteSet {
        sparse: vec![],
        dense: (0..26).map(|x| x as u8).collect(),
        complete: true,
        all_ascii: true,
    };
    Matcher::new(&lits, sset);
}

#[test]
fn test_new_with_large_dense() {
    let lits = Literals::new(vec![vec![1]]);
    let sset = SingleByteSet {
        sparse: vec![],
        dense: (0..26).map(|x| x as u8).collect(),
        complete: true,
        all_ascii: true,
    };
    Matcher::new(&lits, sset);
}

