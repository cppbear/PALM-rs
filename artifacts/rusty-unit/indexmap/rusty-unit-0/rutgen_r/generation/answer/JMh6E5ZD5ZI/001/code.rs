// Answer 0

#[derive(Debug)]
struct TestStruct<K> {
    key: K,
}

impl<K> TestStruct<K> {
    fn new(key: K) -> Self {
        TestStruct { key }
    }

    fn key_ref(&self) -> &K {
        &self.key
    }
}

#[test]
fn test_key_ref_string() {
    let instance = TestStruct::new(String::from("test_key"));
    let result = instance.key_ref();
    assert_eq!(result, "test_key");
}

#[test]
fn test_key_ref_integer() {
    let instance = TestStruct::new(42);
    let result = instance.key_ref();
    assert_eq!(*result, 42);
}

#[test]
fn test_key_ref_float() {
    let instance = TestStruct::new(3.14);
    let result = instance.key_ref();
    assert_eq!(*result, 3.14);
}

#[test]
fn test_key_ref_char() {
    let instance = TestStruct::new('A');
    let result = instance.key_ref();
    assert_eq!(*result, 'A');
}

#[should_panic]
fn test_key_ref_panic() {
    let instance = TestStruct::new(None::<String>);
    let _result = instance.key_ref(); // Attempting to dereference None
}

