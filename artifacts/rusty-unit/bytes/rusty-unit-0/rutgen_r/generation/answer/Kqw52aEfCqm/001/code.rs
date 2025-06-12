// Answer 0

#[test]
fn test_borrow_with_non_empty_slice() {
    struct Bytes {
        data: Vec<u8>,
    }

    impl Bytes {
        fn as_slice(&self) -> &[u8] {
            &self.data
        }

        fn borrow(&self) -> &[u8] {
            self.as_slice()
        }
    }

    let bytes = Bytes { data: vec![1, 2, 3, 4, 5] };
    let result = bytes.borrow();
    assert_eq!(result, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_borrow_with_empty_slice() {
    struct Bytes {
        data: Vec<u8>,
    }

    impl Bytes {
        fn as_slice(&self) -> &[u8] {
            &self.data
        }

        fn borrow(&self) -> &[u8] {
            self.as_slice()
        }
    }

    let bytes = Bytes { data: vec![] };
    let result = bytes.borrow();
    assert_eq!(result, &[]);
}

