// Answer 0

#[test]
fn test_get_mut_successful() {
    let mut ext = Extensions::new();
    ext.insert(String::from("Test String"));
    
    let value_mut = ext.get_mut::<String>().expect("Failed to get mutable reference");
    value_mut.push_str(" Modified");
    
    assert_eq!(ext.get::<String>().unwrap(), "Test String Modified");
}

#[test]
fn test_get_mut_no_entry() {
    let mut ext = Extensions::new();
    
    let value_mut: Option<&mut String> = ext.get_mut();
    assert!(value_mut.is_none());
}

#[test]
fn test_get_mut_with_different_type() {
    let mut ext = Extensions::new();
    ext.insert(42);
    
    let value_mut: Option<&mut String> = ext.get_mut();
    assert!(value_mut.is_none());
}

#[test]
#[should_panic(expected = "Failed to get mutable reference")]
fn test_get_mut_panic() {
    let mut ext = Extensions::new();
    ext.insert(String::from("Hello"));
    
    // This will not panic but return None so we won't actually reach the panic.
    let _value_mut: &mut i32 = ext.get_mut().expect("Should panic"); 
}

