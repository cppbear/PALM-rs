// Answer 0

#[test]
fn test_insert_int() {
    let mut ext = Extensions::new();
    
    // Insert an integer
    assert!(ext.insert(10i32).is_none());
    // Insert another integer
    assert!(ext.insert(20i32).is_none());
    // Replace the first integer
    assert_eq!(ext.insert(30i32), Some(10i32));
}

#[test]
fn test_insert_float() {
    let mut ext = Extensions::new();
    
    // Insert a float
    assert!(ext.insert(10.5f32).is_none());
    // Insert another float
    assert!(ext.insert(20.5f32).is_none());
    // Replace the first float
    assert_eq!(ext.insert(30.5f32), Some(10.5f32));
}

#[test]
fn test_insert_string() {
    let mut ext = Extensions::new();
    
    // Insert a string
    assert!(ext.insert("Hello".to_string()).is_none());
    // Insert another string
    assert!(ext.insert("World".to_string()).is_none());
    // Replace the first string
    assert_eq!(ext.insert("New String".to_string()), Some("Hello".to_string()));
}

#[test]
fn test_insert_multiple_types() {
    let mut ext = Extensions::new();
    
    // Insert different types
    assert!(ext.insert(42i32).is_none());
    assert!(ext.insert("Test".to_string()).is_none());
    assert!(ext.insert(3.14f64).is_none());
    
    // Replace int
    assert_eq!(ext.insert(99i32), Some(42i32));
    // Replace string
    assert_eq!(ext.insert("New Test".to_string()), Some("Test".to_string()));
    // Replace float
    assert_eq!(ext.insert(2.718f64), Some(3.14f64));
}

#[test]
#[should_panic]
fn test_insert_panic() {
    let mut ext = Extensions::new();
    // In a scenario where implementation causes panic, we try the insert operation
    // Since there isn't a specific panic condition documented, this is illustrative.
    ext.insert(std::mem::transmute::<_, &'static str>(888i32)); // Unsafe operation
}

