// Answer 0

#[test]
fn test_is_f64_with_positive_f64_values() {
    let positive_values = vec![
        0.0, 0.1, 1.0, 1.5, 2.0, 2.5, 3.0, 10.0, 100.0, f64::MAX, f64::INFINITY,
    ];
    
    for &val in &positive_values {
        let number = Number::from_f64(val).unwrap();
        let value = Value::Number(number);
        value.is_f64();
    }
}

#[test]
fn test_is_f64_with_negative_f64_values() {
    let negative_values = vec![
        -0.1, -1.0, -1.5, -2.0, -2.5, -3.0, f64::MIN,
    ];
    
    for &val in &negative_values {
        let number = Number::from_f64(val).unwrap();
        let value = Value::Number(number);
        value.is_f64();
    }
}

#[test]
fn test_is_f64_with_special_f64_values() {
    let special_values = vec![
        f64::NAN,
    ];
    
    for &val in &special_values {
        let number = Number::from_f64(val).unwrap();
        let value = Value::Number(number);
        value.is_f64();
    }
}

#[test]
fn test_is_f64_with_non_f64_values() {
    let int_values = vec![64, -64];
    let string_value = String::from("a string");

    for &val in &int_values {
        let number = Number::from_i64(val).unwrap();
        let value = Value::Number(number);
        value.is_f64();
    }
    
    let string_value = Value::String(string_value);
    string_value.is_f64();
    
    let null_value = Value::Null;
    null_value.is_f64();
    
    let bool_value = Value::Bool(true);
    bool_value.is_f64();
    
    let array_value = Value::Array(vec![]);
    array_value.is_f64();
    
    let object_value = Value::Object(Map::new());
    object_value.is_f64();
}

