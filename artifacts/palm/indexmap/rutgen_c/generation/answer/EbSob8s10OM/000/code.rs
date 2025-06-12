// Answer 0

#[test]
fn test_first_empty() {
    struct DummyHashBuilder;
    
    #[cfg(not(feature = "std"))]
    struct DummyIndexSet {
        entries: Vec<Bucket<i32, ()>>,
    }

    impl DummyIndexSet {
        fn as_entries(&self) -> &[Bucket<i32, ()>] {
            &self.entries
        }
    }

    let set = DummyIndexSet { entries: vec![] };
    assert_eq!(set.first(), None);
}

#[test]
fn test_first_non_empty() {
    struct DummyHashBuilder;
    
    #[cfg(not(feature = "std"))]
    struct DummyIndexSet {
        entries: Vec<Bucket<i32, ()>>,
    }

    impl DummyIndexSet {
        fn as_entries(&self) -> &[Bucket<i32, ()>] {
            &self.entries
        }
    }

    let bucket1 = Bucket { hash: 1, key: 10, value: () };
    let bucket2 = Bucket { hash: 2, key: 20, value: () };
    
    let set = DummyIndexSet { entries: vec![bucket1, bucket2] };
    assert_eq!(set.first().map(|b| b.key), Some(10));
}

