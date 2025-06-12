// Answer 0

#[derive(Debug)]
struct Entry<V> {
    value: V,
}

struct RawEntryMut<V> {
    entries: Vec<Entry<V>>,
    index: usize,
}

impl<V> RawEntryMut<V> {
    fn new(entries: Vec<Entry<V>>, index: usize) -> Self {
        Self { entries, index }
    }

    fn index(&self) -> usize {
        self.index
    }

    pub fn get_mut(&mut self) -> &mut V {
        let index = self.index();
        &mut self.entries[index].value
    }
}

#[test]
fn test_get_mut_valid_index() {
    let mut raw_entry = RawEntryMut::new(vec![Entry { value: 42 }, Entry { value: 43 }], 1);
    let value = raw_entry.get_mut();
    *value += 1;
    assert_eq!(*value, 44);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_get_mut_out_of_bounds_index() {
    let raw_entry = RawEntryMut::new(vec![Entry { value: 42 }], 1); // Index out of bounds
    let _ = raw_entry.get_mut();
} 

#[test]
fn test_get_mut_first_entry() {
    let mut raw_entry = RawEntryMut::new(vec![Entry { value: 10 }, Entry { value: 20 }], 0);
    let value = raw_entry.get_mut();
    *value += 5;
    assert_eq!(*value, 15);
}

#[test]
fn test_get_mut_single_entry() {
    let mut raw_entry = RawEntryMut::new(vec![Entry { value: 100 }], 0);
    let value = raw_entry.get_mut();
    *value *= 2;
    assert_eq!(*value, 200);
}

