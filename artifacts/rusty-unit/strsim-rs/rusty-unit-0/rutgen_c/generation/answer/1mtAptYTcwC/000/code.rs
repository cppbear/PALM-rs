// Answer 0

#[test]
fn test_get_mut_extended_ascii() {
    struct TestValue {
        value: usize,
    }

    impl Default for TestValue {
        fn default() -> Self {
            TestValue { value: 0 }
        }
    }

    let mut hashmap = HybridGrowingHashmapChar {
        map: GrowingHashmapChar {
            used: 0,
            fill: 0,
            mask: 0,
            map: None,
        },
        extended_ascii: [TestValue::default(); 256],
    };

    let key = 'A'; // ASCII character
    let val = hashmap.get_mut(key);
    val.value = 10; // Set value for extended ASCII

    assert_eq!(hashmap.extended_ascii[usize::from(key as u8)].value, 10);
}

#[test]
fn test_get_mut_non_extended_ascii() {
    struct TestValue {
        value: usize,
    }

    impl Default for TestValue {
        fn default() -> Self {
            TestValue { value: 0 }
        }
    }

    impl GrowingHashmapChar<TestValue> {
        fn get_mut(&mut self, key: u32) -> &mut TestValue {
            if self.map.is_none() {
                self.allocate();
            }
            // Simulate allocation and growth for this test
            self.map.as_mut().unwrap().push(GrowingHashmapMapElemChar {
                key,
                value: TestValue::default(),
            });

            self.map.as_mut().unwrap().last_mut().unwrap().value
        }
    }

    struct GrowingHashmapMapElemChar<ValueType> {
        key: u32,
        value: ValueType,
    }

    let mut hashmap = HybridGrowingHashmapChar {
        map: GrowingHashmapChar {
            used: 0,
            fill: 0,
            mask: 0,
            map: Some(Vec::new()),
        },
        extended_ascii: [TestValue::default(); 256],
    };

    let key = 'ñ'; // Non-ASCII character
    let val = hashmap.get_mut(key);
    val.value = 20; // Set value for non-extended ASCII

    let elem = &hashmap.map.map.as_ref().unwrap()[0];
    assert_eq!(elem.value.value, 20);
}

