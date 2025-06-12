// Answer 0

#[test]
fn test_get_or_insert_default_i32() {
    let mut ext = Extensions::new();
    let value: &mut i32 = ext.get_or_insert_default::<i32>();
    *value += 10;
}

#[test]
fn test_get_or_insert_default_f64() {
    let mut ext = Extensions::new();
    let value: &mut f64 = ext.get_or_insert_default::<f64>();
    *value += 5.5;
}

#[test]
fn test_get_or_insert_default_string() {
    let mut ext = Extensions::new();
    let value: &mut String = ext.get_or_insert_default::<String>();
    value.push_str("Hello");
}

#[test]
fn test_get_or_insert_default_bool() {
    let mut ext = Extensions::new();
    let value: &mut bool = ext.get_or_insert_default::<bool>();
    *value = true;
}

#[test]
fn test_get_or_insert_default_edge_case_large() {
    let mut ext = Extensions::new();
    for i in 0..1000 {
        let value: &mut i32 = ext.get_or_insert_default::<i32>();
        *value += i;
    }
} 

#[test]
fn test_get_or_insert_default_edge_case_small() {
    let mut ext = Extensions::new();
    let value: &mut i32 = ext.get_or_insert_default::<i32>();
    *value -= 1;
}

