// Answer 0

#[derive(Debug)]
struct TestMap<K, V> {
    key: K,
    value: V,
}

impl<K, V> TestMap<K, V> {
    fn new(key: K, value: V) -> Self {
        Self { key, value }
    }

    fn key_mut(&mut self) -> &mut K {
        &mut self.key
    }
}

#[test]
fn test_key_mut_returns_mutable_reference() {
    let mut map = TestMap::new(10, "value");
    let key_ref = map.key_mut();
    *key_ref += 5;
    assert_eq!(*key_ref, 15);
}

#[test]
fn test_key_mut_panic_on_invalid_state() {
    let mut map: Option<TestMap<i32, &str>> = None;
    // This should panic because we are trying to call key_mut on a None value.
    let result = std::panic::catch_unwind(|| {
        map.as_mut().unwrap().key_mut();
    });

    assert!(result.is_err());
}

#[test]
fn test_key_mut_multiple_calls() {
    let mut map = TestMap::new("initial", 100);
    let first_key_ref = map.key_mut();
    *first_key_ref = "changed_once";
    
    let second_key_ref = map.key_mut();
    *second_key_ref = "changed_twice";

    assert_eq!(*first_key_ref, "changed_twice");
    assert_eq!(*second_key_ref, "changed_twice");
}

#[test]
fn test_key_mut_with_string_key() {
    let mut map = TestMap::new(String::from("key"), "value");
    let key_ref = map.key_mut();
    key_ref.push_str(" modified");
    assert_eq!(*key_ref, "key modified");
}

