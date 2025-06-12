// Answer 0

#[test]
fn test_value_iter_with_valid_index() {
    struct MockMap {
        entries: Vec<MockEntry>,
    }

    struct MockEntry {
        links: Option<MockLinks>,
    }

    struct MockLinks {
        tail: usize,
    }

    struct ValueIter<'a, T> {
        map: &'a MockMap,
        index: usize,
        front: Option<Cursor>,
        back: Option<Cursor>,
    }

    enum Cursor {
        Head,
        Values(usize),
    }

    impl MockMap {
        fn value_iter(&self, idx: Option<usize>) -> ValueIter<'_, MockEntry> {
            use self::Cursor::*;

            if let Some(idx) = idx {
                let back = {
                    let entry = &self.entries[idx];

                    entry.links.map(|l| Values(l.tail)).unwrap_or(Head)
                };

                ValueIter {
                    map: self,
                    index: idx,
                    front: Some(Head),
                    back: Some(back),
                }
            } else {
                ValueIter {
                    map: self,
                    index: usize::MAX,
                    front: None,
                    back: None,
                }
            }
        }
    }

    let entries = vec![
        MockEntry { links: Some(MockLinks { tail: 1 }) },
        MockEntry { links: None },
        MockEntry { links: Some(MockLinks { tail: 2 }) },
    ];
    let map = MockMap { entries };

    let iter = map.value_iter(Some(0));
    assert_eq!(iter.index, 0);
    assert!(matches!(iter.front, Some(Cursor::Head)));
    assert!(matches!(iter.back, Some(Cursor::Values(1))));

    let iter = map.value_iter(Some(1));
    assert_eq!(iter.index, 1);
    assert!(matches!(iter.front, Some(Cursor::Head)));
    assert!(matches!(iter.back, Some(Cursor::Head)));

    let iter = map.value_iter(Some(2));
    assert_eq!(iter.index, 2);
    assert!(matches!(iter.front, Some(Cursor::Head)));
    assert!(matches!(iter.back, Some(Cursor::Values(2))));
}

#[should_panic]
#[test]
fn test_value_iter_with_out_of_bounds_index() {
    struct MockMap {
        entries: Vec<MockEntry>,
    }

    struct MockEntry {
        links: Option<MockLinks>,
    }

    struct MockLinks {
        tail: usize,
    }

    struct ValueIter<'a, T> {
        map: &'a MockMap,
        index: usize,
        front: Option<Cursor>,
        back: Option<Cursor>,
    }

    enum Cursor {
        Head,
        Values(usize),
    }

    impl MockMap {
        fn value_iter(&self, idx: Option<usize>) -> ValueIter<'_, MockEntry> {
            use self::Cursor::*;

            if let Some(idx) = idx {
                let back = {
                    let entry = &self.entries[idx];

                    entry.links.map(|l| Values(l.tail)).unwrap_or(Head)
                };

                ValueIter {
                    map: self,
                    index: idx,
                    front: Some(Head),
                    back: Some(back),
                }
            } else {
                ValueIter {
                    map: self,
                    index: usize::MAX,
                    front: None,
                    back: None,
                }
            }
        }
    }

    let map = MockMap { entries: vec![] };
    let _ = map.value_iter(Some(0)); // This should trigger a panic due to out of bounds access
}

