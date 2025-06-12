// Answer 0

#[derive(Default)]
struct MapEntry {
    key: u32,
    value: u32,
}

struct HashMap {
    mask: u32,
    map: Option<Box<[MapEntry; 16]>>,
}

impl HashMap {
    fn new(mask: u32) -> Self {
        Self {
            mask,
            map: Some(Box::new(Default::default())),
        }
    }

    fn lookup(&self, key: u32) -> usize {
        let hash = key;
        let mut i = hash as usize & self.mask as usize;

        let map = self
            .map
            .as_ref()
            .expect("callers have to ensure map is allocated");

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
fn test_lookup_return_value_when_key_exists() {
    let mut map = HashMap::new(15); // 4 bits for the mask
    let entry = MapEntry { key: 10, value: 20 };
    map.map.as_mut().unwrap()[10] = entry;

    let index = map.lookup(10);
    assert_eq!(index, 10);
}

#[test]
fn test_lookup_return_value_when_key_does_not_exist_but_default() {
    let map = HashMap::new(15); // 4 bits for the mask

    let index = map.lookup(5);
    assert_eq!(index, 5);
}

#[test]
fn test_lookup_with_collision() {
    let mut map = HashMap::new(15); // 4 bits for the mask
    let entry1 = MapEntry { key: 2, value: 1 };
    let entry2 = MapEntry { key: 3, value: 1 }; // Assume it collides with the hash of key 2
    map.map.as_mut().unwrap()[2] = entry1;
    map.map.as_mut().unwrap()[3] = entry2;

    let index = map.lookup(2);
    assert_eq!(index, 2); // Should return the first collision's index
}

#[should_panic(expected = "callers have to ensure map is allocated")]
#[test]
fn test_lookup_panic_when_map_is_none() {
    let map = HashMap { mask: 15, map: None };
    let _ = map.lookup(1);
}

#[test]
fn test_lookup_overflow_handling() {
    let mut map = HashMap::new(15);
    let entry = MapEntry { key: 0, value: 1 };
    map.map.as_mut().unwrap()[0] = entry;

    let index = map.lookup(0);
    assert_eq!(index, 0); // Key = 0 should return 0 if it's in the map
}

