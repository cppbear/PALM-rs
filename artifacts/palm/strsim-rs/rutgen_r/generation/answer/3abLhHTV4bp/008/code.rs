// Answer 0

#[derive(Default)]
struct GrowingHashmapMapElemChar {
    key: char,
    value: i32,
}

struct GrowingHashmap {
    mask: usize,
    used: usize,
    fill: usize,
    map: Option<Vec<GrowingHashmapMapElemChar>>,
}

impl GrowingHashmap {
    fn new(size: usize) -> Self {
        Self {
            mask: size - 1,
            used: 0,
            fill: 0,
            map: Some(vec![GrowingHashmapMapElemChar::default(); size]),
        }
    }

    fn grow(&mut self, min_used: i32) {
        let mut new_size = self.mask + 1;
        while new_size <= min_used {
            new_size <<= 1;
        }

        self.fill = self.used;
        self.mask = new_size - 1;

        let old_map = std::mem::replace(
            self.map
                .as_mut()
                .expect("callers have to ensure map is allocated"),
            vec![GrowingHashmapMapElemChar::default(); new_size as usize],
        );

        for elem in old_map {
            if elem.value != Default::default() {
                let j = self.lookup(elem.key);
                let new_elem = &mut self.map.as_mut().expect("map created above")[j];
                new_elem.key = elem.key;
                new_elem.value = elem.value;
                self.used -= 1;
                if self.used == 0 {
                    break;
                }
            }
        }

        self.used = self.fill;
    }

    fn lookup(&self, key: char) -> usize {
        // A basic implementation for the lookup function
        key as usize & self.mask
    }
}

#[test]
fn test_grow_new_size_less_than_min_used() {
    let mut hashmap = GrowingHashmap::new(4);
    hashmap.used = 2; // Simulate existing elements
    hashmap.fill = 2;

    // The min_used is set to a value greater than the current map size
    // This will test if the new size will be calculated correctly and grown
    hashmap.grow(10);
    assert_eq!(hashmap.mask, 15); // New size is 16, so mask should be 15
}

#[test]
#[should_panic(expected = "callers have to ensure map is allocated")]
fn test_grow_panic_when_map_not_allocated() {
    let mut hashmap = GrowingHashmap {
        mask: 0,
        used: 0,
        fill: 0,
        map: None, // No map allocated, will trigger panic
    };
    hashmap.grow(5);
}

#[test]
fn test_grow_elem_in_old_map_false() {
    let mut hashmap = GrowingHashmap::new(4);
    hashmap.used = 1;
    hashmap.fill = 1;

    // Populate the map with a default value element
    hashmap.map.as_mut().unwrap()[0] = GrowingHashmapMapElemChar::default();
    hashmap.grow(5);
    
    assert_eq!(hashmap.used, 1); // Ensure that used stays the same since the element was default
    assert!(hashmap.map.is_some());
    assert_eq!(hashmap.map.as_ref().unwrap().len(), 8); // Should double the size on grow
}

