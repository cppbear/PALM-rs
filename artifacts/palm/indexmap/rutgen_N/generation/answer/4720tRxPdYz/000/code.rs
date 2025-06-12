// Answer 0

#[derive(Debug)]
struct DummyMap {
    data: Vec<(i32, String)>,
}

impl DummyMap {
    fn swap_remove_index(&mut self, index: usize) -> Option<(i32, String)> {
        if index < self.data.len() {
            let last_index = self.data.len() - 1;
            self.data.swap(index, last_index);
            Some(self.data.remove(last_index))
        } else {
            None
        }
    }
}

struct Entry<'a> {
    map: &'a mut DummyMap,
    index: usize,
}

impl<'a> Entry<'a> {
    fn swap_remove_entry(self) -> (i32, String) {
        self.map.swap_remove_index(self.index).unwrap()
    }
}

#[test]
fn test_swap_remove_entry_valid() {
    let mut map = DummyMap { data: vec![(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())] };
    let entry = Entry { map: &mut map, index: 1 };
    let result = entry.swap_remove_entry();
    assert_eq!(result, (2, "two".to_string()));
    assert_eq!(map.data, vec![(1, "one".to_string()), (3, "three".to_string())]);
}

#[test]
fn test_swap_remove_entry_last() {
    let mut map = DummyMap { data: vec![(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())] };
    let entry = Entry { map: &mut map, index: 2 };
    let result = entry.swap_remove_entry();
    assert_eq!(result, (3, "three".to_string()));
    assert_eq!(map.data, vec![(1, "one".to_string()), (2, "two".to_string())]);
}

#[test]
#[should_panic]
fn test_swap_remove_entry_invalid_index() {
    let mut map = DummyMap { data: vec![(1, "one".to_string()), (2, "two".to_string())] };
    let entry = Entry { map: &mut map, index: 2 };
    entry.swap_remove_entry(); // This should panic due to invalid index.
}

