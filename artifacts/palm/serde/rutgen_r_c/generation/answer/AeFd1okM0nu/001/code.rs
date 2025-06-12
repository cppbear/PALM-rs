// Answer 0

#[test]
fn test_serialize_value_panics() {
    struct TestError;

    impl ser::Error for TestError {}
    
    struct TestSerializeMap {
        impossible: Impossible<(), TestError>,
    }

    impl TestSerializeMap {
        fn new() -> Self {
            Self {
                impossible: Impossible {
                    void: (),
                    ok: PhantomData,
                    error: PhantomData,
                },
            }
        }
    }

    let mut map = TestSerializeMap::new();
    let result = std::panic::catch_unwind(|| {
        map.impossible.serialize_value(&"test_value").unwrap();
    });

    assert!(result.is_err());
}

#[test]
fn test_serialize_value_with_nothing() {
    struct TestError;

    impl ser::Error for TestError {}

    struct TestSerializeMap {
        impossible: Impossible<(), TestError>,
    }

    impl TestSerializeMap {
        fn new() -> Self {
            Self {
                impossible: Impossible {
                    void: (),
                    ok: PhantomData,
                    error: PhantomData,
                },
            }
        }
    }

    let mut map = TestSerializeMap::new();
    let result = std::panic::catch_unwind(|| {
        map.impossible.serialize_value(&()).unwrap();
    });

    assert!(result.is_err());
}

#[test]
fn test_serialize_value_with_non_serializable() {
    struct TestError;

    impl ser::Error for TestError {}

    struct TestSerializeMap {
        impossible: Impossible<(), TestError>,
    }

    impl TestSerializeMap {
        fn new() -> Self {
            Self {
                impossible: Impossible {
                    void: (),
                    ok: PhantomData,
                    error: PhantomData,
                },
            }
        }
    }

    let mut map = TestSerializeMap::new();
    let result = std::panic::catch_unwind(|| {
        map.impossible.serialize_value(&[]).unwrap(); // passing a non-serializable slice
    });

    assert!(result.is_err());
}

