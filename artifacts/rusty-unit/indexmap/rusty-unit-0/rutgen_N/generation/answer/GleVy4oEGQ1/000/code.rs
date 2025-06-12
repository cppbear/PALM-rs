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
fn test_key() {
    let test_instance = TestStruct::new(42);
    assert_eq!(test_instance.key(), 42);
}

#[test]
fn test_key_with_string() {
    let test_instance = TestStruct::new(String::from("test"));
    assert_eq!(test_instance.key(), "test");
}

#[test]
#[should_panic]
fn test_key_panic() {
    let test_instance: TestStruct<Option<i32>> = TestStruct::new(None);
    assert!(test_instance.key().is_some());
}

