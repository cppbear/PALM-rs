// Answer 0

#[test]
fn test_find_empty_dense() {
    let set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![],
        complete: false,
        all_ascii: false,
    };
    assert_eq!(set.find(b"test"), None);
}

#[test]
fn test_find_single_dense() {
    let set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a'],
        complete: false,
        all_ascii: false,
    };
    assert_eq!(set.find(b"test"), None);
    assert_eq!(set.find(b"a"), Some(0));
}

#[test]
fn test_find_double_dense() {
    let set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a', b'b'],
        complete: false,
        all_ascii: false,
    };
    assert_eq!(set.find(b"test"), None);
    assert_eq!(set.find(b"ab"), Some(0));
    assert_eq!(set.find(b"ba"), None);
    assert_eq!(set.find(b"b"), Some(0));
}

#[test]
fn test_find_triple_dense() {
    let set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a', b'b', b'c'],
        complete: false,
        all_ascii: false,
    };
    assert_eq!(set.find(b"test"), None);
    assert_eq!(set.find(b"cba"), Some(1));
    assert_eq!(set.find(b"abc"), Some(0));
    assert_eq!(set.find(b"b"), Some(1));
}

#[test]
fn test_find_large_dense() {
    let set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'x', b'y', b'z', b'a', b'b'],
        complete: false,
        all_ascii: false,
    };
    assert_eq!(set.find(b"test"), None);
    assert_eq!(set.find(b"bzz"), Some(1));
    assert_eq!(set.find(b"xab"), Some(0));
    assert_eq!(set.find(b"a"), Some(3));
}

