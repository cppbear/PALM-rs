// Answer 0

#[test]
fn test_next_value_seed_with_number() {
    let value = Value::Number(Number::from_f64(3.14).unwrap());
    let mut deserializer = MapDeserializer {
        iter: Vec::new().into_iter(),
        value: Some(value),
    };
    let seed = // create a suitable seed for Number;
    deserializer.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_with_bool() {
    let value = Value::Bool(true);
    let mut deserializer = MapDeserializer {
        iter: Vec::new().into_iter(),
        value: Some(value),
    };
    let seed = // create a suitable seed for Bool;
    deserializer.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_with_string() {
    let value = Value::String(String::from("test"));
    let mut deserializer = MapDeserializer {
        iter: Vec::new().into_iter(),
        value: Some(value),
    };
    let seed = // create a suitable seed for String;
    deserializer.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_with_array() {
    let value = Value::Array(vec![Value::Bool(false)]);
    let mut deserializer = MapDeserializer {
        iter: Vec::new().into_iter(),
        value: Some(value),
    };
    let seed = // create a suitable seed for Array;
    deserializer.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_with_object() {
    let value = Value::Object(Map::new());
    let mut deserializer = MapDeserializer {
        iter: Vec::new().into_iter(),
        value: Some(value),
    };
    let seed = // create a suitable seed for Object;
    deserializer.next_value_seed(seed);
}

