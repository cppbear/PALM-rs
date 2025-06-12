// Answer 0

#[test]
fn test_builder_new() {
    let builder = Builder::new();
    assert!(builder.parts.is_ok());
}

#[test]
fn test_builder_scheme() {
    let builder = Builder::new();
    let builder_with_scheme = builder.scheme("https");
    assert!(builder_with_scheme.parts.is_ok());
}

#[test]
fn test_builder_authority() {
    let builder = Builder::new();
    let builder_with_authority = builder.authority("hyper.rs");
    assert!(builder_with_authority.parts.is_ok());
}

#[test]
fn test_builder_path_and_query() {
    let builder = Builder::new();
    let builder_with_path_and_query = builder.path_and_query("/");
    assert!(builder_with_path_and_query.parts.is_ok());
}

#[test]
#[should_panic]
fn test_builder_invalid_scheme() {
    let builder = Builder::new();
    let _ = builder.scheme(123); // Assuming 123 cannot convert to Scheme
}

#[test]
#[should_panic]
fn test_builder_invalid_authority() {
    let builder = Builder::new();
    let _ = builder.authority(456); // Assuming 456 cannot convert to Authority
}

#[test]
#[should_panic]
fn test_builder_invalid_path_and_query() {
    let builder = Builder::new();
    let _ = builder.path_and_query(789); // Assuming 789 cannot convert to PathAndQuery
}

