// Answer 0

#[test]
fn test_swap_remove() {
    struct TestMap {
        entries: Vec<(usize, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![] }
        }

        fn borrow_mut(&mut self) -> RefMut<usize, String> {
            RefMut {
                indices: &mut vec![0, 1, 2, 3, 4],
                entries: &mut self.entries,
            }
        }

        fn swap_remove_index(&mut self, index: usize) -> Option<(usize, String)> {
            if index < self.entries.len() {
                let last_index = self.entries.len() - 1;
                self.entries.swap(index, last_index);
                Some(self.entries.pop().unwrap())
            } else {
                None
            }
        }
    }

    let mut map = TestMap::new();
    map.entries.push((0, String::from("zero")));
    map.entries.push((1, String::from("one")));
    map.entries.push((2, String::from("two")));
    
    let indexed_entry = IndexedEntry::new(&mut map, 1);
    
    let value = indexed_entry.swap_remove();
    
    assert_eq!(value, String::from("one"));
    assert_eq!(map.entries.len(), 2);
    assert!(!map.entries.iter().any(|&(k, _)| k == 1));
}

#[test]
#[should_panic]
fn test_swap_remove_out_of_bounds() {
    struct TestMap {
        entries: Vec<(usize, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![] }
        }

        fn borrow_mut(&mut self) -> RefMut<usize, String> {
            RefMut {
                indices: &mut vec![0, 1, 2, 3, 4],
                entries: &mut self.entries,
            }
        }

        fn swap_remove_index(&mut self, index: usize) -> Option<(usize, String)> {
            if index < self.entries.len() {
                let last_index = self.entries.len() - 1;
                self.entries.swap(index, last_index);
                Some(self.entries.pop().unwrap())
            } else {
                None
            }
        }
    }

    let mut map = TestMap::new();
    map.entries.push((0, String::from("zero")));
    
    let indexed_entry = IndexedEntry::new(&mut map, 1);
    
    indexed_entry.swap_remove(); // This should panic due to out of bounds access
}

