// Answer 0

#[test]
fn test_index_mut_with_existing_key() {
    use serde_json::json;
    
    let mut data = json!({ "x": 0 });

    // replace an existing key
    data["x"] = json!(1);
    assert_eq!(data["x"], json!(1));
}

#[test]
fn test_index_mut_with_new_key() {
    use serde_json::json;
    
    let mut data = json!(null);

    // insert a new key
    data["y"] = json!([false, false, false]);
    assert_eq!(data["y"], json!([false, false, false]));
}

#[test]
fn test_index_mut_with_array_access() {
    use serde_json::json;
    
    let mut data = json!([false, false, false]);

    // replace an array value
    data[0] = json!(true);
    assert_eq!(data[0], json!(true));
}

#[test]
#[should_panic]
fn test_index_mut_on_non_object_for_string_index() {
    use serde_json::json;

    let mut data = json!(42); // non-object

    // this should panic as 42 is not an object or null
    data["new_key"] = json!(true);
}

#[test]
#[should_panic]
fn test_index_mut_on_array_too_small() {
    use serde_json::json;

    let mut data = json!([1]); // array of length 1

    // this should panic because we are trying to access index 2
    data[2] = json!(true);
}

#[test]
fn test_index_mut_with_nested_key_insertion() {
    use serde_json::json;
    
    let mut data = json!(null);

    // inserted a deeply nested key
    data["a"]["b"]["c"]["d"] = json!(true);
    assert_eq!(data["a"]["b"]["c"]["d"], json!(true));
}

