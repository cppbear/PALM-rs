// Answer 0

#[test]
fn test_flat_map_take_entry_recognized_key() {
    let mut entry = Some((
        Content::Str("key".into()),
        Content::U32(42),
    ));
    let recognized = vec!["key", "another_key"];
    
    let result = flat_map_take_entry(&mut entry, &recognized);

    assert!(result.is_some());
    assert_eq!(result, Some((Content::Str("key".into()), Content::U32(42))));
}

#[test]
fn test_flat_map_take_entry_unrecognized_key() {
    let mut entry = Some((
        Content::Str("unrecognized_key".into()),
        Content::U32(42),
    ));
    let recognized = vec!["key", "another_key"];

    let result = flat_map_take_entry(&mut entry, &recognized);

    assert!(result.is_none());
    assert_eq!(entry, Some((Content::Str("unrecognized_key".into()), Content::U32(42))));
}

#[test]
fn test_flat_map_take_entry_none_entry() {
    let mut entry: Option<(Content, Content)> = None;
    let recognized = vec!["key", "another_key"];
    
    let result = flat_map_take_entry(&mut entry, &recognized);
    
    assert!(result.is_none());
    assert!(entry.is_none());
}

#[test]
fn test_flat_map_take_entry_edge_case_empty_recognized() {
    let mut entry = Some((
        Content::Str("some_key".into()),
        Content::U32(42),
    ));
    let recognized: Vec<&str> = vec![];

    let result = flat_map_take_entry(&mut entry, &recognized);

    assert!(result.is_none());
    assert_eq!(entry, Some((Content::Str("some_key".into()), Content::U32(42))));
}

