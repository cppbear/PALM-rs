// Answer 0

#[test]
fn test_into_deserializer() {
    struct Deserializer {
        value: i32,
    }

    impl Deserializer {
        fn new(value: i32) -> Self {
            Deserializer { value }
        }
    
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let deserializer = Deserializer::new(42);
    let result = deserializer.into_deserializer();
    assert_eq!(result.value, 42);
}

#[test]
fn test_into_deserializer_zero() {
    struct Deserializer {
        value: i32,
    }

    impl Deserializer {
        fn new(value: i32) -> Self {
            Deserializer { value }
        }
    
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let deserializer = Deserializer::new(0);
    let result = deserializer.into_deserializer();
    assert_eq!(result.value, 0);
}

#[test]
fn test_into_deserializer_negative() {
    struct Deserializer {
        value: i32,
    }

    impl Deserializer {
        fn new(value: i32) -> Self {
            Deserializer { value }
        }
    
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let deserializer = Deserializer::new(-1);
    let result = deserializer.into_deserializer();
    assert_eq!(result.value, -1);
}

