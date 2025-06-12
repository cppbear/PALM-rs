// Answer 0

#[test]
fn test_remove_found_works_with_non_empty_map() {
    struct TestEntry {
        hash: usize,
        links: Option<Links>,
    }

    struct Links {
        next: usize,
        tail: usize,
    }

    struct TestMap<T> {
        indices: Vec<Pos>,
        entries: Vec<TestEntry>,
        extra_values: Vec<ExtraValue>,
        mask: usize,
    }

    struct Pos {
        index: Option<usize>,
        hash: usize,
    }

    struct ExtraValue {
        next: Link,
        prev: Link,
    }

    enum Link {
        Entry(usize),
        None,
    }

    impl Pos {
        fn none() -> Self {
            Pos { index: None, hash: 0 }
        }

        fn new(index: usize, hash: usize) -> Self {
            Pos { index: Some(index), hash }
        }
    }

    let mut test_map = TestMap {
        indices: vec![Pos::new(0, 1), Pos::new(1, 2)],
        entries: vec![
            TestEntry { hash: 1, links: Some(Links { next: 1, tail: 0 }) },
            TestEntry { hash: 2, links: Some(Links { next: 0, tail: 1 }) },
        ],
        extra_values: vec![
            ExtraValue { next: Link::Entry(1), prev: Link::None },
            ExtraValue { next: Link::None, prev: Link::Entry(0) },
        ],
        mask: 1,
    };

    let entry = test_map.remove_found(0, 1);

    assert_eq!(entry.hash, 2);
    assert_eq!(test_map.entries.len(), 1);
    assert_eq!(test_map.indices.len(), 2);
    assert_eq!(test_map.indices[0].index, None);
}

#[test]
#[should_panic]
fn test_remove_found_panic_on_empty_map() {
    struct TestEntry {
        hash: usize,
        links: Option<Links>,
    }

    struct Links {}

    struct TestMap<T> {
        indices: Vec<Pos>,
        entries: Vec<TestEntry>,
        extra_values: Vec<ExtraValue>,
        mask: usize,
    }

    struct Pos {
        index: Option<usize>,
        hash: usize,
    }

    struct ExtraValue {
        next: Link,
        prev: Link,
    }

    enum Link {
        Entry(usize),
        None,
    }

    impl Pos {
        fn none() -> Self {
            Pos { index: None, hash: 0 }
        }
    }

    let mut test_map = TestMap {
        indices: vec![],
        entries: vec![],
        extra_values: vec![],
        mask: 0,
    };

    test_map.remove_found(0, 0);
}

