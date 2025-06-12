// Answer 0

#[derive(Clone)]
struct TestStruct {
    extended_ascii: Vec<u8>,
    map: std::collections::HashMap<u32, u8>,
}

impl TestStruct {
    fn new() -> Self {
        TestStruct {
            extended_ascii: vec![0; 256],
            map: std::collections::HashMap::new(),
        }
    }

    fn get(&self, key: char) -> u8 {
        let value = key as u32;
        if value <= 255 {
            let val_u8 = u8::try_from(value).expect("we check the bounds above");
            self.extended_ascii[usize::from(val_u8)]
        } else {
            *self.map.get(&value).unwrap_or(&0)
        }
    }
}

#[test]
fn test_get_max_boundary() {
    let test_struct = TestStruct::new();
    let key = 'ÿ'; // Unicode character for 255
    let result = test_struct.get(key);
    assert_eq!(result, 0); // Default value since extended_ascii is zero-initialized
}

#[test]
fn test_get_with_valid_ascii() {
    let mut test_struct = TestStruct::new();
    test_struct.extended_ascii[65] = 100; // Set the value for character 'A'
    let key = 'A';
    let result = test_struct.get(key);
    assert_eq!(result, 100); // Expect 100 for ASCII 'A'
}

#[test]
fn test_get_with_panic_scenario() {
    let test_struct = TestStruct::new();
    let key = 'Ā'; // Unicode character with value above 255
    let result = test_struct.get(key);
    assert_eq!(result, 0); // Should return default value since map does not have value
}

