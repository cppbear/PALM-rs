// Answer 0

fn equivalent_key_test() {
    struct Key(String);

    impl Equivalent<Key> for Key {
        fn equivalent(&self, other: &Key) -> bool {
            self.0 == other.0
        }
    }

    let key = Key("test".to_string());
    let closure = equivalent_key(&key);

    let tuple1 = (Key("test".to_string()), 1);
    let tuple2 = (Key("not_test".to_string()), 2);

    assert!(closure(&tuple1));
    assert!(!closure(&tuple2));
}

#[test]
fn test_equivalent_key_with_different_value_types() {
    struct Key(String);

    impl Equivalent<Key> for Key {
        fn equivalent(&self, other: &Key) -> bool {
            self.0 == other.0
        }
    }

    let key = Key("example".to_string());
    let closure = equivalent_key(&key);

    let tuple1 = (Key("example".to_string()), 42);
    let tuple2 = (Key("example".to_string()), 3.14);
    let tuple3 = (Key("different".to_string()), 'c');

    assert!(closure(&tuple1));
    assert!(closure(&tuple2));
    assert!(!closure(&tuple3));
}

#[test]
#[should_panic]
fn test_equivalent_key_with_panic() {
    struct Key;

    impl Equivalent<Key> for Key {
        fn equivalent(&self, _: &Key) -> bool {
            panic!("Panic triggered")
        }
    }

    let key = Key;
    let closure = equivalent_key(&key);

    // this should panic
    let _ = closure(&(Key, 10));
}

