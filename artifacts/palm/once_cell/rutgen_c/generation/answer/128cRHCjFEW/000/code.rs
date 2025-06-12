// Answer 0

#[test]
fn test_set_success() {
    let once_bool = OnceBool::new();
    let result = once_bool.set(true);
    assert!(result.is_ok());
}

#[test]
fn test_set_failure() {
    let once_bool = OnceBool::new();
    let _ = once_bool.set(true);
    let result = once_bool.set(false);
    assert!(result.is_err());
}

#[test]
fn test_set_boundary_condition() {
    let once_bool = OnceBool::new();
    
    // Test setting to true
    assert!(once_bool.set(true).is_ok());
    
    // Test setting again to false should fail
    assert!(once_bool.set(false).is_err());
    
    // Test setting to true should not be allowed after that
    let result = once_bool.set(true);
    assert!(result.is_err());
}

