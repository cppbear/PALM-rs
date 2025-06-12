// Answer 0

#[derive(Default)]
struct MapEntry {
    key: u32,
    value: usize,
}

struct HashMap {
    map: Option<Vec<MapEntry>>,
    mask: u32,
}

impl HashMap {
    fn new(mask: u32) -> Self {
        HashMap {
            map: Some(vec![MapEntry::default(); (mask + 1) as usize]),
            mask,
        }
    }

    fn lookup(&self, key: u32) -> usize {
        let hash = key;
        let mut i = hash as usize & self.mask as usize;
        let map = self.map.as_ref().expect("callers have to ensure map is allocated");

        if map[i].value == Default::default() || map[i].key == key {
            return i;
        }

        let mut perturb = key;
        loop {
            i = (i * 5 + perturb as usize + 1) & self.mask as usize;
            if map[i].value == Default::default() || map[i].key == key {
                return i;
            }
            perturb >>= 5;
        }
    }
}

#[test]
fn test_lookup_with_default_value() {
    let mut hashmap = HashMap::new(15);
    let key = 5;
    hashmap.map.as_mut().unwrap()[key as usize].value = 1; // Set a value
    hashmap.map.as_mut().unwrap()[key as usize].key = key; // Set a matching key

    let index = hashmap.lookup(key);
    assert_eq!(index, key as usize);
}

#[test]
fn test_lookup_with_collision() {
    let mut hashmap = HashMap::new(15);
    let key = 3;
    let collision_index = (5 * (key as usize + 1)) & hashmap.mask as usize;
    hashmap.map.as_mut().unwrap()[collision_index].value = 2; // Set a value at collision
    hashmap.map.as_mut().unwrap()[collision_index].key = 4; // Different key

    hashmap.map.as_mut().unwrap()[key as usize].value = 1; // Set a value
    hashmap.map.as_mut().unwrap()[key as usize].key = key; // Set a matching key

    let index = hashmap.lookup(key);
    assert_eq!(index, key as usize);
}

#[test]
fn test_lookup_with_empty_slot() {
    let hashmap = HashMap::new(15);
    let key = 10; // Expect an empty slot for key

    let index = hashmap.lookup(key);
    assert_eq!(index, key as usize & hashmap.mask as usize);
}

