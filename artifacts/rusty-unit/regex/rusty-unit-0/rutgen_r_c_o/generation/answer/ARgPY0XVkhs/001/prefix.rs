// Answer 0

#[test]
fn test_new_with_empty_literals() {
    let lits = Literals::new(vec![]);
    let sset = SingleByteSet {
        sparse: vec![],
        dense: vec![],
        complete: false,
        all_ascii: false,
    };
    Matcher::new(&lits, sset);
}

