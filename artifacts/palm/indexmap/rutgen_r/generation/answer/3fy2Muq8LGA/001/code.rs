// Answer 0

#[test]
fn test_into_mut_valid_case() {
    struct Entry<'a, V> {
        value: V,
    }

    struct Map<'a, V> {
        entries: Vec<Entry<'a, V>>,
    }

    impl<'a, V> Map<'a, V> {
        pub fn into_mut(&mut self, index: usize) -> &'a mut V {
            let index = index;
            &mut self.entries[index].value
        }

        pub fn new() -> Self {
            Self { entries: Vec::new() }
        }

        pub fn add_entry(&mut self, value: V) {
            self.entries.push(Entry { value });
        }
    }

    let mut map: Map<i32> = Map::new();
    map.add_entry(10);
    map.add_entry(20);

    let value_ref: &mut i32 = map.into_mut(0);
    *value_ref = 30; // Should modify the first entry to 30.

    assert_eq!(map.entries[0].value, 30);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_into_mut_out_of_bounds() {
    struct Entry<'a, V> {
        value: V,
    }

    struct Map<'a, V> {
        entries: Vec<Entry<'a, V>>,
    }

    impl<'a, V> Map<'a, V> {
        pub fn into_mut(&mut self, index: usize) -> &'a mut V {
            let index = index;
            &mut self.entries[index].value
        }

        pub fn new() -> Self {
            Self { entries: Vec::new() }
        }

        pub fn add_entry(&mut self, value: V) {
            self.entries.push(Entry { value });
        }
    }

    let mut map: Map<i32> = Map::new();
    map.add_entry(10);
    map.add_entry(20);
    
    // Attempting to access an index that is out of bounds.
    let _value_ref: &mut i32 = map.into_mut(2); // This should panic
}

