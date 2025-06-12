// Answer 0

#[derive(Debug)]
struct TestStruct<K> {
    key: K,
}

impl<K> TestStruct<K> {
    fn new(key: K) -> Self {
        TestStruct { key }
    }

    fn key(self) -> K {
        self.key
    }
}

#[test]
fn test_key_return_value() {
    let test_value = "test_key";
    let test_struct = TestStruct::new(test_value);
    assert_eq!(test_struct.key(), test_value);
}

#[test]
fn test_key_return_integer() {
    let test_value = 42;
    let test_struct = TestStruct::new(test_value);
    assert_eq!(test_struct.key(), test_value);
}

#[test]
#[should_panic]
fn test_key_panic_on_empty_option() {
    struct OptionKey {
        key: Option<String>,
    }

    impl OptionKey {
        fn new(key: Option<String>) -> Self {
            OptionKey { key }
        }

        fn key(self) -> Option<String> {
            self.key
        }
    }

    let test_struct = OptionKey::new(None);
    let _ = test_struct.key().expect("called `key()` on `None`");
}

