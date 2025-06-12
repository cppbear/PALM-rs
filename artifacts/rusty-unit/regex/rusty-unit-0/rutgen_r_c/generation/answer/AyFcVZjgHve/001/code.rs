// Answer 0

#[test]
fn test_len_non_empty() {
    struct TestInputAt {
        pos: usize,
        c: Char,
        byte: Option<u8>,
        len: usize,
    }

    let input = TestInputAt {
        pos: 0,
        c: Char(97), // ASCII 'a'
        byte: Some(97),
        len: 1,
    };
    assert_eq!(input.len(), 1);
}

#[test]
fn test_len_zero_length() {
    struct TestInputAt {
        pos: usize,
        c: Char,
        byte: Option<u8>,
        len: usize,
    }

    let input = TestInputAt {
        pos: 0,
        c: Char(0), // null character
        byte: None,
        len: 0,
    };
    assert_eq!(input.len(), 0);
}

#[test]
fn test_len_large_length() {
    struct TestInputAt {
        pos: usize,
        c: Char,
        byte: Option<u8>,
        len: usize,
    }

    let input = TestInputAt {
        pos: 5,
        c: Char(100), // some character
        byte: Some(100),
        len: usize::MAX, // maximum length
    };
    assert_eq!(input.len(), usize::MAX);
}

#[test]
fn test_len_with_boundary_condition() {
    struct TestInputAt {
        pos: usize,
        c: Char,
        byte: Option<u8>,
        len: usize,
    }

    let input = TestInputAt {
        pos: 1,
        c: Char(120), // some character
        byte: Some(120),
        len: 1, // minimum non-zero length
    };
    assert_eq!(input.len(), 1);
}

