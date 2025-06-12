// Answer 0

#[test]
fn test_get_mut_returns_mutable_reference() {
    struct MockBufMut {
        data: Vec<u8>,
    }

    impl MockBufMut {
        fn new() -> Self {
            Self { data: vec![0; 10] }
        }
    }

    let mut limit = Limit {
        inner: MockBufMut::new(),
        limit: 10,
    };

    let inner_ref = limit.get_mut();
    inner_ref.data.push(1);

    assert_eq!(inner_ref.data.len(), 11);
}

#[test]
fn test_get_mut_alters_inner_data() {
    struct MockBufMut {
        data: Vec<u8>,
    }

    impl MockBufMut {
        fn new() -> Self {
            Self { data: vec![0; 5] }
        }
    }

    let mut limit = Limit {
        inner: MockBufMut::new(),
        limit: 5,
    };

    {
        let inner_mut = limit.get_mut();
        inner_mut.data[0] = 42;
    }

    assert_eq!(limit.get_ref().data[0], 42);
}

