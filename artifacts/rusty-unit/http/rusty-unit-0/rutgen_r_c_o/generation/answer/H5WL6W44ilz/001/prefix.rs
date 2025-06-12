// Answer 0

#[test]
fn test_insert_empty_map() {
    let mut map = HeaderMap::with_capacity(10);
    let key = "Test-Header".parse().unwrap();
    let val = "TestValue".to_string();
    map.insert(key, val);
}

#[test]
fn test_insert_with_existing_key() {
    let mut map = HeaderMap::with_capacity(10);
    let key = "Example-Header".parse().unwrap();
    let val1 = "FirstValue".to_string();
    let val2 = "SecondValue".to_string();
    map.insert(key.clone(), val1);
    let prev_val = map.insert(key.clone(), val2);
}

#[test]
fn test_insert_large_capacity() {
    let capacity = 32767;
    let mut map = HeaderMap::with_capacity(capacity);
    let key = "Header-90".parse().unwrap();
    let val = "ValueWithMaxCapacity".to_string();
    map.insert(key, val);
}

#[test]
#[should_panic]
fn test_insert_exceeding_max_capacity() {
    let mut map = HeaderMap::with_capacity(32767);
    for i in 0..32768 {
        let key = format!("Header-{}", i).parse().unwrap();
        let val = format!("Value-{}", i).to_string();
        map.insert(key, val);
    }
}

#[test]
fn test_insert_custom_struct_as_key() {
    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    struct HeaderKey {
        name: String,
    }
    
    impl IntoHeaderName for HeaderKey {
        fn into_header_name(self) -> HeaderName {
            self.name.parse().unwrap()
        }
    }
    
    let mut map = HeaderMap::with_capacity(5);
    let key = HeaderKey {
        name: "Custom-Header".to_string(),
    };
    let val = "CustomValue".to_string();
    map.insert(key, val);
}

#[test]
fn test_insert_empty_string_key() {
    let mut map = HeaderMap::with_capacity(10);
    let key = "".parse().unwrap();
    let val = "EmptyKeyValue".to_string();
    map.insert(key, val);
}

#[test]
fn test_insert_long_key_and_value() {
    let mut map = HeaderMap::with_capacity(10);
    let key = "A".repeat(256).parse().unwrap();
    let val = "B".repeat(256).to_string();
    map.insert(key, val);
}

