// Answer 0

#[test]
fn test_case_1() {
    let lits = Literals::new(vec![vec![b'a'], vec![b'b']]);
    let sset = SingleByteSet {
        sparse: vec![],
        dense: vec![],
        complete: false,
        all_ascii: true,
    };
    Matcher::new(&lits, sset);
}

#[test]
fn test_case_2() {
    let lits = Literals::new(vec![vec![b'c'], vec![b'd']]);
    let sset = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'c', b'd'],
        complete: false,
        all_ascii: true,
    };
    Matcher::new(&lits, sset);
}

#[test]
fn test_case_3() {
    let lits = Literals::new(vec![vec![b'e'], vec![b'f']]);
    let sset = SingleByteSet {
        sparse: vec![true, false, false, false, false, true],
        dense: vec![b'e'],
        complete: false,
        all_ascii: true,
    };
    Matcher::new(&lits, sset);
}

#[test]
fn test_case_4() {
    let lits = Literals::new(vec![
        vec![b'g', b'h'],
        vec![b'i', b'j']
    ]);
    let sset = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'g', b'h'],
        complete: false,
        all_ascii: false,
    };
    Matcher::new(&lits, sset);
}

#[test]
fn test_case_5() {
    let lits = Literals::new(vec![
        vec![b'k', b'l'],
        vec![b'm', b'n']
    ]);
    let sset = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'k', b'l', b'm'],
        complete: false,
        all_ascii: false,
    };
    Matcher::new(&lits, sset);
}

