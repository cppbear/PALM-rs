// Answer 0

#[test]
fn test_get_or_insert_with_new_value() {
    let mut ext = Extensions::new();
    *ext.get_or_insert(1i32) += 2;
    assert_eq!(*ext.get::<i32>().unwrap(), 3);
}

#[test]
fn test_get_or_insert_existing_value() {
    let mut ext = Extensions::new();
    *ext.get_or_insert(1i32) += 2; // Insert 1i32 and modify it to 3
    *ext.get_or_insert(3i32) += 4; // This call should not insert a new value
    assert_eq!(*ext.get::<i32>().unwrap(), 3);
}

#[test]
fn test_get_or_insert_with_string() {
    let mut ext = Extensions::new();
    let inserted_ref = ext.get_or_insert("hello".to_string());
    assert_eq!(*inserted_ref, "hello");
    
    // Modify the value inserted
    inserted_ref.push_str(", world");
    assert_eq!(*ext.get::<String>().unwrap(), "hello, world");
}

#[test]
fn test_get_or_insert_default_value() {
    let mut ext = Extensions::new();
    let default_ref: &mut Vec<i32> = ext.get_or_insert_default();
    default_ref.push(1);
    
    assert_eq!(default_ref.len(), 1);
    assert_eq!(*ext.get::<Vec<i32>>().unwrap(), vec![1]);
}

