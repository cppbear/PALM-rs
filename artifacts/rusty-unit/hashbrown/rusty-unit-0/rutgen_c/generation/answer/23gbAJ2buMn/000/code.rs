// Answer 0

#[test]
fn test_iter_hash_mut_debug() {
    struct MockRawIterHash;

    impl Clone for MockRawIterHash {
        fn clone(&self) -> Self {
            MockRawIterHash
        }
    }

    let raw_iter_hash = MockRawIterHash;
    let iter_hash_mut = IterHashMut {
        inner: raw_iter_hash,
        marker: PhantomData,
    };

    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        iter_hash_mut.fmt(&mut formatter).unwrap();
    }

    assert!(!output.is_empty());
}

#[test]
fn test_iter_hash_mut_debug_empty() {
    struct MockRawIterHash;

    impl Clone for MockRawIterHash {
        fn clone(&self) -> Self {
            MockRawIterHash
        }
    }

    let raw_iter_hash = MockRawIterHash;
    let iter_hash_mut = IterHashMut {
        inner: raw_iter_hash,
        marker: PhantomData,
    };

    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        iter_hash_mut.fmt(&mut formatter).unwrap();
    }

    assert_eq!(output, "[ ]");
}

