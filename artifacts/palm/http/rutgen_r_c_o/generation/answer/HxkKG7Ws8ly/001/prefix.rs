// Answer 0

#[test]
fn test_extensions_ref_none_on_error() {
    let builder = Builder {
        inner: Err(Error { inner: ErrorKind::SomeError }),
    };
    let extensions = builder.extensions_ref();
}

#[test]
fn test_extensions_ref_some_with_string_extension() {
    let mut builder = Builder::new();
    builder = builder.extension("My Extension");
    builder = builder.extension(5u32);
    let parts = Parts {
        extensions: Extensions { map: None },
        ..Default::default()
    };
    builder.inner = Ok(parts);
    let extensions = builder.extensions_ref().unwrap();
}

#[test]
fn test_extensions_ref_some_with_integer_extension() {
    let mut builder = Builder::new();
    builder = builder.extension(10u32);
    let parts = Parts {
        extensions: Extensions { map: None },
        ..Default::default()
    };
    builder.inner = Ok(parts);
    let extensions = builder.extensions_ref().unwrap();
}

#[test]
fn test_extensions_ref_some_with_float_extension() {
    let mut builder = Builder::new();
    builder = builder.extension(3.14_f64);
    let parts = Parts {
        extensions: Extensions { map: None },
        ..Default::default()
    };
    builder.inner = Ok(parts);
    let extensions = builder.extensions_ref().unwrap();
}

#[test]
fn test_extensions_ref_some_with_custom_type() {
    struct CustomType;
    
    let mut builder = Builder::new();
    builder = builder.extension(CustomType);
    let parts = Parts {
        extensions: Extensions { map: None },
        ..Default::default()
    };
    builder.inner = Ok(parts);
    let extensions = builder.extensions_ref().unwrap();
}

#[test]
fn test_extensions_ref_multiple_extensions() {
    let mut builder = Builder::new();
    for i in 1..=10 {
        builder = builder.extension(format!("Extension {}", i).as_str());
    }
    let parts = Parts {
        extensions: Extensions { map: None },
        ..Default::default()
    };
    builder.inner = Ok(parts);
    let extensions = builder.extensions_ref().unwrap();
}

