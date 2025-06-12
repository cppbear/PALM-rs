// Answer 0

#[test]
fn test_status_with_integer() {
    struct DummyExtensions;
    let builder = Builder::new();
    let response_builder = builder.status(200);
    
    let result = response_builder.inner;
    assert!(result.is_ok());
    let parts = result.unwrap();
    assert_eq!(parts.status, StatusCode::OK);
}

#[test]
fn test_status_with_non_integer() {
    struct DummyError;
    impl TryInto<StatusCode> for &'static str {
        type Error = DummyError;
        fn try_into(self) -> Result<StatusCode, Self::Error> {
            Err(DummyError) // simulate an error
        }
    }
    
    struct DummyExtensions;
    let builder = Builder::new();
    let response_builder = builder.status("Invalid Status");
    
    let result = response_builder.inner;
    assert!(result.is_err());
}

#[test]
fn test_status_with_zero() {
    struct DummyExtensions;
    let builder = Builder::new();
    let response_builder = builder.status(0);
    
    let result = response_builder.inner;
    assert!(result.is_ok());
    let parts = result.unwrap();
    assert_eq!(parts.status, StatusCode::from_u16(0).unwrap());
}

#[test]
fn test_status_with_min_valid_value() {
    struct DummyExtensions;
    let builder = Builder::new();
    let response_builder = builder.status(100);
    
    let result = response_builder.inner;
    assert!(result.is_ok());
    let parts = result.unwrap();
    assert_eq!(parts.status, StatusCode::from_u16(100).unwrap());
}

#[test]
fn test_status_with_max_valid_value() {
    struct DummyExtensions;
    let builder = Builder::new();
    let response_builder = builder.status(599);
    
    let result = response_builder.inner;
    assert!(result.is_ok());
    let parts = result.unwrap();
    assert_eq!(parts.status, StatusCode::from_u16(599).unwrap());
}

#[test]
#[should_panic(expected = "Invalid status code")]
fn test_status_with_out_of_bounds_value() {
    struct DummyError;
    impl TryInto<StatusCode> for i32 {
        type Error = DummyError;
        fn try_into(self) -> Result<StatusCode, Self::Error> {
            if self < 100 || self > 599 {
                panic!("Invalid status code");
            }
            StatusCode::from_u16(self as u16).map_err(|_| DummyError)
        }
    }
    
    let builder = Builder::new();
    let _ = builder.status(600); // This should trigger the panic.
}

