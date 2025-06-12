// Answer 0

#[test]
fn test_get_mut_below_bound() {
    struct TestValueType(i32);
    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType(0)
        }
    }
    
    let mut hashmap = HybridGrowingHashmapChar {
        map: GrowingHashmapChar {
            used: 0,
            fill: 0,
            mask: 0,
            map: None,
        },
        extended_ascii: [TestValueType::default(); 256],
    };

    let result = hashmap.get_mut('A'); // 'A' has a value of 65, <= 255
    result.0 = 1; // Test modification
    assert_eq!(hashmap.extended_ascii[65].0, 1);
}

#[test]
fn test_get_mut_at_bound() {
    struct TestValueType(i32);
    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType(0)
        }
    }
    
    let mut hashmap = HybridGrowingHashmapChar {
        map: GrowingHashmapChar {
            used: 0,
            fill: 0,
            mask: 0,
            map: None,
        },
        extended_ascii: [TestValueType::default(); 256],
    };

    let result = hashmap.get_mut('\u{FF}'); // '\u{FF}' has a value of 255, <= 255
    result.0 = 1; // Test modification
    assert_eq!(hashmap.extended_ascii[255].0, 1);
}

#[test]
fn test_get_mut_above_bound() {
    struct TestValueType(i32);
    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType(0)
        }
    }
    
    struct MapElem {
        key: u32,
        value: TestValueType,
    }
    impl Default for MapElem {
        fn default() -> Self {
            MapElem {
                key: 0,
                value: TestValueType::default(),
            }
        }
    }
    
    let mut hashmap = HybridGrowingHashmapChar {
        map: GrowingHashmapChar {
            used: 0,
            fill: 0,
            mask: 0,
            map: Some(vec![MapElem::default(); 10]), // Preallocated map
        },
        extended_ascii: [TestValueType::default(); 256],
    };

    let result = hashmap.get_mut('\u{100}'); // '\u{100}' has a value of 256, > 255
    result.0 = 2; // Test modification
    assert_eq!(hashmap.map.as_ref().unwrap()[hashmap.map.lookup(256)].value.0, 2);
}

#[should_panic]
#[test]
fn test_get_mut_panic_on_u8_conversion() {
    struct TestValueType(i32);
    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType(0)
        }
    }
    
    let mut hashmap = HybridGrowingHashmapChar {
        map: GrowingHashmapChar {
            used: 0,
            fill: 0,
            mask: 0,
            map: None,
        },
        extended_ascii: [TestValueType::default(); 256],
    };

    let _ = hashmap.get_mut('\u{10FFFF}'); // Out of bounds for u8, should panic
}

