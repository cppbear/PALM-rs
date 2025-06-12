// Answer 0

#[test]
fn test_new_extensions() {
    let extensions = http::new();
    assert_eq!(extensions.map, None);
}

