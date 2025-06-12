// Answer 0

#[test]
fn test_serialize_unit_panic() {
    struct Dummy;

    impl Dummy {
        fn serialize_unit(self) -> Result<String, String> {
            Err(key_must_be_a_string())
        }
    }

    let dummy = Dummy;
    let result = dummy.serialize_unit();
    assert_eq!(result, Err(key_must_be_a_string()));
}

fn key_must_be_a_string() -> String {
    "key must be a string".into()
}

