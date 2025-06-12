// Answer 0

#[test]
fn test_fmt() {
    struct Header;

    impl std::fmt::Display for Header {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("failed to convert header to a str")
        }
    }

    let header = Header;
    let result = format!("{}", header);
    assert_eq!(result, "failed to convert header to a str");
}

