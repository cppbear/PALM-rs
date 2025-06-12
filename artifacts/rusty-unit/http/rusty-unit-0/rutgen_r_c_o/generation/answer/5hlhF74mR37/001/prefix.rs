// Answer 0

#[test]
fn test_get_or_insert_with_i32_positive() {
    let mut ext = Extensions::new();
    let result = ext.get_or_insert_with(|| 5i32);
}

#[test]
fn test_get_or_insert_with_i32_negative() {
    let mut ext = Extensions::new();
    let result = ext.get_or_insert_with(|| -3i32);
}

#[test]
fn test_get_or_insert_with_i32_zero() {
    let mut ext = Extensions::new();
    let result = ext.get_or_insert_with(|| 0i32);
}

#[test]
fn test_get_or_insert_with_i32_large_positive() {
    let mut ext = Extensions::new();
    let result = ext.get_or_insert_with(|| 1_000_000i32);
}

#[test]
fn test_get_or_insert_with_i32_large_negative() {
    let mut ext = Extensions::new();
    let result = ext.get_or_insert_with(|| -1_000_000i32);
}

#[test]
fn test_get_or_insert_with_i32_boundary_positive() {
    let mut ext = Extensions::new();
    let result = ext.get_or_insert_with(|| 2_147_483_647i32);
}

#[test]
fn test_get_or_insert_with_i32_boundary_negative() {
    let mut ext = Extensions::new();
    let result = ext.get_or_insert_with(|| -2_147_483_648i32);
}

#[test]
fn test_get_or_insert_with_i32_multiple_inserts() {
    let mut ext = Extensions::new();
    let result1 = ext.get_or_insert_with(|| 10i32);
    let result2 = ext.get_or_insert_with(|| 20i32);
}

