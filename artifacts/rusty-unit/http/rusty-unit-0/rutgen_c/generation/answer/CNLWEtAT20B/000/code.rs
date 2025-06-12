// Answer 0

#[test]
fn test_method_display_get() {
    struct InlineExtension;

    let method = Method(Inner::Get);
    let mut output = String::new();
    let result = method.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Get");
}

#[test]
fn test_method_display_post() {
    struct InlineExtension;

    let method = Method(Inner::Post);
    let mut output = String::new();
    let result = method.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Post");
}

#[test]
fn test_method_display_delete() {
    struct InlineExtension;

    let method = Method(Inner::Delete);
    let mut output = String::new();
    let result = method.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Delete");
}

#[test]
fn test_method_display_extension_inline() {
    struct InlineExtension;

    let method = Method(Inner::ExtensionInline(InlineExtension));
    let mut output = String::new();
    let result = method.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "ExtensionInline");
}

#[test]
fn test_method_display_extension_allocated() {
    struct AllocatedExtension;

    let method = Method(Inner::ExtensionAllocated(AllocatedExtension));
    let mut output = String::new();
    let result = method.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "ExtensionAllocated");
}

