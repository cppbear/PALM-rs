// Answer 0

#[test]
fn test_get_outside_extended_ascii() {
    struct TestStruct {
        map: std::collections::HashMap<u32, ValueType>,
        extended_ascii: [ValueType; 256],
    }

    enum ValueType {
        Null,
        NonNull(u32),
    }

    let mut map: std::collections::HashMap<u32, ValueType> = std::collections::HashMap::new();
    map.insert(256, ValueType::NonNull(256));
    map.insert(300, ValueType::NonNull(300));

    let extended_ascii = [ValueType::Null; 256];

    let test_struct = TestStruct {
        map,
        extended_ascii,
    };

    // Test with a key that is greater than 255 which should return from the map
    match test_struct.get('ç') { // ASCII value for 'ç' is 231, which is not greater than 255.
        ValueType::Null => assert!(false, "Expected a valid entry from map"),
        ValueType::NonNull(val) => assert_eq!(val, 256), // should return value for 256
    }
}

#[test]
fn test_get_high_value() {
    struct TestStruct {
        map: std::collections::HashMap<u32, ValueType>,
        extended_ascii: [ValueType; 256],
    }

    enum ValueType {
        Null,
        NonNull(u32),
    }

    let mut map: std::collections::HashMap<u32, ValueType> = std::collections::HashMap::new();
    map.insert(u32::MAX, ValueType::NonNull(u32::MAX)); // testing with a very high value

    let extended_ascii = [ValueType::Null; 256];

    let test_struct = TestStruct {
        map,
        extended_ascii,
    };

    // Test with a high value greater than 255, should return valid entry from map
    match test_struct.get('\u{ffff}') { // Using a character that corresponds to a value above 255
        ValueType::Null => assert!(false, "Expected a valid entry from map"),
        ValueType::NonNull(val) => assert_eq!(val, u32::MAX), // should return value for u32::MAX
    }
}

