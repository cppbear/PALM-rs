// Answer 0

#[test]
fn test_pointer_mut_empty_pointer() {
    let mut value = Value::Object(Map::new());
    let result = value.pointer_mut("");
    assert!(result.is_some()); // since it's an empty pointer it should return Some reference to self
}

#[test]
fn test_pointer_mut_invalid_pointer() {
    let mut value = Value::Object(Map::new());
    let result = value.pointer_mut("invalid_pointer");
    assert!(result.is_none()); // since it doesn't start with '/', it should return None
}

#[test]
fn test_pointer_mut_with_invalid_start() {
    let mut value = Value::Object(Map::new());
    let result = value.pointer_mut("not/valid");
    assert!(result.is_none()); // since it doesn't start with '/', it should return None
}

#[test]
fn test_pointer_mut_non_empty_pointer() {
    let mut value = Value::Object(Map::new());
    let result = value.pointer_mut("/valid");
    assert!(result.is_none()); // there is no object/key 'valid', should return None
}

