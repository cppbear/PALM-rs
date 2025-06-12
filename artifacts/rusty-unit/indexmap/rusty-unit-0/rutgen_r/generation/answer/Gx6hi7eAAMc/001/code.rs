// Answer 0

#[test]
fn test_into_mut_valid_index() {
    struct MockMap<V> {
        entries: Vec<Entry<V>>,
    }

    struct Entry<V> {
        value: V,
    }

    struct EntryWrapper<'a, V> {
        map: &'a mut MockMap<V>,
        index: usize,
    }

    impl<'a, V> EntryWrapper<'a, V> {
        pub fn into_mut(self) -> &'a mut V {
            &mut self.map.entries[self.index].value
        }
    }

    let mut map = MockMap {
        entries: vec![Entry { value: 42 }],
    };
    let index = 0;
    let mut entry_wrapper = EntryWrapper { map: &mut map, index };

    let value: &mut i32 = entry_wrapper.into_mut();
    *value += 1; // Modify the value

    assert_eq!(*value, 43);
}

#[should_panic]
#[test]
fn test_into_mut_invalid_index() {
    struct MockMap<V> {
        entries: Vec<Entry<V>>,
    }

    struct Entry<V> {
        value: V,
    }

    struct EntryWrapper<'a, V> {
        map: &'a mut MockMap<V>,
        index: usize,
    }

    impl<'a, V> EntryWrapper<'a, V> {
        pub fn into_mut(self) -> &'a mut V {
            &mut self.map.entries[self.index].value
        }
    }

    let mut map = MockMap {
        entries: Vec::new(), // Empty map, no entries
    };
    let index = 0; // Invalid index
    let entry_wrapper = EntryWrapper { map: &mut map, index };

    let _value: &mut i32 = entry_wrapper.into_mut(); // This should panic
}

