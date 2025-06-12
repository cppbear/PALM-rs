// Answer 0

#[derive(Debug)]
struct Content<'de> {
    value: &'de str,
}

#[test]
fn test_flat_map_take_entry_with_recognized_key() {
    let recognized_keys = vec!["name", "age", "city"];
    let mut entry = Some((Content { value: "name" }, Content { value: "John" }));

    let result = flat_map_take_entry(&mut entry, &recognized_keys);

    assert_eq!(result, Some((Content { value: "name" }, Content { value: "John" })));
    assert_eq!(entry, None);
}

#[test]
fn test_flat_map_take_entry_with_unrecognized_key() {
    let recognized_keys = vec!["name", "age", "city"];
    let mut entry = Some((Content { value: "country" }, Content { value: "USA" }));

    let result = flat_map_take_entry(&mut entry, &recognized_keys);

    assert_eq!(result, None);
    assert_eq!(entry, Some((Content { value: "country" }, Content { value: "USA" })));
}

#[test]
fn test_flat_map_take_entry_with_none() {
    let recognized_keys = vec!["name", "age", "city"];
    let mut entry: Option<(Content, Content)> = None;

    let result = flat_map_take_entry(&mut entry, &recognized_keys);

    assert_eq!(result, None);
    assert_eq!(entry, None);
}

