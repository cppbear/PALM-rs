// Answer 0

#[test]
fn test_new() {
    struct MyMap {
        data: std::collections::HashMap<String, String>,
    }

    let mut my_map = MyMap {
        data: std::collections::HashMap::new(),
    };

    let name = "test_variant";
    let result = new(&mut my_map.data, name);

    assert_eq!(result.name, name);
    assert!(result.fields.is_empty());
}

