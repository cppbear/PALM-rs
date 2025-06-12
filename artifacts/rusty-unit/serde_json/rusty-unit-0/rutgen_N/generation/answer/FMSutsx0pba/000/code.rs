// Answer 0

#[test]
fn test_end_with_map() {
    struct TestMap;

    impl ser::SerializeMap for TestMap {
        fn end(self) -> Result<()> {
            Ok(())
        }
    }

    enum Compound {
        Map(TestMap),
        #[cfg(feature = "arbitrary_precision")]
        Number,
        #[cfg(feature = "raw_value")]
        RawValue,
    }

    let map = Compound::Map(TestMap);
    let result = map.end();
    assert!(result.is_ok());
}

#[test]
#[cfg(feature = "arbitrary_precision")]
fn test_end_with_number() {
    enum Compound {
        Map,
        #[cfg(feature = "arbitrary_precision")]
        Number,
        #[cfg(feature = "raw_value")]
        RawValue,
    }

    let number = Compound::Number;
    let result = number.end();
    assert!(result.is_ok());
}

#[test]
#[cfg(feature = "raw_value")]
fn test_end_with_raw_value() {
    enum Compound {
        Map,
        #[cfg(feature = "arbitrary_precision")]
        Number,
        #[cfg(feature = "raw_value")]
        RawValue,
    }

    let raw_value = Compound::RawValue;
    let result = raw_value.end();
    assert!(result.is_ok());
}

