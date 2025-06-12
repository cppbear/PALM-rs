// Answer 0

#[test]
fn test_builder_with_valid_parts() {
    let uri = Uri::builder()
        .scheme("http")
        .authority("example.com")
        .path_and_query("/path/to/resource")
        .build()
        .unwrap();
}

#[test]
fn test_builder_without_scheme() {
    let result = Uri::builder()
        .authority("example.com")
        .path_and_query("/path/to/resource")
        .build();
}

#[test]
fn test_builder_without_authority() {
    let result = Uri::builder()
        .scheme("http")
        .path_and_query("/path/to/resource")
        .build();
}

#[test]
fn test_builder_without_path_and_query() {
    let result = Uri::builder()
        .scheme("http")
        .authority("example.com")
        .build();
}

#[test]
fn test_builder_with_invalid_scheme() {
    let result = Uri::builder()
        .scheme("!@#%/^")
        .authority("example.com")
        .path_and_query("/path/to/resource")
        .build();
}

#[test]
fn test_builder_with_missing_parts() {
    let result = Uri::builder()
        .build();
}

