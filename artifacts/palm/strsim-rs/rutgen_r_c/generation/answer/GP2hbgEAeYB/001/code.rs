// Answer 0

#[test]
fn test_get_with_valid_ascii_character() {
    struct TestValue {
        value: usize,
    }

    impl Default for TestValue {
        fn default() -> Self {
            TestValue { value: 0 }
        }
    }

    struct TestGrowingHashmapChar {
        map: Option<Vec<GrowingHashmapMapElemChar<TestValue>>>,
    }

    impl TestGrowingHashmapChar {
        fn new() -> Self {
            TestGrowingHashmapChar { map: None }
        }
    }

    let mut extended_ascii = [TestValue::default(); 256];
    extended_ascii[255] = TestValue { value: 255 };

    let hashmap = HybridGrowingHashmapChar {
        map: TestGrowingHashmapChar::new(),
        extended_ascii,
    };

    let result = hashmap.get(char::from(255));
    assert_eq!(result.value, 255);
}

#[test]
#[should_panic(expected = "we check the bounds above")]
fn test_get_with_out_of_bounds_character() {
    struct TestValue {
        value: usize,
    }

    impl Default for TestValue {
        fn default() -> Self {
            TestValue { value: 0 }
        }
    }

    struct TestGrowingHashmapChar {
        map: Option<Vec<GrowingHashmapMapElemChar<TestValue>>>,
    }

    impl TestGrowingHashmapChar {
        fn new() -> Self {
            TestGrowingHashmapChar { map: None }
        }
    }

    let extended_ascii = [TestValue::default(); 256];

    let hashmap = HybridGrowingHashmapChar {
        map: TestGrowingHashmapChar::new(),
        extended_ascii,
    };

    // This character is out of bounds (value = 256)
    let _result = hashmap.get(char::from(256));
}

