// Answer 0

#[test]
fn test_value_iter_mut_valid_index() {
    struct Entry {
        links: Option<Links>,
    }

    struct Links {
        tail: usize,
    }

    struct HeaderMap<T> {
        entries: Vec<Entry>,
    }

    struct ValueIterMut<'a, T> {
        map: *mut HeaderMap<T>,
        index: usize,
        front: Option<Head>,
        back: Option<Cursor>,
        lt: PhantomData<&'a T>,
    }

    enum Head {}
    enum Cursor {
        Values(usize),
        Head,
    }

    impl<T> HeaderMap<T> {
        fn value_iter_mut(&mut self, idx: usize) -> ValueIterMut<'_, T> {
            use self::Cursor::*;

            let back = {
                let entry = &self.entries[idx];

                entry.links.map(|l| Values(l.tail)).unwrap_or(Head)
            };

            ValueIterMut {
                map: self as *mut _,
                index: idx,
                front: Some(Head),
                back: Some(back),
                lt: PhantomData,
            }
        }
    }

    // Setup Headers and entries
    let entries = vec![
        Entry { links: Some(Links { tail: 0 }) },
        Entry { links: None },
    ];

    let mut header_map = HeaderMap { entries };

    // Test valid index (should not panic)
    let iter = header_map.value_iter_mut(0);
    assert!(iter.index == 0);
    assert!(matches!(iter.back, Some(Cursor::Values(0))));
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_value_iter_mut_invalid_index() {
    struct Entry {
        links: Option<Links>,
    }

    struct Links {
        tail: usize,
    }

    struct HeaderMap<T> {
        entries: Vec<Entry>,
    }

    struct ValueIterMut<'a, T> {
        map: *mut HeaderMap<T>,
        index: usize,
        front: Option<Head>,
        back: Option<Cursor>,
        lt: PhantomData<&'a T>,
    }

    enum Head {}
    enum Cursor {
        Values(usize),
        Head,
    }

    impl<T> HeaderMap<T> {
        fn value_iter_mut(&mut self, idx: usize) -> ValueIterMut<'_, T> {
            use self::Cursor::*;

            let back = {
                let entry = &self.entries[idx];

                entry.links.map(|l| Values(l.tail)).unwrap_or(Head)
            };

            ValueIterMut {
                map: self as *mut _,
                index: idx,
                front: Some(Head),
                back: Some(back),
                lt: PhantomData,
            }
        }
    }

    // Setup Headers with one entry
    let entries = vec![Entry { links: Some(Links { tail: 0 }) }];
    let mut header_map = HeaderMap { entries };

    // Test invalid index (should panic)
    let _iter = header_map.value_iter_mut(1); // Out of bounds
}

