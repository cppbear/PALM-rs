// Answer 0

#[test]
fn test_or_insert_vacant() {
    use std::collections::BTreeMap;
    use serde_json::{Map, Value};

    let mut map: Map<String, Value> = Map::new();
    let key = "serde".to_string();
    
    let entry = Entry::Vacant(VacantEntry {
        vacant: map.entry(key.clone())
    });
    
    let result = entry.or_insert(Value::Number(serde_json::Number::from(12)));
    
    assert_eq!(*result, Value::Number(serde_json::Number::from(12)));
    assert_eq!(map.get(&key).unwrap(), &Value::Number(serde_json::Number::from(12)));
}

#[test]
fn test_or_insert_occupied() {
    use std::collections::BTreeMap;
    use serde_json::{Map, Value};

    let mut map: Map<String, Value> = Map::new();
    let key = "serde".to_string();
    map.insert(key.clone(), Value::Number(serde_json::Number::from(10)));

    let entry = Entry::Occupied(OccupiedEntry {
        occupied: map.entry(key.clone()).or_insert(Value::Null).into()
    });
    
    let result = entry.or_insert(Value::Number(serde_json::Number::from(12)));
    
    assert_eq!(*result, Value::Number(10)); // Should not replace the existing value
    assert_eq!(map.get(&key).unwrap(), &Value::Number(10)); // Value should remain the same
}

