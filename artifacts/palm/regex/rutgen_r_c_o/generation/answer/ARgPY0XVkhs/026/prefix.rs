// Answer 0

#[test]
fn test_new_with_single_literal_and_empty_sset() {
    let literals = Literals::new(vec![b"test".to_vec()]); // lits.literals().len() == 1
    let sset = SingleByteSet {
        sparse: vec![],
        dense: vec![],
        complete: false,
        all_ascii: false,
    }; // sset.dense.len() == 0, sset.complete == false
    Matcher::new(&literals, sset);
}

#[test]
fn test_new_with_single_literal_and_partial_sset() {
    let literals = Literals::new(vec![b"example".to_vec()]); // lits.literals().len() == 1
    let sset = SingleByteSet {
        sparse: vec![false; 10],
        dense: vec![b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z'],
        complete: false,
        all_ascii: false,
    }; // sset.dense.len() = 26, which should be avoided
    Matcher::new(&literals, sset);
}

#[test]
fn test_new_with_single_literal_and_few_dense_bytes() {
    let literals = Literals::new(vec![b"unique".to_vec()]); // lits.literals().len() == 1
    let sset = SingleByteSet {
        sparse: vec![false; 5],
        dense: vec![b'a', b'b', b'c', b'd', b'e'], 
        complete: false,
        all_ascii: false,
    }; // sset.dense.len() is less than 26
    Matcher::new(&literals, sset);
}

#[test]
fn test_new_with_single_literal_and_all_dense_bytes() {
    let literals = Literals::new(vec![b"yetAnother".to_vec()]); // lits.literals().len() == 1
    let sset = SingleByteSet {
        sparse: vec![false; 20],
        dense: (b'a'..=b'z').collect(), // sset.dense.len() = 26, should be avoided
        complete: false,
        all_ascii: false,
    };
    Matcher::new(&literals, sset);
}

