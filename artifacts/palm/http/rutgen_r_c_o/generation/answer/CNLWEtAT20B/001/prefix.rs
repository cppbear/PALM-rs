// Answer 0

#[test]
fn test_method_options_display() {
    let method = Method(Inner::Options);
    let _ = format!("{}", method);
}

#[test]
fn test_method_get_display() {
    let method = Method(Inner::Get);
    let _ = format!("{}", method);
}

#[test]
fn test_method_post_display() {
    let method = Method(Inner::Post);
    let _ = format!("{}", method);
}

#[test]
fn test_method_put_display() {
    let method = Method(Inner::Put);
    let _ = format!("{}", method);
}

#[test]
fn test_method_delete_display() {
    let method = Method(Inner::Delete);
    let _ = format!("{}", method);
}

#[test]
fn test_method_head_display() {
    let method = Method(Inner::Head);
    let _ = format!("{}", method);
}

#[test]
fn test_method_trace_display() {
    let method = Method(Inner::Trace);
    let _ = format!("{}", method);
}

#[test]
fn test_method_connect_display() {
    let method = Method(Inner::Connect);
    let _ = format!("{}", method);
}

#[test]
fn test_method_patch_display() {
    let method = Method(Inner::Patch);
    let _ = format!("{}", method);
}

#[test]
fn test_method_extension_inline_display() {
    let inline_extension = InlineExtension::from_str("extension").unwrap();
    let method = Method(Inner::ExtensionInline(inline_extension));
    let _ = format!("{}", method);
}

#[test]
fn test_method_extension_allocated_display() {
    let allocated_extension = AllocatedExtension::from_str("allocated").unwrap();
    let method = Method(Inner::ExtensionAllocated(allocated_extension));
    let _ = format!("{}", method);
}

