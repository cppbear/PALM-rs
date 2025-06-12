// Answer 0

#[test]
fn test_pop_empty() {
    struct TestMap<K, V> {
        core: IndexMapCore<K, V>
    }

    impl<K, V> TestMap<K, V> {
        pub fn new() -> Self {
            TestMap {
                core: IndexMapCore::new(),
            }
        }

        pub fn pop(&mut self) -> Option<(K, V)> {
            self.core.pop()
        }
    }

    let mut map = TestMap::<i32, i32>::new();
    assert_eq!(map.pop(), None);
}

#[test]
fn test_pop_single_element() {
    struct TestMap<K, V> {
        core: IndexMapCore<K, V>
    }

    impl<K, V> TestMap<K, V> {
        pub fn new() -> Self {
            TestMap {
                core: IndexMapCore::with_capacity(1),
            }
        }

        pub fn push(&mut self, key: K, value: V) {
            self.core.entries.push(Bucket { key, value });
        }

        pub fn pop(&mut self) -> Option<(K, V)> {
            self.core.pop()
        }
    }

    let mut map = TestMap::<i32, i32>::new();
    map.push(1, 10);
    assert_eq!(map.pop(), Some((1, 10)));
    assert_eq!(map.pop(), None);
}

#[test]
fn test_pop_multiple_elements() {
    struct TestMap<K, V> {
        core: IndexMapCore<K, V>
    }

    impl<K, V> TestMap<K, V> {
        pub fn new() -> Self {
            TestMap {
                core: IndexMapCore::with_capacity(3),
            }
        }

        pub fn push(&mut self, key: K, value: V) {
            self.core.entries.push(Bucket { key, value });
        }

        pub fn pop(&mut self) -> Option<(K, V)> {
            self.core.pop()
        }
    }

    let mut map = TestMap::<i32, i32>::new();
    map.push(1, 10);
    map.push(2, 20);
    map.push(3, 30);
    
    assert_eq!(map.pop(), Some((3, 30)));
    assert_eq!(map.pop(), Some((2, 20)));
    assert_eq!(map.pop(), Some((1, 10)));
    assert_eq!(map.pop(), None);
}

