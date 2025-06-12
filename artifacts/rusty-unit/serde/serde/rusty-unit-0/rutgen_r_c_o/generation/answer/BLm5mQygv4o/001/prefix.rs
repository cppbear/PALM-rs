// Answer 0

#[test]
fn test_unit_variant_success() {
    struct MockMapAccessOk;

    impl<'de> MapAccess<'de> for MockMapAccessOk {
        type Error = Error;

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Ok(())
        }
    }

    let map = MockMapAccessOk {};
    let variant_access = MapAsEnum { map };

    let _ = variant_access.unit_variant();
}

#[test]
#[should_panic]
fn test_unit_variant_error() {
    struct MockMapAccessErr;

    impl<'de> MapAccess<'de> for MockMapAccessErr {
        type Error = Error;

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(Error)
        }
    }

    let map = MockMapAccessErr {};
    let variant_access = MapAsEnum { map };

    let _ = variant_access.unit_variant();
}

