// Answer 0

#[test]
fn test_equivalent_with_matching_keys() {
    struct Key(String);
    
    struct EquivalentStruct;

    impl Equivalent<Key> for EquivalentStruct {
        fn equivalent(&self, other: &Key) -> bool {
            self.0 == other.0
        }
    }

    let equivalent_struct = EquivalentStruct;
    let closure = equivalent(&equivalent_struct);
    
    let key_a = Key("test".to_string());
    let key_b = Key("test".to_string());
    let key_c = Key("different".to_string());

    assert!(closure(&key_a));
    assert!(closure(&key_b));
    assert!(!closure(&key_c));
}

#[test]
fn test_equivalent_with_empty_string() {
    struct Key(String);
    
    struct EquivalentStruct;

    impl Equivalent<Key> for EquivalentStruct {
        fn equivalent(&self, other: &Key) -> bool {
            self.0 == other.0
        }
    }

    let equivalent_struct = EquivalentStruct;
    let closure = equivalent(&equivalent_struct);
    
    let key_a = Key("".to_string());
    let key_b = Key("".to_string());
    let key_c = Key("non_empty".to_string());

    assert!(closure(&key_a));
    assert!(closure(&key_b));
    assert!(!closure(&key_c));
}

#[test]
fn test_equivalent_with_different_key_types() {
    struct IntKey(i32);
    
    struct EquivalentStruct;

    impl Equivalent<IntKey> for EquivalentStruct {
        fn equivalent(&self, other: &IntKey) -> bool {
            other.0 == 42
        }
    }

    let equivalent_struct = EquivalentStruct;
    let closure = equivalent(&equivalent_struct);
    
    let key_a = IntKey(42);
    let key_b = IntKey(10);

    assert!(closure(&key_a));
    assert!(!closure(&key_b));
}

