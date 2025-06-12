// Answer 0

#[test]
fn test_get_mut_with_extended_ascii() {
    struct TestStruct {
        extended_ascii: [u32; 256],
        map: std::collections::HashMap<u32, u32>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                extended_ascii: [0; 256],
                map: std::collections::HashMap::new(),
            }
        }

        fn get_mut(&mut self, key: char) -> &mut u32 {
            let value = key as u32;
            if value <= 255 {
                let val_u8 = u8::try_from(value).expect("we check the bounds above");
                &mut self.extended_ascii[usize::from(val_u8)]
            } else {
                self.map.get_mut(value).expect("Key should exist in map")
            }
        }
    }

    let mut test_struct = TestStruct::new();
    
    // Test for keys within the extended ASCII range
    test_struct.get_mut('A');
    assert_eq!(test_struct.extended_ascii[65], 0); // 'A' corresponds to 65
    
    // Modify extended ASCII value
    *test_struct.get_mut('A') += 1;
    assert_eq!(test_struct.extended_ascii[65], 1);
    
    // Test for keys outside the extended ASCII range
    test_struct.map.insert(256, 0);
    test_struct.get_mut('€'); // '€' corresponds to 0x20AC which is 8364 in decimal
    assert_eq!(test_struct.map.get(&256), Some(&0)); // Ensure the map is still valid
    *test_struct.get_mut('€') += 1; // Increment the value
    assert!(test_struct.map.get(&256).is_some()); // Ensure the key still exists
}

