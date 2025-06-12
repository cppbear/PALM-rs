// Answer 0

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_insert_before_panic_index_out_of_bounds() {
    struct TestMap {
        entries: Vec<(char, ())>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![] }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn insert_before(&mut self, index: usize, key: char, value: ()) -> (usize, Option<()>) {
            let len = self.len();
            assert!(
                index <= len,
                "index out of bounds: the len is {len} but the index is {index}. Expected index <= len"
            );
            self.entries.insert(index, (key, value));
            (index, None)
        }
    }

    let mut test_map = TestMap::new();
    test_map.insert_before(1, 'a', ()); // This should not panic as the map is empty

    // Now this will cause panic since the index is out of bounds (index 2 for length 1)
    test_map.insert_before(2, 'b', ());
}

#[test]
fn test_insert_before_at_end() {
    struct TestMap {
        entries: Vec<(char, ())>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![] }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn insert_before(&mut self, index: usize, key: char, value: ()) -> (usize, Option<()>) {
            let len = self.len();
            assert!(
                index <= len,
                "index out of bounds: the len is {len} but the index is {index}. Expected index <= len"
            );
            self.entries.insert(index, (key, value));
            (index, None)
        }
    }

    let mut test_map = TestMap::new();
    test_map.insert_before(0, 'a', ());
    let (index, old_value) = test_map.insert_before(1, 'b', ());
    
    assert_eq!(index, 1);
    assert_eq!(old_value, None);
}

#[test]
fn test_insert_before_shift_existing_key() {
    struct TestMap {
        entries: Vec<(char, ())>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![] }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn insert_before(&mut self, index: usize, key: char, value: ()) -> (usize, Option<()>) {
            let len = self.len();
            assert!(
                index <= len,
                "index out of bounds: the len is {len} but the index is {index}. Expected index <= len"
            );

            if let Some(pos) = self.entries.iter().position(|&(k, _)| k == key) {
                let old_value = self.entries[pos].1;
                if index > pos {
                    index -= 1;
                }
                self.entries.remove(pos);
                self.entries.insert(index, (key, value));
                return (index, Some(old_value));
            } 

            self.entries.insert(index, (key, value));
            (index, None)
        }
    }

    let mut test_map = TestMap::new();
    test_map.insert_before(0, 'a', ());
    let (index, old_value) = test_map.insert_before(0, 'a', ());
    
    assert_eq!(index, 0);
    assert_eq!(old_value, Some(()));
}

