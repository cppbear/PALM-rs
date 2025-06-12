// Answer 0

#[test]
fn test_into_mut_valid_entry() {
    struct Entry<V> {
        value: V,
    }

    struct Map<V> {
        entries: Vec<Entry<V>>,
    }

    impl<V> Map<V> {
        fn new() -> Self {
            Map { entries: Vec::new() }
        }
        
        fn add_entry(&mut self, value: V) {
            self.entries.push(Entry { value });
        }

        fn into_mut(&mut self, index: usize) -> &mut V {
            let entry = &mut self.entries[index];
            &mut entry.value
        }
    }

    let mut map = Map::new();
    map.add_entry(10);
    map.add_entry(20);
    
    let value_ref = map.into_mut(1);
    *value_ref += 5;

    assert_eq!(map.entries[1].value, 25);
}

#[should_panic]
#[test]
fn test_into_mut_out_of_bounds() {
    struct Entry<V> {
        value: V,
    }

    struct Map<V> {
        entries: Vec<Entry<V>>,
    }

    impl<V> Map<V> {
        fn new() -> Self {
            Map { entries: Vec::new() }
        }
        
        fn add_entry(&mut self, value: V) {
            self.entries.push(Entry { value });
        }

        fn into_mut(&mut self, index: usize) -> &mut V {
            let entry = &mut self.entries[index];
            &mut entry.value
        }
    }

    let mut map = Map::new();
    map.add_entry(30);

    // This will panic since there is no entry at index 1
    let _value_ref = map.into_mut(1);
}

