// Answer 0

#[test]
fn test_extensions_mut_valid() {
    let mut builder = Builder::new().status(StatusCode::OK).version(Version::HTTP_11);
    let mut response = builder.extension("Initial Extension").body("Response Body").unwrap();
    let extensions = response.extensions_mut().unwrap();
    extensions.insert(42u32);
}

#[test]
fn test_extensions_mut_empty() {
    let mut builder = Builder::new().status(StatusCode::OK).version(Version::HTTP_11);
    let mut response = builder.body("Response Body").unwrap();
    let extensions = response.extensions_mut();
    assert!(extensions.is_none());
}

#[test]
fn test_extensions_mut_maximum_entries() {
    let mut builder = Builder::new().status(StatusCode::OK).version(Version::HTTP_11);
    let mut response = builder.body("Response Body").unwrap();
    let extensions = response.extensions_mut().unwrap();
    for i in 0..100 {
        extensions.insert(i);
    }
}

#[test]
fn test_extensions_mut_key_value_length() {
    let mut builder = Builder::new().status(StatusCode::OK).version(Version::HTTP_11);
    let mut response = builder.extension("key_with_max_length_64_characters_abcdefghijklmnopqrstuvwx").body("Response Body").unwrap();
    let extensions = response.extensions_mut().unwrap();
    extensions.insert("value_with_max_length_256_characters_abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghij1234567890");
}

#[should_panic]
fn test_extensions_mut_invalid_state() {
    let builder = Builder::new(); 
    let response = builder.body("Response Body").unwrap();
    let extensions = response.extensions_mut(); 
    assert!(extensions.is_none());
}

