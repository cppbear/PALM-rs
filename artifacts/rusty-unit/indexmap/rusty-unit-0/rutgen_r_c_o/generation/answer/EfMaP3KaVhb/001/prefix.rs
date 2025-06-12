// Answer 0

#[test]
fn test_unit_value_debug_with_minimum_value() {
    let unit_value = UnitValue(1);
    let _ = format!("{:?}", unit_value);
}

#[test]
fn test_unit_value_debug_with_middle_value() {
    let unit_value = UnitValue(500);
    let _ = format!("{:?}", unit_value);
}

#[test]
fn test_unit_value_debug_with_maximum_value() {
    let unit_value = UnitValue(1000);
    let _ = format!("{:?}", unit_value);
}

#[test]
#[should_panic]
fn test_unit_value_debug_with_zero_value() {
    let unit_value = UnitValue(0);
    let _ = format!("{:?}", unit_value);
}

#[test]
#[should_panic]
fn test_unit_value_debug_with_negative_value() {
    let unit_value = UnitValue(-1);
    let _ = format!("{:?}", unit_value);
}

