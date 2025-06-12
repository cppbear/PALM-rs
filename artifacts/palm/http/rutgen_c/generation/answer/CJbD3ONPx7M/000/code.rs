// Answer 0

#[test]
fn test_fmt_path_and_query() {
    struct TestByteStr {
        bytes: Bytes,
    }

    impl fmt::Display for TestByteStr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", String::from_utf8_lossy(&self.bytes))
        }
    }

    let path_and_query = PathAndQuery {
        data: TestByteStr { bytes: Bytes::from_static(b"/example/path") },
        query: 0,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", path_and_query);
    assert!(result.is_ok());
    assert_eq!(output, "/example/path");
}

