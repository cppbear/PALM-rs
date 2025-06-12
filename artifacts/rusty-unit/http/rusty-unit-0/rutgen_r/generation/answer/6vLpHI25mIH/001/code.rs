// Answer 0

#[test]
fn test_extensions_mut_insert() {
    // Arrange
    let mut response: Response<()> = Response::default();

    // Act
    response.extensions_mut().insert("hello");

    // Assert
    assert_eq!(response.extensions().get(), Some(&"hello"));
}

#[test]
fn test_extensions_mut_default_state() {
    // Arrange
    let response: Response<()> = Response::default();

    // Act
    let extensions = response.extensions();

    // Assert
    assert!(extensions.is_empty());
}

#[test]
fn test_extensions_mut_multiple_inserts() {
    // Arrange
    let mut response: Response<()> = Response::default();

    // Act
    response.extensions_mut().insert("first");
    response.extensions_mut().insert("second");

    // Assert
    assert_eq!(response.extensions().get(), Some(&"second"));
}

#[test]
fn test_extensions_mut_insert_and_get() {
    // Arrange
    let mut response: Response<()> = Response::default();

    // Act
    response.extensions_mut().insert("test_value");

    // Assert
    assert_eq!(response.extensions().get(), Some(&"test_value"));
}

#[should_panic]
fn test_extensions_mut_panic_on_invalid_access() {
    // Arrange
    let mut response: Response<()> = Response::default();

    // Act
    {
        let _extensions = response.extensions_mut();
    }

    // Assert (Expecting panic as mut borrow is not held)
    let _ = response.extensions();
}

