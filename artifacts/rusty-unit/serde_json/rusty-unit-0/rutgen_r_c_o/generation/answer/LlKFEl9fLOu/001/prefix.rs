// Answer 0

#[test]
fn test_index_mut_empty_map() {
    let mut map = Map { map: MapImpl::new() };
    let index = "";
    let _ = map.index_mut(index);
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_mut_non_existing_key() {
    let mut map = Map { map: MapImpl::new() };
    let index = "non_existing_key";
    let _ = map.index_mut(index);
}

#[test]
fn test_index_mut_existing_key() {
    let mut map = Map { map: MapImpl::new() };
    map.map.insert("existing_key".to_string(), Value::Bool(true));
    let index = "existing_key";
    let _ = map.index_mut(index);
}

