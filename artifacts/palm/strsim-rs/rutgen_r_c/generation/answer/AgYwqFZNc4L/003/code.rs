// Answer 0

#[test]
fn test_get_mut_initial_allocation() {
    struct TestValue {
        value: i32,
    }
    impl Default for TestValue {
        fn default() -> Self {
            TestValue { value: -1 } // Assign a unique default value
        }
    }

    let mut map: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    let v = map.get_mut(1);
    v.value = 42; // Set value through mutable reference

    assert_eq!(v.value, 42);
}

#[test]
fn test_get_mut_resizing() {
    struct TestValue {
        value: i32,
    }
    impl Default for TestValue {
        fn default() -> Self {
            TestValue { value: -1 }
        }
    }

    let mut map: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    // Adding elements to trigger resizing
    for i in 0..7 {
        let v = map.get_mut(i);
        v.value = i as i32;
    }

    // Before reaching resize threshold
    assert_eq!(map.used, 7);
    assert_eq!(map.fill, 7);

    // Last insertion should trigger resize
    let v = map.get_mut(7);
    v.value = 100;

    assert_eq!(v.value, 100);
    assert!(map.used > 7); // Check that resizing has occurred
}

#[test]
#[should_panic]
fn test_get_mut_accessing_invalid_index() {
    struct TestValue {
        value: i32,
    }
    impl Default for TestValue {
        fn default() -> Self {
            TestValue { value: -1 }
        }
    }

    let mut map: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    // This should cause a panic because the index is invalid (out of bounds)
    let _ = map.get_mut(9999);
}

