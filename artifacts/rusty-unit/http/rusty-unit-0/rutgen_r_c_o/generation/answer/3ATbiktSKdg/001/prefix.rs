// Answer 0

#[test]
fn test_method_get() {
    let builder = Builder::new();
    let _ = builder.method("GET");
}

#[test]
fn test_method_post() {
    let builder = Builder::new();
    let _ = builder.method("POST");
}

#[test]
fn test_method_put() {
    let builder = Builder::new();
    let _ = builder.method("PUT");
}

#[test]
fn test_method_delete() {
    let builder = Builder::new();
    let _ = builder.method("DELETE");
}

#[test]
fn test_method_patch() {
    let builder = Builder::new();
    let _ = builder.method("PATCH");
}

#[test]
fn test_method_options() {
    let builder = Builder::new();
    let _ = builder.method("OPTIONS");
}

#[test]
fn test_method_head() {
    let builder = Builder::new();
    let _ = builder.method("HEAD");
}

#[test]
fn test_method_trace() {
    let builder = Builder::new();
    let _ = builder.method("TRACE");
}

#[test]
fn test_method_connect() {
    let builder = Builder::new();
    let _ = builder.method("CONNECT");
}

#[test]
fn test_method_empty_string() {
    let builder = Builder::new();
    let _ = builder.method("");
}

#[should_panic]
#[test]
fn test_method_invalid_method() {
    let builder = Builder::new();
    let _ = builder.method("InvalidMethod");
}

