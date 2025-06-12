// Answer 0

#[test]
fn test_value_iter_mut_with_valid_index() {
    struct TestEntry<T> {
        links: Option<Links<T>>,
    }

    struct Links<T> {
        tail: Vec<T>,
    }

    struct TestMap<T> {
        entries: Vec<TestEntry<T>>,
    }

    struct ValueIterMut<'a, T> {
        map: *mut TestMap<T>,
        index: usize,
        front: Option<Cursor<T>>,
        back: Option<Cursor<T>>,
        lt: std::marker::PhantomData<&'a T>,
    }

    enum Cursor<T> {
        Values(Vec<T>),
        Head,
    }

    impl<T> TestMap<T> {
        fn value_iter_mut(&mut self, idx: usize) -> ValueIterMut<'_, T> {
            use self::Cursor::*;

            let back = {
                let entry = &self.entries[idx];

                entry.links.as_ref().map(|l| Values(l.tail.clone())).unwrap_or(Head)
            };

            ValueIterMut {
                map: self as *mut _,
                index: idx,
                front: Some(Head),
                back: Some(back),
                lt: std::marker::PhantomData,
            }
        }
    }

    let entry1 = TestEntry {
        links: Some(Links { tail: vec![1, 2, 3] }),
    };
    let entry2 = TestEntry {
        links: None,
    };
    let mut test_map = TestMap {
        entries: vec![entry1, entry2],
    };

    let mut iter = test_map.value_iter_mut(0);
    assert!(matches!(iter.back, Some(Cursor::Values(ref v)) if v == &vec![1, 2, 3]));

    let mut iter2 = test_map.value_iter_mut(1);
    assert!(matches!(iter2.back, Some(Cursor::Head)));
}

#[test]
#[should_panic]
fn test_value_iter_mut_with_invalid_index() {
    struct TestEntry<T> {
        links: Option<Links<T>>,
    }

    struct Links<T> {
        tail: Vec<T>,
    }

    struct TestMap<T> {
        entries: Vec<TestEntry<T>>,
    }

    struct ValueIterMut<'a, T> {
        map: *mut TestMap<T>,
        index: usize,
        front: Option<Cursor<T>>,
        back: Option<Cursor<T>>,
        lt: std::marker::PhantomData<&'a T>,
    }

    enum Cursor<T> {
        Values(Vec<T>),
        Head,
    }

    impl<T> TestMap<T> {
        fn value_iter_mut(&mut self, idx: usize) -> ValueIterMut<'_, T> {
            use self::Cursor::*;

            let back = {
                let entry = &self.entries[idx];

                entry.links.as_ref().map(|l| Values(l.tail.clone())).unwrap_or(Head)
            };

            ValueIterMut {
                map: self as *mut _,
                index: idx,
                front: Some(Head),
                back: Some(back),
                lt: std::marker::PhantomData,
            }
        }
    }

    let entry = TestEntry {
        links: Some(Links { tail: vec![1, 2, 3] }),
    };
    let test_map = TestMap {
        entries: vec![entry],
    };

    // Accessing index 1 should panic since there's only one entry (index 0)
    let _ = test_map.value_iter_mut(1);
}

