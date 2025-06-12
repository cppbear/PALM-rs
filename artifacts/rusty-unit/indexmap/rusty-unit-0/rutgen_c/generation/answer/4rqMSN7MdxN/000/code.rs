// Answer 0

#[test]
fn test_sort_unstable_by() {
    struct TestMap {
        core: IndexMapCore<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                core: IndexMapCore {
                    indices: Indices::new(),
                    entries: Entries::new(),
                },
            }
        }

        fn as_mut_slice(&mut self) -> &mut Slice<i32, String> {
            self.core.entries.as_mut_slice()
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Entry<i32, String>]),
        {
            f(self.as_mut_slice());
        }
    }

    let mut map = TestMap::new();
    map.core.entries.push(Entry::new(3, "three".to_string()));
    map.core.entries.push(Entry::new(1, "one".to_string()));
    map.core.entries.push(Entry::new(2, "two".to_string()));

    map.sort_unstable_by(|k1, v1, k2, v2| k1.cmp(k2));

    assert_eq!(map.core.entries[0].key, 1);
    assert_eq!(map.core.entries[1].key, 2);
    assert_eq!(map.core.entries[2].key, 3);
}

#[test]
fn test_sort_unstable_by_reverse() {
    struct TestMap {
        core: IndexMapCore<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                core: IndexMapCore {
                    indices: Indices::new(),
                    entries: Entries::new(),
                },
            }
        }

        fn as_mut_slice(&mut self) -> &mut Slice<i32, String> {
            self.core.entries.as_mut_slice()
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Entry<i32, String>]),
        {
            f(self.as_mut_slice());
        }
    }

    let mut map = TestMap::new();
    map.core.entries.push(Entry::new(1, "one".to_string()));
    map.core.entries.push(Entry::new(3, "three".to_string()));
    map.core.entries.push(Entry::new(2, "two".to_string()));

    map.sort_unstable_by(|k1, v1, k2, v2| k2.cmp(k1));

    assert_eq!(map.core.entries[0].key, 3);
    assert_eq!(map.core.entries[1].key, 2);
    assert_eq!(map.core.entries[2].key, 1);
}

