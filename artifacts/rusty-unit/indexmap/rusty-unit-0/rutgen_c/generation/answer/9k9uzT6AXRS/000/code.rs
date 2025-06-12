// Answer 0

#[test]
fn test_split_splice_empty() {
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

        fn with_entries<F>(&mut self, _f: F) where F: FnOnce(&mut [Self::Entry]) {}
    }

    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let (spliced, drained) = map.split_splice(0..0);
    assert_eq!(spliced.len(), 0);
    assert_eq!(drained.len(), 0);
}

#[test]
fn test_split_splice_single_entry() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<usize, usize>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.entries
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.entries
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.entries
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut map = IndexMapCore {
        indices: Indices::new(),
        entries: TestEntries { entries: vec![Bucket { hash: HashValue::default(), key: 1, value: 2 }] },
    };
    let (spliced, mut drained) = map.split_splice(0..1);
    
    assert_eq!(spliced.len(), 0);
    assert_eq!(drained.next().unwrap().key, 1);
    assert_eq!(drained.next(), None);
}

#[test]
fn test_split_splice_multiple_entries() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<usize, usize>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.entries
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.entries
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.entries
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut map = IndexMapCore {
        indices: Indices::new(),
        entries: TestEntries { entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: 2 },
            Bucket { hash: HashValue::default(), key: 3, value: 4 },
            Bucket { hash: HashValue::default(), key: 5, value: 6 },
        ] },
    };
    
    let (spliced, mut drained) = map.split_splice(1..3);
    assert_eq!(spliced.len(), 1);
    assert_eq!(drained.next().unwrap().key, 3);
    assert_eq!(drained.next().unwrap().key, 5);
    assert_eq!(drained.next(), None);
}

