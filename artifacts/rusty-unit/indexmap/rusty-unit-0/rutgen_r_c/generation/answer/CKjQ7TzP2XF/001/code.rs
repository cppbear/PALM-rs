// Answer 0

#[test]
fn test_try_reserve_exact_success() {
    struct TestEntries;

    impl Entries for TestEntries {
        type Entry = Bucket<usize, usize>;

        fn into_entries(self) -> Vec<Self::Entry> {
            vec![]
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &[]
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut []
        }

        fn with_entries<F>(&mut self, _: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {}
    }

    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let result = map.try_reserve_exact(10);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "allocation failed")]
fn test_try_reserve_exact_failure_on_capacity() {
    struct TestEntries;

    impl Entries for TestEntries {
        type Entry = Bucket<usize, usize>;

        fn into_entries(self) -> Vec<Self::Entry> {
            vec![Bucket { hash: HashValue::new(), key: 1, value: 1 }]
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &[]
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut []
        }

        fn with_entries<F>(&mut self, _: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {}
    }

    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1);
    let result = map.try_reserve_exact(usize::MAX);
    assert!(result.is_err());
}

#[test]
fn test_try_reserve_exact_no_op() {
    struct TestEntries;

    impl Entries for TestEntries {
        type Entry = Bucket<usize, usize>;

        fn into_entries(self) -> Vec<Self::Entry> {
            vec![Bucket { hash: HashValue::new(), key: 1, value: 1 }]
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &[]
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut []
        }

        fn with_entries<F>(&mut self, _: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {}
    }

    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    let result = map.try_reserve_exact(0);
    assert!(result.is_ok());
}

