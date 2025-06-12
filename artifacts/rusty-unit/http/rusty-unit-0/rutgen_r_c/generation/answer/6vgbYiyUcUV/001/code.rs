// Answer 0

#[test]
fn test_is_safe_with_post_method() {
    struct TestMethod(Method);

    let method = TestMethod(Method(Method::Inner::Post));
    assert_eq!(method.0.is_safe(), false);
}

#[test]
fn test_is_safe_with_put_method() {
    struct TestMethod(Method);

    let method = TestMethod(Method(Method::Inner::Put));
    assert_eq!(method.0.is_safe(), false);
}

#[test]
fn test_is_safe_with_delete_method() {
    struct TestMethod(Method);

    let method = TestMethod(Method(Method::Inner::Delete));
    assert_eq!(method.0.is_safe(), false);
}

#[test]
fn test_is_safe_with_patch_method() {
    struct TestMethod(Method);

    let method = TestMethod(Method(Method::Inner::Patch));
    assert_eq!(method.0.is_safe(), false);
}

#[test]
fn test_is_safe_with_trace_method() {
    struct TestMethod(Method);

    let method = TestMethod(Method(Method::Inner::Trace));
    assert_eq!(method.0.is_safe(), false);
}

#[test]
fn test_is_safe_with_head_method() {
    struct TestMethod(Method);

    let method = TestMethod(Method(Method::Inner::Head));
    assert_eq!(method.0.is_safe(), false);
}

#[test]
fn test_is_safe_with_options_method() {
    struct TestMethod(Method);

    let method = TestMethod(Method(Method::Inner::Options));
    assert_eq!(method.0.is_safe(), false);
}

