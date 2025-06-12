// Answer 0

#[derive(Default)]
struct GrowingHashmapMapElemChar {
    key: char,
    value: i32,
}

struct GrowingHashmap<ValueType> {
    mask: usize,
    used: i32,
    fill: i32,
    map: Option<Vec<GrowingHashmapMapElemChar>>,
}

impl<ValueType> GrowingHashmap<ValueType> {
    fn new(size: usize) -> Self {
        Self {
            mask: size - 1,
            used: 0,
            fill: 0,
            map: Some(vec![GrowingHashmapMapElemChar::default(); size]),
        }
    }

    fn lookup(&self, key: char) -> usize {
        // Simplified lookup for testing purpose
        key as usize & self.mask
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
}

#[test]
fn test_grow_increases_size() {
    let mut hashmap = GrowingHashmap::<i32>::new(4);
    hashmap.used = 3;
    hashmap.fill = 3;

    hashmap.grow(5);

    assert_eq!(hashmap.mask, 7);
    assert_eq!(hashmap.map.as_ref().unwrap().len(), 8);
}

#[test]
fn test_grow_does_not_change_less_than_mask() {
    let mut hashmap = GrowingHashmap::<i32>::new(4);
    hashmap.used = 2;
    hashmap.fill = 2;

    hashmap.grow(2);

    assert_eq!(hashmap.mask, 3);
    assert_eq!(hashmap.map.as_ref().unwrap().len(), 4);
}

#[test]
fn test_grow_doubles_size() {
    let mut hashmap = GrowingHashmap::<i32>::new(8);
    hashmap.used = 5;
    hashmap.fill = 5;

    hashmap.grow(10);

    assert_eq!(hashmap.mask, 15);
    assert_eq!(hashmap.map.as_ref().unwrap().len(), 16);
}

