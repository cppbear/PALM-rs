// Answer 0

#[test]
fn test_peek_with_valid_index() {
    struct Tester {
        slice: Vec<u8>,
        index: usize,
    }

    let mut tester = Tester {
        slice: vec![1, 2, 3],
        index: 1,
    };

    assert_eq!(tester.peek().unwrap(), Some(2));
}

#[test]
fn test_peek_with_zero_index() {
    struct Tester {
        slice: Vec<u8>,
        index: usize,
    }

    let mut tester = Tester {
        slice: vec![5, 10, 15],
        index: 0,
    };

    assert_eq!(tester.peek().unwrap(), Some(5));
}

#[test]
fn test_peek_at_end_of_slice() {
    struct Tester {
        slice: Vec<u8>,
        index: usize,
    }

    let mut tester = Tester {
        slice: vec![10, 20, 30],
        index: 3,
    };

    assert_eq!(tester.peek().unwrap(), None);
}

#[test]
fn test_peek_empty_slice() {
    struct Tester {
        slice: Vec<u8>,
        index: usize,
    }

    let mut tester = Tester {
        slice: vec![],
        index: 0,
    };

    assert_eq!(tester.peek().unwrap(), None);
}

