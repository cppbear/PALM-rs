// Answer 0

#[test]
fn test_pointer_mut_valid_json_object() {
    use serde_json::{json, Value};

    let mut value: Value = json!({"x": 1.0, "y": 2.0});

    assert_eq!(value.pointer_mut("/x"), Some(&mut 1.0.into()));
    *value.pointer_mut("/x").unwrap() = 3.5.into();
    assert_eq!(value.pointer("/x"), Some(&3.5.into()));
}

#[test]
fn test_pointer_mut_valid_json_array() {
    use serde_json::{json, Value};

    let mut value: Value = json!({"array": [1.0, 2.0, 3.0]});

    assert_eq!(value.pointer_mut("/array/1"), Some(&mut 2.0.into()));
    *value.pointer_mut("/array/1").unwrap() = 4.5.into();
    assert_eq!(value.pointer("/array/1"), Some(&4.5.into()));
}
  
#[test]
fn test_pointer_mut_with_escaped_characters() {
    use serde_json::{json, Value};

    let mut value: Value = json!({"foo~bar": {"baz~1": 42}});

    assert_eq!(value.pointer_mut("/foo~0bar/baz~1"), Some(&mut 42.into()));
    *value.pointer_mut("/foo~0bar/baz~1").unwrap() = 100.into();
    assert_eq!(value.pointer("/foo~0bar/baz~1"), Some(&100.into()));
}

#[test]
fn test_pointer_mut_invalid_pointer() {
    use serde_json::{json, Value};

    let mut value: Value = json!({"a": 1});

    assert_eq!(value.pointer_mut("a"), None);
    assert_eq!(value.pointer_mut("/b/c"), None);
}

#[test]
fn test_pointer_mut_empty_pointer() {
    use serde_json::{json, Value};

    let mut value: Value = json!({"a": 1});

    assert_eq!(value.pointer_mut("/"), Some(&mut value));
}

#[test]
#[should_panic]
fn test_pointer_mut_panic_on_non_recursive_object() {
    use serde_json::{json, Value};

    let mut value: Value = json!(1);

    let _ = value.pointer_mut("/x");
}

