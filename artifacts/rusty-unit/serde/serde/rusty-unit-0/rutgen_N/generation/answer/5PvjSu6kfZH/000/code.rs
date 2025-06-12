// Answer 0

#[test]
fn test_serialize_bytes_err() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(_: Unsupported) -> &'static str {
            "Bad type"
        }
    }

    let result = TestSerializer.serialize_bytes(&[1, 2, 3]);
    assert_eq!(result, Err("Bad type"));
}

