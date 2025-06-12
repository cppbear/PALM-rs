// Answer 0

#[derive(Debug)]
struct Entry<V> {
    value: V,
}

struct Map<V> {
    entries: Vec<Entry<V>>,
}

impl<V> Map<V> {
    pub fn new() -> Self {
        Map { entries: Vec::new() }
    }

    pub fn insert(&mut self, value: V) {
        self.entries.push(Entry { value });
    }

    pub fn into_mut(&mut self, index: usize) -> &mut V {
        let index = index;
        &mut self.entries[index].value
    }
}

#[test]
fn test_into_mut() {
    let mut map: Map<i32> = Map::new();
    map.insert(10);
    map.insert(20);
    
    {
        let value = map.into_mut(0);
        *value += 5;
    }
    
    assert_eq!(map.entries[0].value, 15);
}

#[test]
#[should_panic]
fn test_into_mut_out_of_bounds() {
    let mut map: Map<i32> = Map::new();
    map.insert(10);
    
    let _value = map.into_mut(1); // This should panic due to out of bounds
}

