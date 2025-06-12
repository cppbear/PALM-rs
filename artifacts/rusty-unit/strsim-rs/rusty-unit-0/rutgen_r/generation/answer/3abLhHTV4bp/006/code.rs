// Answer 0

fn test_grow_new_size_greater_than_min_used() {
    struct GrowingHashmapMapElemChar {
        key: char,
        value: i32,
    }

    struct GrowingHashmap {
        map: Option<Vec<GrowingHashmapMapElemChar>>,
        mask: usize,
        used: usize,
        fill: usize,
    }

    impl GrowingHashmap {
        fn lookup(&self, key: char) -> usize {
            key as usize % self.map.as_ref().unwrap().len()
        }

        fn grow(&mut self, min_used: i32) {
            let mut new_size = self.mask + 1;
            while new_size <= min_used as usize {
                new_size <<= 1;
            }

            self.fill = self.used;
            self.mask = new_size - 1;

            let old_map = std::mem::replace(
                self.map.as_mut().expect("callers have to ensure map is allocated"),
                vec![GrowingHashmapMapElemChar::default(); new_size],
            );

            for elem in old_map {
                if elem.key != '\0' {
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

    let mut hashmap = GrowingHashmap {
        map: Some(vec![GrowingHashmapMapElemChar { key: 'a', value: 1 }, GrowingHashmapMapElemChar { key: '\0', value: 0 }]),
        mask: 1,
        used: 1,
        fill: 0,
    };

    hashmap.grow(3);
    assert_eq!(hashmap.mask, 3);
}

fn test_grow_element_exists() {
    struct GrowingHashmapMapElemChar {
        key: char,
        value: i32,
    }

    struct GrowingHashmap {
        map: Option<Vec<GrowingHashmapMapElemChar>>,
        mask: usize,
        used: usize,
        fill: usize,
    }

    impl GrowingHashmap {
        fn lookup(&self, key: char) -> usize {
            key as usize % self.map.as_ref().unwrap().len()
        }

        fn grow(&mut self, min_used: i32) {
            // grow function implementation
        }
    }

    let mut hashmap = GrowingHashmap {
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 'b', value: 2 }, 
            GrowingHashmapMapElemChar { key: '\0', value: 0 }
        ]),
        mask: 1,
        used: 1,
        fill: 0,
    };

    hashmap.grow(5);
    assert_eq!(hashmap.mask, 3);
}

fn test_grow_used_zero() {
    struct GrowingHashmapMapElemChar {
        key: char,
        value: i32,
    }

    struct GrowingHashmap {
        map: Option<Vec<GrowingHashmapMapElemChar>>,
        mask: usize,
        used: usize,
        fill: usize,
    }

    impl GrowingHashmap {
        fn lookup(&self, key: char) -> usize {
            key as usize % self.map.as_ref().unwrap().len()
        }

        fn grow(&mut self, min_used: i32) {
            // grow function implementation
        }
    }

    let mut hashmap = GrowingHashmap {
        map: Some(vec![
            GrowingHashmapMapElemChar { key: '\0', value: 0 },
        ]),
        mask: 1,
        used: 0,
        fill: 0,
    };

    hashmap.grow(2);
    assert_eq!(hashmap.used, 0);
}

#[test]
fn run_tests() {
    test_grow_new_size_greater_than_min_used();
    test_grow_element_exists();
    test_grow_used_zero();
}

