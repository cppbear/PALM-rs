// Answer 0

#[test]
fn test_builder_new() {
    // Create a new default instance of Builder
    let builder = http::uri::Builder::new();

    // Struct to assert the default state of Builder
    struct ExpectedBuilder {
        scheme: Option<String>,
        authority: Option<String>,
        path_and_query: Option<String>,
    }

    // Expected default state of the Builder
    let expected = ExpectedBuilder {
        scheme: None,
        authority: None,
        path_and_query: None,
    };

    // Assert that the default values are correctly initialized
    assert_eq!(builder.scheme, expected.scheme);
    assert_eq!(builder.authority, expected.authority);
    assert_eq!(builder.path_and_query, expected.path_and_query);
}

// Testing behavior when using the builder to ensure proper initialization
#[test]
fn test_builder_initialize_and_build() {
    // Create a new builder instance
    let builder = http::uri::Builder::new()
        .scheme("https")
        .authority("hyper.rs")
        .path_and_query("/");

    // Attempting to build should succeed without any panic
    let uri = builder.build().unwrap();

    // Assert the values match what has been provided
    assert_eq!(uri.scheme_str(), Some("https"));
    assert_eq!(uri.authority().to_string(), "hyper.rs");
    assert_eq!(uri.path(), "/");
}

