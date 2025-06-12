// Answer 0

#[test]
fn test_get_mut() {
    use serde_json::json;
    use serde_json::map::{Map, Entry};

    let mut map = Map::new();
    map.insert("serde".to_owned(), json!([1, 2, 3]));

    match map.entry("serde") {
        Entry::Occupied(mut occupied) => {
            occupied.get_mut().as_array_mut().unwrap().push(json!(4));
        }
        Entry::Vacant(_) => unimplemented!(),
    }

    assert_eq!(map["serde"].as_array().unwrap(), &vec![json!(1), json!(2), json!(3), json!(4)]);
}

#[test]
fn test_get_mut_empty_entry() {
    use serde_json::json;
    use serde_json::map::{Map, Entry};

    let mut map = Map::new();

    match map.entry("missing") {
        Entry::Occupied(_) => unreachable!(),
        Entry::Vacant(vacant) => {
            // Since the entry is vacant, we can insert a value
            vacant.insert(json!(42));
        }
    }

    assert_eq!(map["missing"], json!(42));
}

#[test]
fn test_get_mut_with_existing_value() {
    use serde_json::json;
    use serde_json::map::{Map, Entry};

    let mut map = Map::new();
    map.insert("key".to_owned(), json!(100));

    match map.entry("key") {
        Entry::Occupied(mut occupied) => {
            *occupied.get_mut() = json!(200);
        }
        Entry::Vacant(_) => unimplemented!(),
    }

    assert_eq!(map["key"], json!(200));
}

