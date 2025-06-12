// Answer 0

#[test]
fn test_builder_default_instance() {
    let builder = Builder::new();
}

#[test]
fn test_builder_with_http_scheme() {
    let builder = Builder::new().scheme("http");
}

#[test]
fn test_builder_with_https_scheme() {
    let builder = Builder::new().scheme("https");
}

#[test]
fn test_builder_with_example_com_authority() {
    let builder = Builder::new().authority("example.com");
}

#[test]
fn test_builder_with_hyper_rs_authority() {
    let builder = Builder::new().authority("hyper.rs");
}

#[test]
fn test_builder_with_root_path_and_query() {
    let builder = Builder::new().path_and_query("/");
}

#[test]
fn test_builder_with_path_to_resource() {
    let builder = Builder::new().path_and_query("/path/to/resource");
}

#[test]
fn test_builder_with_empty_path_and_query() {
    let builder = Builder::new().path_and_query("?");
}

