// Answer 0

#[test]
fn test_insert_i32_none() {
    let mut ext = Extensions::new();
    let result = ext.insert(42i32);
}

#[test]
fn test_insert_u8_none() {
    let mut ext = Extensions::new();
    let result = ext.insert(255u8);
}

#[test]
fn test_insert_i32_replacement() {
    let mut ext = Extensions::new();
    let _ = ext.insert(10i32);
    let result = ext.insert(20i32);
}

#[test]
fn test_insert_u8_replacement() {
    let mut ext = Extensions::new();
    let _ = ext.insert(100u8);
    let result = ext.insert(50u8);
}

#[test]
fn test_insert_into_empty_extensions() {
    let mut ext = Extensions::new();
    let result = ext.insert(vec![1, 2, 3]);
}

#[test]
fn test_insert_and_get() {
    let mut ext = Extensions::new();
    let _ = ext.insert(25i32);
    let result = ext.get::<i32>();
}

#[test]
fn test_insert_and_get_mut() {
    let mut ext = Extensions::new();
    let _ = ext.insert(5usize);
    let result = ext.get_mut::<usize>();
}

#[test]
fn test_insert_default_case() {
    let mut ext = Extensions::new();
    let result = ext.get_or_insert_default::<String>();
}

#[test]
fn test_insert_empty_collection() {
    let mut ext = Extensions::new();
    let result = ext.insert(vec![]);
}

#[test]
fn test_insert_full_u8_range() {
    let mut ext = Extensions::new();
    for i in 0..=255 {
        let _ = ext.insert(i);
    }
}

