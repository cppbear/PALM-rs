// Answer 0

#[test]
fn test_key_mut() {
    struct TestMap {
        key: i32,
    }

    impl TestMap {
        fn new(key: i32) -> Self {
            TestMap { key }
        }

        fn key_mut(&mut self) -> &mut i32 {
            &mut self.key
        }
    }

    // Test case: Normal scenario
    let mut map = TestMap::new(10);
    {
        let key_reference = map.key_mut();
        *key_reference += 5;
    }
    assert_eq!(map.key, 15);

    // Test case: Boundary condition with negative key
    let mut map_negative = TestMap::new(-5);
    {
        let key_reference = map_negative.key_mut();
        *key_reference = -10;
    }
    assert_eq!(map_negative.key, -10);

    // Test case: Boundary condition with zero key
    let mut map_zero = TestMap::new(0);
    {
        let key_reference = map_zero.key_mut();
        *key_reference += 1;
    }
    assert_eq!(map_zero.key, 1);
}

