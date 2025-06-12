// Answer 0

#[test]
fn test_get_mut_valid_index() {
    struct Entry {
        value: i32,
    }

    struct EntryMap {
        entries: Vec<Entry>,
        index: usize,
    }

    impl EntryMap {
        fn new(entries: Vec<Entry>, index: usize) -> Self {
            EntryMap { entries, index }
        }

        fn index(&self) -> usize {
            self.index
        }

        fn get_mut(&mut self) -> &mut i32 {
            let index = self.index();
            &mut self.entries[index].value
        }
    }

    // Initialize with a valid index
    let mut map = EntryMap::new(vec![Entry { value: 1 }, Entry { value: 2 }], 1);
    let value = map.get_mut();
    *value += 5;
    assert_eq!(*value, 7);
}

#[test]
#[should_panic]
fn test_get_mut_out_of_bounds_index() {
    struct Entry {
        value: i32,
    }

    struct EntryMap {
        entries: Vec<Entry>,
        index: usize,
    }

    impl EntryMap {
        fn new(entries: Vec<Entry>, index: usize) -> Self {
            EntryMap { entries, index }
        }

        fn index(&self) -> usize {
            self.index
        }

        fn get_mut(&mut self) -> &mut i32 {
            let index = self.index();
            &mut self.entries[index].value
        }
    }

    // Initialize with an out-of-bounds index
    let mut map = EntryMap::new(vec![Entry { value: 1 }, Entry { value: 2 }], 2);
    map.get_mut(); // This should panic due to out-of-bounds access
}

