// Answer 0

#[derive(Debug)]
struct TestEntry {
    value: usize,
}

impl TestEntry {
    fn index(&self) -> usize {
        self.value
    }
}

#[derive(Debug)]
struct TestMap {
    entries: Vec<TestEntry>,
}

impl TestMap {
    fn new() -> Self {
        TestMap { entries: Vec::new() }
    }
    
    fn move_index(&mut self, from: usize, to: usize) {
        if to > self.entries.len() {
            panic!("Index out of bounds");
        }
        let entry = self.entries.remove(from);
        self.entries.insert(to, entry);
    }
}

impl TestEntry {
    fn move_index(self: Self, to: usize) {
        let index = self.index();
        let map = &mut TestMap::new(); // Assuming there’s a mutable reference to the map.
        map.move_index(index, to);
    }
}

#[test]
fn test_move_index_within_bounds() {
    let mut map = TestMap::new();
    map.entries.push(TestEntry { value: 0 });
    map.entries.push(TestEntry { value: 1 });
    map.entries.push(TestEntry { value: 2 });
    
    let entry = map.entries.get(1).cloned().unwrap();
    entry.move_index(0);
    
    assert_eq!(map.entries[0].value, 1);
    assert_eq!(map.entries[1].value, 0);
    assert_eq!(map.entries[2].value, 2);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_move_index_out_of_bounds() {
    let mut map = TestMap::new();
    map.entries.push(TestEntry { value: 0 });
    
    let entry = map.entries.get(0).cloned().unwrap();
    entry.move_index(2); // This should panic, as index 2 is out of bounds.
}

#[test]
fn test_move_index_downward() {
    let mut map = TestMap::new();
    map.entries.push(TestEntry { value: 0 });
    map.entries.push(TestEntry { value: 1 });
    map.entries.push(TestEntry { value: 2 });
    
    let entry = map.entries.get(2).cloned().unwrap();
    entry.move_index(1);
    
    assert_eq!(map.entries[1].value, 2);
    assert_eq!(map.entries[2].value, 1);
}

