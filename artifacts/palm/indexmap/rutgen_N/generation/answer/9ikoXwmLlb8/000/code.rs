// Answer 0

#[derive(Debug)]
struct MyStruct<K, V> {
    key: K,
    value: V,
}

impl<K, V> MyStruct<K, V> {
    fn refs(&self) -> (&K, &V) {
        (&self.key, &self.value)
    }
}

#[test]
fn test_refs() {
    let my_struct = MyStruct {
        key: 42,
        value: "Hello",
    };
    
    let (key_ref, value_ref) = my_struct.refs();
    
    assert_eq!(*key_ref, 42);
    assert_eq!(*value_ref, "Hello");
}

#[test]
fn test_refs_with_different_types() {
    let my_struct = MyStruct {
        key: "KeyString",
        value: 3.14,
    };
    
    let (key_ref, value_ref) = my_struct.refs();
    
    assert_eq!(*key_ref, "KeyString");
    assert_eq!(*value_ref, 3.14);
}

