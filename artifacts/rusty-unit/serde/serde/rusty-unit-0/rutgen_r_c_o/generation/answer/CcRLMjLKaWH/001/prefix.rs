// Answer 0

#[test]
fn test_flat_map_take_entry_recognized_key() {
    let mut entry = Some((Content::Str("recognized_key"), Content::U32(42)));
    let recognized = ["recognized_key", "another_key"];
    flat_map_take_entry(&mut entry, &recognized);
}

#[test]
fn test_flat_map_take_entry_alternate_recognized_key() {
    let mut entry = Some((Content::Str("another_key"), Content::U32(84)));
    let recognized = ["recognized_key", "another_key"];
    flat_map_take_entry(&mut entry, &recognized);
}

#[test]
fn test_flat_map_take_entry_recognized_key_with_different_value() {
    let mut entry = Some((Content::Str("recognized_key"), Content::I32(-7)));
    let recognized = ["recognized_key"];
    flat_map_take_entry(&mut entry, &recognized);
}

