// Answer 0

#[test]
fn test_status_mut() {
    // Create a Response with a default body
    let mut response: Response<()> = Response::new(());
    
    // Set the status using the mutable reference
    *response.status_mut() = StatusCode(NonZeroU16::new(201).unwrap()); // 201 for StatusCode::CREATED

    // Assert that the status was set correctly
    assert_eq!(response.status(), StatusCode(NonZeroU16::new(201).unwrap()));
}

#[test]
fn test_status_mut_initialization() {
    // Create a Response with a different default body
    let mut response: Response<()> = Response::new(());

    // Initialize the status with a different value
    *response.status_mut() = StatusCode(NonZeroU16::new(400).unwrap()); // 400 for Bad Request

    // Assert that it reflects the new status
    assert_eq!(response.status(), StatusCode(NonZeroU16::new(400).unwrap()));
}

#[test]
#[should_panic]
fn test_status_mut_panic_on_invalid_status() {
    // Create a Response with a default body
    let mut response: Response<()> = Response::new(());
    
    // This assumes a panic would arise from using an invalid non-zero status (hypothetical)
    *response.status_mut() = StatusCode(NonZeroU16::new(0).unwrap()); // Invalid to set
}

