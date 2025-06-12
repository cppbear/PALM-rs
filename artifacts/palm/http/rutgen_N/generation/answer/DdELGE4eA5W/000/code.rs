// Answer 0

#[test]
fn test_value_iter_with_some_index() {
    struct TestMap<T> {
        entries: Vec<Entry<T>>,
    }

    struct Entry<T> {
        links: Option<Link<T>>,
    }

    struct Link<T> {
        tail: T,
    }

    struct ValueIter<'a, T> {
        map: &'a TestMap<T>,
        index: usize,
        front: Option<Cursor>,
        back: Option<Cursor>,
    }

    enum Cursor {
        Head,
        Values(T),
    }

    impl<T> TestMap<T> {
        fn value_iter(&self, idx: Option<usize>) -> ValueIter<'_, T> {
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
        Entry { links: Some(Link { tail: 5 }) },
        Entry { links: None },
    ];
    
    let my_map = TestMap { entries };
    
    let iter = my_map.value_iter(Some(0));
    
    assert_eq!(iter.index, 0);
    if let Some(Cursor::Values(val)) = iter.back {
        assert_eq!(val, 5);
    } else {
        panic!("Expected back to be Values(5)");
    }
}

#[test]
fn test_value_iter_with_none_index() {
    struct TestMap<T> {
        entries: Vec<Entry<T>>,
    }

    struct Entry<T> {
        links: Option<Link<T>>,
    }

    struct Link<T> {
        tail: T,
    }

    struct ValueIter<'a, T> {
        map: &'a TestMap<T>,
        index: usize,
        front: Option<Cursor>,
        back: Option<Cursor>,
    }

    enum Cursor {
        Head,
        Values(T),
    }

    impl<T> TestMap<T> {
        fn value_iter(&self, idx: Option<usize>) -> ValueIter<'_, T> {
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
        Entry { links: Some(Link { tail: 5 }) },
        Entry { links: None },
    ];
    
    let my_map = TestMap { entries };
    
    let iter = my_map.value_iter(None);
    
    assert_eq!(iter.index, usize::MAX);
    assert!(iter.front.is_none());
    assert!(iter.back.is_none());
}

