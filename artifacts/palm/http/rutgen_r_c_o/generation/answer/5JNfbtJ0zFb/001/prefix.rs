// Answer 0

#[test]
fn test_extensions_ref_with_valid_extensions() {
    let mut req = Builder::new()
        .extension("My Extension")
        .extension(5u32);
    let extensions = req.extensions_ref().unwrap();
}

#[test]
fn test_extensions_ref_with_empty_extensions() {
    let req = Builder::new();
    let extensions = req.extensions_ref();
}

#[test]
fn test_extensions_ref_with_invalid_non_clone_extension() {
    struct NonClone;
    let req = Builder::new()
        .extension(NonClone);
}

#[test]
fn test_extensions_ref_with_multiple_valid_extensions() {
    let mut req = Builder::new()
        .extension("First Extension")
        .extension(10u32)
        .extension("Second Extension");
    let extensions = req.extensions_ref().unwrap();
}

#[test]
fn test_extensions_ref_with_single_numeric_extension() {
    let mut req = Builder::new()
        .extension(100u64);
    let extensions = req.extensions_ref().unwrap();
}

#[test]
fn test_extensions_ref_with_mixed_extension_types() {
    let mut req = Builder::new()
        .extension("String Extension")
        .extension(3.14f64);
    let extensions = req.extensions_ref().unwrap();
}

#[test]
#[should_panic]
fn test_extensions_ref_with_builder_error() {
    let req = Builder::new()
        .method("INVALID_METHOD")
        .extension("My Extension");
    let extensions = req.extensions_ref();
}

