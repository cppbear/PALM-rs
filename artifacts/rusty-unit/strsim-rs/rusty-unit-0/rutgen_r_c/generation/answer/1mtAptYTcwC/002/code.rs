// Answer 0

#[test]
fn test_get_mut_with_extended_ascii() {
    struct DummyValue {
        value: usize,
    }
    impl Default for DummyValue {
        fn default() -> Self {
            DummyValue { value: 0 }
        }
    }

    struct DummyMapElement {
        key: u32,
        value: DummyValue,
    }

    struct GrowingHashmapChar {
        used: i32,
        fill: i32,
        mask: i32,
        map: Option<Vec<DummyMapElement>>,
    }

    impl GrowingHashmapChar {
        fn new() -> Self {
            GrowingHashmapChar {
                used: 0,
                fill: 0,
                mask: 255,
                map: Some(vec![DummyMapElement { key: 0, value: DummyValue::default() }; 256]),
            }
        }
        
        fn get_mut(&mut self, key: u32) -> &mut DummyValue {
            if self.map.is_none() {
                self.map = Some(vec![DummyMapElement { key: 0, value: DummyValue::default() }; 256]);
            }
            self.map.as_mut().unwrap().get_mut(key as usize).unwrap().value
        }
    }

    struct HybridGrowingHashmapChar {
        map: GrowingHashmapChar,
        extended_ascii: [DummyValue; 256],
    }

    impl HybridGrowingHashmapChar {
        fn new() -> Self {
            HybridGrowingHashmapChar {
                map: GrowingHashmapChar::new(),
                extended_ascii: [DummyValue::default(); 256],
            }
        }

        fn get_mut(&mut self, key: char) -> &mut DummyValue {
            let value = key as u32;
            if value <= 255 {
                let val_u8 = u8::try_from(value).expect("we check the bounds above");
                &mut self.extended_ascii[usize::from(val_u8)]
            } else {
                self.map.get_mut(value)
            }
        }
    }

    let mut hashmap = HybridGrowingHashmapChar::new();

    // Testing for a character within the extended ASCII range (0-255)
    let value_255 = hashmap.get_mut('\u{FF}'); 
    value_255.value = 255;
    assert_eq!(hashmap.extended_ascii[255].value, 255);

    // Testing for a character outside the extended ASCII range (256-4095)
    let value_256 = hashmap.get_mut('\u{100}'); 
    value_256.value = 300;
    assert_eq!(hashmap.map.get_mut(256).unwrap().value.value, 300);
}

#[test]
#[should_panic]
fn test_get_mut_should_panic_for_unmapped_value() {
    struct DummyValue {
        value: usize,
    }
    
    struct DummyMapElement {
        key: u32,
        value: DummyValue,
    }

    struct GrowingHashmapChar {
        used: i32,
        fill: i32,
        mask: i32,
        map: Option<Vec<DummyMapElement>>,
    }

    impl GrowingHashmapChar {
        fn new() -> Self {
            GrowingHashmapChar {
                used: 0,
                fill: 0,
                mask: 255,
                map: Some(vec![
                    DummyMapElement { key: 0, value: DummyValue { value: 0 } }; 256
                ]),
            }
        }

        fn get_mut(&mut self, key: u32) -> &mut DummyValue {
            self.map.as_mut().unwrap().get_mut(key as usize).unwrap().value
        }
    }

    struct HybridGrowingHashmapChar {
        map: GrowingHashmapChar,
        extended_ascii: [DummyValue; 256],
    }

    impl HybridGrowingHashmapChar {
        fn new() -> Self {
            HybridGrowingHashmapChar {
                map: GrowingHashmapChar::new(),
                extended_ascii: [DummyValue { value: 0 }; 256],
            }
        }

        fn get_mut(&mut self, key: char) -> &mut DummyValue {
            let value = key as u32;
            if value <= 255 {
                let val_u8 = u8::try_from(value).expect("we check the bounds above");
                &mut self.extended_ascii[usize::from(val_u8)]
            } else {
                self.map.get_mut(value)
            }
        }
    }

    let mut hashmap = HybridGrowingHashmapChar::new();

    // Testing for a character that has not been initialized in the map
    let _value = hashmap.get_mut('\u{1000}'); // This should panic as the map is not initialized yet for this value
}

