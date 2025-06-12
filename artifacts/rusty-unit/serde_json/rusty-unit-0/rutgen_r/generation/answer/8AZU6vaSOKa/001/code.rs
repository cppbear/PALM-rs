// Answer 0

#[test]
fn test_is_i64_non_number() {
    use serde_json::Value;

    let v_null = Value::Null;
    let v_bool_true = Value::Bool(true);
    let v_bool_false = Value::Bool(false);
    let v_string = Value::String("not a number".to_string());
    
    assert!(!v_null.is_i64());
    assert!(!v_bool_true.is_i64());
    assert!(!v_bool_false.is_i64());
    assert!(!v_string.is_i64());
}

#[test]
fn test_is_i64_number_edge_cases() {
    use serde_json::json;

    // i64::MAX
    let v_max = json!(i64::MAX);
    assert!(v_max.is_i64());

    // i64::MIN
    let v_min = json!(i64::MIN);
    assert!(v_min.is_i64());

    // Just above i64::MAX (should be a non-i64)
    let v_above_max = json!(i64::MAX as u64 + 1);
    assert!(!v_above_max.is_i64());

    // Just below i64::MIN (should be a non-i64)
    let v_below_min = json!(i64::MIN as u64 - 1);
    assert!(!v_below_min.is_i64());

    // Decimal point (should be a non-i64)
    let v_decimal = json!(256.0);
    assert!(!v_decimal.is_i64());
}

