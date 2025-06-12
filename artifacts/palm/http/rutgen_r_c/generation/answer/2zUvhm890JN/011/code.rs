// Answer 0

#[test]
fn test_as_str_methods() {
    let methods = [
        Method::OPTIONS,
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
        Method::HEAD,
        Method::TRACE,
        Method::CONNECT,
        Method::PATCH,
    ];

    let expected_outputs = [
        "OPTIONS",
        "GET",
        "POST",
        "PUT",
        "DELETE",
        "HEAD",
        "TRACE",
        "CONNECT",
        "PATCH",
    ];

    for (method, &expected) in methods.iter().zip(expected_outputs.iter()) {
        assert_eq!(method.as_str(), expected);
    }
}

#[test]
fn test_as_str_inline_extension() {
    struct TestExtension;
    
    impl TestExtension {
        fn create_inline_extension() -> InlineExtension {
            InlineExtension([b'T', b'E', b'S', b'T', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 4)
        }
    }
    
    let inline_extension = TestExtension::create_inline_extension();
    let method = Method(Inner::ExtensionInline(inline_extension));
    assert_eq!(method.as_str(), "TEST");
}

#[test]
fn test_as_str_allocated_extension() {
    struct TestAllocatedExtension;

    impl TestAllocatedExtension {
        fn create_allocated_extension() -> AllocatedExtension {
            AllocatedExtension(Box::from([b'T', b'E', b'S', b'T'] as [u8; 4]))
        }
    }

    let allocated_extension = TestAllocatedExtension::create_allocated_extension();
    let method = Method(Inner::ExtensionAllocated(allocated_extension));
    assert_eq!(method.as_str(), "TEST");
}

