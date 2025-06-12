// Answer 0

#[test]
fn test_flat_map_take_entry_some_recognized() {
    let mut entry = Some((
        Content::Str("recognized_key"),
        Content::U32(42),
    ));
    let recognized = ["recognized_key"];
    let result = flat_map_take_entry(&mut entry, &recognized);
    assert_eq!(result, Some((
        Content::Str("recognized_key"),
        Content::U32(42),
    )));
    assert!(entry.is_none());
}

#[test]
fn test_flat_map_take_entry_some_not_recognized() {
    let mut entry = Some((
        Content::Str("not_recognized_key"),
        Content::U32(42),
    ));
    let recognized = ["recognized_key"];
    let result = flat_map_take_entry(&mut entry, &recognized);
    assert_eq!(result, None);
    assert!(entry.is_some());
}

#[test]
fn test_flat_map_take_entry_none() {
    let mut entry: Option<(Content<'static>, Content<'static>)> = None;
    let recognized = ["recognized_key"];
    let result = flat_map_take_entry(&mut entry, &recognized);
    assert_eq!(result, None);
    assert!(entry.is_none());
}

