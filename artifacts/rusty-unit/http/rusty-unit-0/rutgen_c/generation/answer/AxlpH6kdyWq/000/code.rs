// Answer 0

#[test]
fn test_method_debug_options() {
    let method = Method(Inner::Options);
    let mut output = String::new();
    let result = method.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "OPTIONS");
}

#[test]
fn test_method_debug_get() {
    let method = Method(Inner::Get);
    let mut output = String::new();
    let result = method.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "GET");
}

#[test]
fn test_method_debug_post() {
    let method = Method(Inner::Post);
    let mut output = String::new();
    let result = method.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "POST");
}

#[test]
fn test_method_debug_put() {
    let method = Method(Inner::Put);
    let mut output = String::new();
    let result = method.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "PUT");
}

#[test]
fn test_method_debug_delete() {
    let method = Method(Inner::Delete);
    let mut output = String::new();
    let result = method.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "DELETE");
}

#[test]
fn test_method_debug_head() {
    let method = Method(Inner::Head);
    let mut output = String::new();
    let result = method.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "HEAD");
}

#[test]
fn test_method_debug_trace() {
    let method = Method(Inner::Trace);
    let mut output = String::new();
    let result = method.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "TRACE");
}

#[test]
fn test_method_debug_connect() {
    let method = Method(Inner::Connect);
    let mut output = String::new();
    let result = method.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "CONNECT");
}

#[test]
fn test_method_debug_patch() {
    let method = Method(Inner::Patch);
    let mut output = String::new();
    let result = method.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "PATCH");
}

