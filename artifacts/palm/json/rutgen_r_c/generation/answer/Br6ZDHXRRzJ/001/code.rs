// Answer 0

#[test]
fn test_unexpected_with_object() {
    use crate::value::Value;
    use crate::map::Map;

    struct SimpleMap;

    impl<K, V> Map<K, V> {
        fn new() -> Self {
            SimpleMap
        }
    }

    let obj_value = Value::Object(SimpleMap::new());
    let unexpected_result = obj_value.unexpected();
    
    match unexpected_result {
        Unexpected::Map => (),
        _ => panic!("Expected Unexpected::Map, found a different variant"),
    }
}

