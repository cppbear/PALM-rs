// Answer 0

#[test]
fn test_extension_adds_extension() {
    use std::any::Any;
    use http::*;

    // Create a request builder and add an extension
    let req = Request::builder()
        .extension("My Extension")
        .body(())
        .unwrap();

    // Check that the extension was added correctly
    assert_eq!(req.extensions().get::<&'static str>(), Some(&"My Extension"));
}

#[test]
fn test_extension_with_different_type() {
    use std::any::Any;
    use http::*;

    // Create a request builder and add an integer extension
    let req = Request::builder()
        .extension(42)
        .body(())
        .unwrap();

    // Check that the integer extension is returned correctly
    assert_eq!(req.extensions().get::<i32>(), Some(&42));
}

#[test]
fn test_extension_multiple_extensions() {
    use std::any::Any;
    use http::*;

    // Create a request builder and add multiple extensions
    let req = Request::builder()
        .extension("First Extension")
        .extension("Second Extension")
        .body(())
        .unwrap();

    // Check that both extensions are present
    assert_eq!(req.extensions().get::<&'static str>(), Some(&"Second Extension")); // Last added should be retrieved
}

#[test]
#[should_panic]
fn test_extension_no_body() {
    use http::*;
    
    // Create a request builder without a body, which should panic
    let _req = Request::builder()
        .extension("My Extension")
        .unwrap(); // The body method is not called, this should panic
}

