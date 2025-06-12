// Answer 0

#[test]
fn test_borrow() {
    struct BytesWrapper {
        data: Vec<u8>,
    }

    impl BytesWrapper {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }

        fn new(data: Vec<u8>) -> Self {
            BytesWrapper { data }
        }
    }

    let bytes = BytesWrapper::new(vec![1, 2, 3, 4, 5]);
    let borrowed = bytes.borrow();
    assert_eq!(borrowed, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_borrow_empty() {
    struct BytesWrapper {
        data: Vec<u8>,
    }

    impl BytesWrapper {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }

        fn new(data: Vec<u8>) -> Self {
            BytesWrapper { data }
        }
    }

    let bytes = BytesWrapper::new(vec![]);
    let borrowed = bytes.borrow();
    assert_eq!(borrowed, &[]);
}

