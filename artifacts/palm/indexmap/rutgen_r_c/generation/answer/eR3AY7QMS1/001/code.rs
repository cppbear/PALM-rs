// Answer 0

#[test]
fn test_shift_remove() {
    struct MockKey;
    struct MockValue(usize);

    struct MockIndexMapCore<'a> {
        entries: Vec<MockValue>,
        key_indices: Vec<usize>,
        _lifetime: std::marker::PhantomData<&'a ()>,
    }

    impl<'a> MockIndexMapCore<'a> {
        fn new() -> Self {
            Self { 
                entries: vec![],
                key_indices: vec![],
                _lifetime: std::marker::PhantomData,
            }
        }

        fn borrow_mut(&mut self) -> RefMut<'a, MockKey, MockValue> {
            RefMut {
                indices: &mut self.key_indices,
                entries: &mut self.entries,
            }
        }
        
        fn shift_remove_index(&mut self, index: usize) -> Option<(MockKey, MockValue)> {
            if index < self.entries.len() {
                let removed_value = self.entries.remove(index);
                self.key_indices.remove(index);
                Some((MockKey, removed_value))
            } else {
                None
            }
        }
    }

    impl<'a, K, V> IndexedEntry<'a, K, V> {
        fn new(map: &'a mut MockIndexMapCore<'a>, index: usize) -> Self {
            Self {
                map: map.borrow_mut(),
                index,
            }
        }
    }

    let mut map = MockIndexMapCore::new();
    map.entries.push(MockValue(1));
    map.entries.push(MockValue(2));
    map.entries.push(MockValue(3));

    let entry = IndexedEntry::new(&mut map, 1);
    let removed_value = entry.shift_remove();

    assert_eq!(removed_value.0 .0, 2); // Checking the value returned
    assert_eq!(map.entries.len(), 2); // Ensuring the size of the map is correct
    assert_eq!(map.entries[0].0, 1); // Check that the first value remains
    assert_eq!(map.entries[1].0, 3); // Check that the second value shifts correctly
}

#[test]
#[should_panic]
fn test_shift_remove_out_of_bounds() {
    struct MockKey;
    struct MockValue(usize);

    struct MockIndexMapCore<'a> {
        entries: Vec<MockValue>,
        key_indices: Vec<usize>,
        _lifetime: std::marker::PhantomData<&'a ()>,
    }

    impl<'a> MockIndexMapCore<'a> {
        fn new() -> Self {
            Self {
                entries: vec![],
                key_indices: vec![],
                _lifetime: std::marker::PhantomData,
            }
        }

        fn borrow_mut(&mut self) -> RefMut<'a, MockKey, MockValue> {
            RefMut {
                indices: &mut self.key_indices,
                entries: &mut self.entries,
            }
        }
        
        fn shift_remove_index(&mut self, index: usize) -> Option<(MockKey, MockValue)> {
            if index < self.entries.len() {
                let removed_value = self.entries.remove(index);
                self.key_indices.remove(index);
                Some((MockKey, removed_value))
            } else {
                None
            }
        }
    }

    impl<'a, K, V> IndexedEntry<'a, K, V> {
        fn new(map: &'a mut MockIndexMapCore<'a>, index: usize) -> Self {
            Self {
                map: map.borrow_mut(),
                index,
            }
        }
    }

    let mut map = MockIndexMapCore::new();
    map.entries.push(MockValue(1));
    map.entries.push(MockValue(2));

    let entry = IndexedEntry::new(&mut map, 5); // Out of bounds index
    let _ = entry.shift_remove(); // This should panic
}

