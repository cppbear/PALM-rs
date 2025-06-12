// Answer 0

#[test]
fn test_serialize_none_err() {
    struct Dummy;

    impl Dummy {
        fn serialize_none(self) -> Result<String, String> {
            Err(key_must_be_a_string())
        }
    }

    let dummy = Dummy;
    let result = dummy.serialize_none();
    assert!(result.is_err());
}

