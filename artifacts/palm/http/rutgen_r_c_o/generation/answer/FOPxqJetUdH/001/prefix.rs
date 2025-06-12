// Answer 0

#[test]
fn test_len_empty() {
    let ext = Extensions::new();
    let _ = ext.len();
}

#[test]
fn test_len_after_insert_one() {
    let mut ext = Extensions::new();
    ext.insert(5i32);
    let _ = ext.len();
}

#[test]
fn test_len_after_insert_two() {
    let mut ext = Extensions::new();
    ext.insert(5i32);
    ext.insert(10i32);
    let _ = ext.len();
}

#[test]
fn test_len_after_insert_multiple() {
    let mut ext = Extensions::new();
    for i in 0..10 {
        ext.insert(i);
    }
    let _ = ext.len();
}

#[test]
fn test_len_after_insert_hundred() {
    let mut ext = Extensions::new();
    for i in 0..100 {
        ext.insert(i);
    }
    let _ = ext.len();
}

#[test]
fn test_len_after_insert_thousand() {
    let mut ext = Extensions::new();
    for i in 0..1000 {
        ext.insert(i);
    }
    let _ = ext.len();
}

