// Answer 0

#[test]
fn test_get_mut_with_char_boundary_value() {
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

        fn get_mut(&mut self, key: char) -> &mut u8 {
            let value = key as u32;
            if value <= 255 {
                let val_u8 = u8::try_from(value).expect("we check the bounds above");
                &mut self.extended_ascii[usize::from(val_u8)]
            } else {
                self.map.get_mut(value).unwrap()
            }
        }
    }

    let mut test_struct = TestStruct::new();

    // Testing with a char that corresponds to the maximum u8 value of 255
    let key = '\u{FF}';
    let result = test_struct.get_mut(key);
    *result = 42; // Set the value at index 255 to 42
    assert_eq!(test_struct.extended_ascii[255], 42);
}

#[test]
#[should_panic]
fn test_get_mut_with_char_above_bound() {
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

        fn get_mut(&mut self, key: char) -> &mut u8 {
            let value = key as u32;
            if value <= 255 {
                let val_u8 = u8::try_from(value).expect("we check the bounds above");
                &mut self.extended_ascii[usize::from(val_u8)]
            } else {
                self.map.get_mut(value).unwrap()
            }
        }
    }

    let mut test_struct = TestStruct::new();

    // Testing with a char that corresponds to a value above the valid u8 range
    let key = '\u{100}'; // which is 256 in decimal
    test_struct.get_mut(key); // This should panic because u8::try_from(256) will fail
}

