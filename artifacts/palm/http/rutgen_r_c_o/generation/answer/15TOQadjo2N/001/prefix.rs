// Answer 0

#[test]
fn test_get_or_insert_i32() {
    let mut ext = Extensions::new();
    *ext.get_or_insert(5i32) += 10;
}

#[test]
fn test_get_or_insert_f64() {
    let mut ext = Extensions::new();
    *ext.get_or_insert(3.14f64) += 1.86;
}

#[test]
fn test_get_or_insert_string() {
    let mut ext = Extensions::new();
    let val = String::from("Hello");
    ext.get_or_insert(val.clone());
}

#[test]
fn test_get_or_insert_custom_struct() {
    #[derive(Clone, Default)]
    struct MyStruct {
        value: i32,
    }
    
    let mut ext = Extensions::new();
    *ext.get_or_insert(MyStruct::default()) += MyStruct { value: 10 };
}

#[test]
fn test_get_or_insert_empty_string() {
    let mut ext = Extensions::new();
    let empty_val = String::new();
    ext.get_or_insert(empty_val.clone());
}

#[test]
fn test_get_or_insert_large_integer() {
    let mut ext = Extensions::new();
    *ext.get_or_insert(i32::max_value()) += 1;
}

#[test]
fn test_get_or_insert_negative_integer() {
    let mut ext = Extensions::new();
    *ext.get_or_insert(-10i32) += 5;
}

#[test]
fn test_get_or_insert_large_float() {
    let mut ext = Extensions::new();
    *ext.get_or_insert(1.7976931348623157E+308f64) += 0.1;
}

#[test]
fn test_get_or_insert_small_float() {
    let mut ext = Extensions::new();
    *ext.get_or_insert(-1.7976931348623157E+308f64) += 0.2;
}

