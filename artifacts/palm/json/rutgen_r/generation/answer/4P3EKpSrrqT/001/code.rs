// Answer 0

#[test]
fn test_set_failed() {
    // Arrange
    let mut failed = false;
    let mut result = false;

    // Act
    {
        let mut context = &mut result;
        set_failed(&mut context, &mut failed);
    }

    // Assert
    assert!(failed);
}

#[test]
#[should_panic]
fn test_set_failed_panic() {
    // Arrange
    let mut failed: &mut bool = &mut false;

    // Act
    // This test intentionally uses a reference that will cause a panic
    set_failed(&mut false, failed);
}

