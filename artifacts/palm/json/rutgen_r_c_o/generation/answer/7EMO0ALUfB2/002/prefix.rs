// Answer 0

#[test]
fn test_serialize_value_valid_key_and_value() {
    let mut map = Map::new();
    let mut serialize_map = SerializeMap::Map { map, next_key: Some(String::from("valid_key")) };
    let valid_value = String::from("valid_value");
    serialize_map.serialize_value(&valid_value).unwrap();
}

#[test]
fn test_serialize_value_multiple_entries() {
    let mut map = Map::new();
    let mut serialize_map = SerializeMap::Map { map, next_key: Some(String::from("first_key")) };
    let first_value = String::from("first_value");
    serialize_map.serialize_value(&first_value).unwrap();

    serialize_map.next_key = Some(String::from("second_key"));
    let second_value = String::from("second_value");
    serialize_map.serialize_value(&second_value).unwrap();
}

#[test]
fn test_serialize_value_with_different_types() {
    let mut map = Map::new();
    let mut serialize_map = SerializeMap::Map { map, next_key: Some(String::from("key_string")) };
    let string_value = String::from("some_string");
    serialize_map.serialize_value(&string_value).unwrap();

    serialize_map.next_key = Some(String::from("key_bool"));
    let bool_value = true;
    serialize_map.serialize_value(&bool_value).unwrap();

    serialize_map.next_key = Some(String::from("key_number"));
    let number_value = 42;
    serialize_map.serialize_value(&number_value).unwrap();
}

#[test]
#[should_panic(expected = "serialize_value called before serialize_key")]
fn test_serialize_value_panics_before_key() {
    let mut map = Map::new();
    let mut serialize_map = SerializeMap::Map { map, next_key: None };
    let valid_value = String::from("valid_value");
    serialize_map.serialize_value(&valid_value);
}

