// Answer 0

fn grow_tests() {
    struct GrowingHashmapMapElemChar<ValueType> {
        key: char,
        value: ValueType,
    }

    struct GrowingHashmap<ValueType> {
        map: Option<Vec<GrowingHashmapMapElemChar<ValueType>>>,
        used: i32,
        mask: i32,
        fill: i32,
    }

    impl<ValueType: Default> GrowingHashmap<ValueType> {
        fn new() -> Self {
            GrowingHashmap {
                map: Some(Vec::new()),
                used: 0,
                mask: 0,
                fill: 0,
            }
        }

        fn lookup(&self, key: char) -> usize {
            self.map.as_ref().unwrap().iter().position(|elem| elem.key == key).unwrap_or_else(|| self.map.as_ref().unwrap().len())
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
                vec![GrowingHashmapMapElemChar::<ValueType>::default(); new_size as usize],
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
    fn test_grow_equal_new_size_and_min_used() {
        let mut hashmap: GrowingHashmap<i32> = GrowingHashmap::new();
        hashmap.mask = 3; // Set to a value that will make new_size equal to min_used
        hashmap.used = 2; // Set used for testing
        hashmap.map = Some(vec![
            GrowingHashmapMapElemChar { key: 'a', value: 1 },
            GrowingHashmapMapElemChar { key: 'b', value: 2 },
        ]);
        hashmap.grow(4); // new_size will equal min_used
        assert_eq!(hashmap.mask, 7); // Check that the new mask is correct
    }

    #[test]
    fn test_grow_invalidate_existing_map() {
        let mut hashmap: GrowingHashmap<i32> = GrowingHashmap::new();
        hashmap.map = Some(vec![
            GrowingHashmapMapElemChar { key: 'c', value: 0 }, // Default value should not be copied
            GrowingHashmapMapElemChar { key: 'd', value: 5 },
        ]);
        hashmap.used = 2;
        hashmap.mask = 1; // Starts small to test growth

        hashmap.grow(2); // Trigger growth, new size should be 2

        assert_eq!(hashmap.map.as_ref().unwrap().len(), 2); // New map size
        assert_eq!(hashmap.used, 1); // Only 'd' should be remaining
    }

    #[should_panic(expected = "callers have to ensure map is allocated")]
    #[test]
    fn test_grow_map_not_allocated() {
        let mut hashmap: GrowingHashmap<i32> = GrowingHashmap::new();
        hashmap.map = None; // Mimicking a condition where the map is not allocated
        hashmap.grow(1); // Should panic
    }

    #[test]
    fn test_grow_no_elements_in_old_map() {
        let mut hashmap: GrowingHashmap<i32> = GrowingHashmap::new();
        hashmap.map = Some(vec![GrowingHashmapMapElemChar { key: 'e', value: 0 }]);
        hashmap.used = 1;
        hashmap.mask = 1; // Small size to trigger growth

        hashmap.grow(1); // Should grow without any valid elements triggering panic

        assert_eq!(hashmap.used, 1); // Check the used count
    }
}

