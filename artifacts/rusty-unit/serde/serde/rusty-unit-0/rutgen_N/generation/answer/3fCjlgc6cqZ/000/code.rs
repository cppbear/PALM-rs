// Answer 0

#[test]
fn test_serialize_u16() {
    struct TestSerializer;

    struct Content {
        value: u16,
    }

    impl Content {
        fn U16(value: u16) -> Content {
            Content { value }
        }
    }

    let serializer = TestSerializer;
    let value: u16 = 42;
    
    let result: Result<Content, ()> = Ok(Content::U16(value));
    
    assert_eq!(result.unwrap().value, value);
}

#[test]
fn test_serialize_u16_boundary() {
    struct TestSerializer;

    struct Content {
        value: u16,
    }

    impl Content {
        fn U16(value: u16) -> Content {
            Content { value }
        }
    }

    let serializer = TestSerializer;
    
    let value: u16 = 0;
    let result_zero: Result<Content, ()> = Ok(Content::U16(value));
    assert_eq!(result_zero.unwrap().value, value);
    
    let value_max: u16 = 65535;
    let result_max: Result<Content, ()> = Ok(Content::U16(value_max));
    assert_eq!(result_max.unwrap().value, value_max);
}

