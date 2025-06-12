// Answer 0

#[test]
fn test_builder_empty() {
    let builder = Uri::builder();
    let _ = builder.build();
}

#[test]
fn test_builder_maximum_scheme_length() {
    let builder = Uri::builder().scheme("https").authority("example.com").path_and_query("/path");
    let _ = builder.build();
}

#[test]
fn test_builder_maximum_authority_length() {
    let builder = Uri::builder().scheme("http").authority("a".repeat(255).as_str()).path_and_query("/path");
    let _ = builder.build();
}

#[test]
fn test_builder_zero_length_path_and_query() {
    let builder = Uri::builder().scheme("http").authority("example.com").path_and_query("");
    let _ = builder.build();
}

#[test]
fn test_builder_large_path_and_query() {
    let long_path_and_query = "/".to_owned() + &"a".repeat(65534 - 1);
    let builder = Uri::builder().scheme("http").authority("example.com").path_and_query(long_path_and_query.as_str());
    let _ = builder.build();
}

#[test]
fn test_builder_with_valid_scheme_authority_path() {
    let builder = Uri::builder().scheme("ftp").authority("ftp.example.com").path_and_query("/files");
    let _ = builder.build();
}

#[test]
fn test_builder_with_special_characters_in_scheme() {
    let builder = Uri::builder().scheme("http+example").authority("example.com").path_and_query("/path");
    let _ = builder.build();
}

#[test]
#[should_panic]
fn test_builder_with_invalid_scheme() {
    let builder = Uri::builder().scheme("").authority("example.com").path_and_query("/path");
    let _ = builder.build();
}

#[test]
#[should_panic]
fn test_builder_with_invalid_authority() {
    let builder = Uri::builder().scheme("http").authority("").path_and_query("/path");
    let _ = builder.build();
}

#[test]
#[should_panic]
fn test_builder_with_excessive_length_scheme() {
    let long_scheme = "a".repeat(65);
    let builder = Uri::builder().scheme(long_scheme.as_str()).authority("example.com").path_and_query("/path");
    let _ = builder.build();
}

#[test]
#[should_panic]
fn test_builder_with_excessive_length_authority() {
    let long_authority = "a".repeat(256);
    let builder = Uri::builder().scheme("http").authority(long_authority.as_str()).path_and_query("/path");
    let _ = builder.build();
}

