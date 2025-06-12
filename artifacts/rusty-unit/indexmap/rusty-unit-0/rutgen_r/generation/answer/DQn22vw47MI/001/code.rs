// Answer 0

#[test]
fn test_insert_with_value() {
    struct TestEntry {
        value: i32,
    }
    
    impl TestEntry {
        fn insert_entry(&mut self, value: i32) -> &mut i32 {
            self.value = value;
            &mut self.value
        }
        
        fn into_mut(&mut self) -> &mut i32 {
            &mut self.value
        }
        
        fn insert(&mut self, value: i32) -> &mut i32 {
            self.insert_entry(value).into_mut()
        }
    }
    
    let mut entry = TestEntry { value: 0 };
    let new_value = 42;
    let result = entry.insert(new_value);
    
    assert_eq!(*result, new_value);
    assert_eq!(entry.value, new_value);
}

#[test]
#[should_panic]
fn test_insert_panic_on_multiple_insertions() {
    struct PanicEntry {
        value: i32,
    }
    
    impl PanicEntry {
        fn insert_entry(&mut self, value: i32) -> &mut i32 {
            self.value = value;
            &mut self.value
        }
        
        fn into_mut(&mut self) -> &mut i32 {
            &mut self.value
        }
        
        fn insert(&mut self, value: i32) -> &mut i32 {
            if self.value != 0 {
                panic!("Attempt to insert into a non-empty entry");
            }
            self.insert_entry(value).into_mut()
        }
    }
    
    let mut panic_entry = PanicEntry { value: 1 };
    let _ = panic_entry.insert(42);
}

#[test]
fn test_insert_zero_value() {
    struct ZeroValueEntry {
        value: i32,
    }
    
    impl ZeroValueEntry {
        fn insert_entry(&mut self, value: i32) -> &mut i32 {
            self.value = value;
            &mut self.value
        }
        
        fn into_mut(&mut self) -> &mut i32 {
            &mut self.value
        }
        
        fn insert(&mut self, value: i32) -> &mut i32 {
            self.insert_entry(value).into_mut()
        }
    }
    
    let mut zero_entry = ZeroValueEntry { value: 0 };
    let result = zero_entry.insert(0);
    
    assert_eq!(*result, 0);
    assert_eq!(zero_entry.value, 0);
}

