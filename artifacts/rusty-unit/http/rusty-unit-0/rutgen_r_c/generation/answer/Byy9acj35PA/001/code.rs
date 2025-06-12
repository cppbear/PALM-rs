// Answer 0

#[test]
fn test_clear_non_empty() {
    use std::any::Any;
    use std::collections::HashMap;

    let mut ext = {
        let mut map = HashMap::new();
        map.insert(TypeId::of::<i32>(), Box::new(5i32));
        Extensions { map: Some(Box::new(map)) }
    };

    ext.clear();

    assert!(ext.get::<i32>().is_none());
}

#[test]
fn test_clear_empty() {
    let mut ext = Extensions::new();

    ext.clear();

    assert!(ext.is_empty());
} 

#[test]
fn test_clear_with_multiple_types() {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    let mut ext = {
        let mut map = HashMap::new();
        map.insert(TypeId::of::<i32>(), Box::new(5i32));
        map.insert(TypeId::of::<String>(), Box::new("test".to_string()));
        Extensions { map: Some(Box::new(map)) }
    };

    ext.clear();

    assert!(ext.get::<i32>().is_none());
    assert!(ext.get::<String>().is_none());
}

#[test]
fn test_clear_with_default() {
    use std::collections::HashMap;

    let mut ext = {
        let mut map = HashMap::new();
        map.insert(TypeId::of::<i32>(), Box::new(10i32));
        Extensions { map: Some(Box::new(map)) }
    };

    ext.clear();

    assert!(ext.len() == 0);
}

