// Answer 0

#[derive(Default)]
struct ValueType {
    data: i32,
}

struct HashMap {
    map: Option<Vec<Entry>>,
    fill: usize,
    used: usize,
    mask: usize,
}

#[derive(Default)]
struct Entry {
    key: u32,
    value: ValueType,
}

impl HashMap {
    pub fn new(size: usize) -> Self {
        let mask = size.saturating_sub(1); // mask is size - 1
        let map = Some(vec![Entry::default(); size]);
        HashMap { map, fill: 0, used: 0, mask }
    }

    pub fn lookup(&self, key: u32) -> usize {
        // Simulate a simple hash lookup
        key as usize & self.mask
    }

    pub fn grow(&mut self, new_size: usize) {
        self.map = Some(vec![Entry::default(); new_size]);
        self.mask = new_size.saturating_sub(1);
    }

    pub fn allocate(&mut self) {
        // Init the map
        self.map = Some(vec![Entry::default(); 8]);
        self.mask = 7; // 8 - 1
    }

    pub fn get_mut(&mut self, key: u32) -> &mut ValueType {
        if self.map.is_none() {
            self.allocate();
        }
        let mut i = self.lookup(key);
        if self.map.as_ref().expect("map should have been created above")[i].value == Default::default() {
            self.fill += 1;
            if self.fill * 3 >= (self.mask + 1) * 2 {
                self.grow((self.used + 1) * 2);
                i = self.lookup(key);
            }
            self.used += 1;
        }
        let elem = &mut self.map.as_mut().expect("map should have been created above")[i];
        elem.key = key;
        &mut elem.value
    }
}

#[test]
fn test_get_mut_with_allocated_map() {
    let mut hashmap = HashMap::new(8);
    let value_ref = hashmap.get_mut(1);
    value_ref.data = 42;
    assert_eq!(value_ref.data, 42);
}

#[test]
fn test_get_mut_fills_and_grows() {
    let mut hashmap = HashMap::new(4); // small initial size for growth
    hashmap.get_mut(1);
    hashmap.get_mut(2);
    hashmap.get_mut(3);
    let value_ref = hashmap.get_mut(4); // This should cause a growth
    value_ref.data = 84;
    assert_eq!(value_ref.data, 84);
} 

#[test]
fn test_get_mut_default_value() {
    let mut hashmap = HashMap::new(8);
    let value_ref = hashmap.get_mut(1);
    assert_eq!(value_ref.data, 0); // default value of ValueType
    value_ref.data = 100;
    assert_eq!(hashmap.get_mut(1).data, 100);
} 

#[test]
#[should_panic]
fn test_get_mut_access_panics_if_index_out_of_bounds() {
    let mut hashmap = HashMap::new(4);
    hashmap.get_mut(1);
    hashmap.get_mut(2);
    hashmap.get_mut(3);
    hashmap.get_mut(4); // triggers growth
    hashmap.fill = 2; // manually set fill to trigger panic on certain accesses
    hashmap.get_mut(5); // This may panic if the internal logic does not handle it correctly
}

