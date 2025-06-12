// Answer 0

#[test]
fn test_expecting() {
    struct ByteArray;

    impl std::fmt::Debug for ByteArray {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("byte array")
        }
    }

    let byte_array = ByteArray;
    let mut formatter = std::fmt::Formatter::default();
    let result = byte_array.fmt(&mut formatter);

    assert!(result.is_ok());
}

