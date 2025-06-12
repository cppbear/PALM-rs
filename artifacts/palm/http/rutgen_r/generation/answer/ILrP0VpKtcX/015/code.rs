// Answer 0

#[test]
fn test_remove_found_valid() {
    struct TestEntry {
        hash: u64,
        links: Option<Links>,
    }

    struct Links {
        next: usize,
        tail: usize,
    }

    struct TestMap {
        entries: Vec<TestEntry>,
        indices: Vec<Pos>,
        extra_values: Vec<Link>,
        mask: usize,
    }

    struct Pos {
        index: usize,
        hash: u64,
    }

    struct Link {
        prev: Entry,
        next: Entry,
    }

    impl Pos {
        fn none() -> Self {
            Self { index: 0, hash: 0 }
        }

        fn new(index: usize, hash: u64) -> Self {
            Self { index, hash }
        }
        
        fn resolve(&self) -> Option<(usize, u64)> {
            Some((self.index, self.hash))
        }
    }

    let mut map = TestMap {
        entries: vec![
            TestEntry { hash: 1, links: Some(Links { next: 1, tail: 2 }) },
            TestEntry { hash: 1, links: Some(Links { next: 0, tail: 1 }) },
        ],
        indices: vec![Pos::new(0, 1), Pos::new(1, 1)],
        extra_values: vec![
            Link { prev: Entry::None, next: Entry::Entry(1) },
            Link { prev: Entry::Entry(0), next: Entry::Entry(0) },
            Link { prev: Entry::Entry(1), next: Entry::None },
        ],
        mask: 1,
    };

    let probe = 0;
    let found = 0;

    let entry = map.remove_found(probe, found);
    assert_eq!(entry.hash, 1);
}

#[test]
#[should_panic]
fn test_remove_found_empty_entries() {
    struct TestEntry {
        hash: u64,
        links: Option<Links>,
    }

    struct Links {
        next: usize,
        tail: usize,
    }

    struct TestMap {
        entries: Vec<TestEntry>,
        indices: Vec<Pos>,
        extra_values: Vec<Link>,
        mask: usize,
    }

    struct Pos {
        index: usize,
        hash: u64,
    }

    struct Link {
        prev: Entry,
        next: Entry,
    }

    impl Pos {
        fn none() -> Self {
            Self { index: 0, hash: 0 }
        }

        fn new(index: usize, hash: u64) -> Self {
            Self { index, hash }
        }
        
        fn resolve(&self) -> Option<(usize, u64)> {
            Some((self.index, self.hash))
        }
    }

    let mut map = TestMap {
        entries: vec![],
        indices: vec![],
        extra_values: vec![],
        mask: 1,
    };

    let probe = 0;
    let found = 0;

    map.remove_found(probe, found);
}

#[test]
#[should_panic]
fn test_remove_found_invalid_probes() {
    struct TestEntry {
        hash: u64,
        links: Option<Links>,
    }

    struct Links {
        next: usize,
        tail: usize,
    }

    struct TestMap {
        entries: Vec<TestEntry>,
        indices: Vec<Pos>,
        extra_values: Vec<Link>,
        mask: usize,
    }

    struct Pos {
        index: usize,
        hash: u64,
    }

    struct Link {
        prev: Entry,
        next: Entry,
    }

    impl Pos {
        fn none() -> Self {
            Self { index: 0, hash: 0 }
        }

        fn new(index: usize, hash: u64) -> Self {
            Self { index, hash }
        }
        
        fn resolve(&self) -> Option<(usize, u64)> {
            Some((self.index, self.hash))
        }
    }

    let mut map = TestMap {
        entries: vec![
            TestEntry { hash: 1, links: Some(Links { next: 1, tail: 2 }) },
        ],
        indices: vec![Pos::new(0, 1)],
        extra_values: vec![
            Link { prev: Entry::None, next: Entry::Entry(1) },
        ],
        mask: 1,
    };

    let probe = 2; // Invalid probe
    let found = 0;

    map.remove_found(probe, found);
}

