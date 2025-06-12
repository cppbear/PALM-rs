// Answer 0

#[test]
fn test_extensions_mut_success() {
    let mut req = Builder::new()
        .method("GET")
        .uri("/")
        .version(Version::HTTP_11)
        .extension("My Extension");
    let extensions = req.extensions_mut().unwrap();
    extensions.insert("Some Value");
}

#[test]
fn test_extensions_mut_with_numeric_extension() {
    let mut req = Builder::new()
        .method("POST")
        .uri("/path")
        .version(Version::HTTP_2)
        .extension(5u32);
    let extensions = req.extensions_mut().unwrap();
    extensions.insert(10u32);
}

#[test]
fn test_extensions_mut_with_float_extension() {
    let mut req = Builder::new()
        .method("PUT")
        .uri("/path?query=value")
        .version(Version::HTTP_10)
        .extension(3.14f64);
    let extensions = req.extensions_mut().unwrap();
    extensions.insert(1.59f64);
}

#[test]
fn test_extensions_mut_with_none_extension() {
    let mut req = Builder::new()
        .method("DELETE")
        .uri("/")
        .version(Version::HTTP_11);
    let extensions = req.extensions_mut();
    assert!(extensions.is_none());
}

#[test]
fn test_extensions_mut_after_header() {
    let mut req = Builder::new()
        .method("PATCH")
        .uri("/path")
        .version(Version::HTTP_2)
        .header("Content-Type", "application/json")
        .extension("My Extension");
    let extensions = req.extensions_mut().unwrap();
    extensions.insert("Another Value");
}

