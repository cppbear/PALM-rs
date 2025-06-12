// Answer 0

#[test]
fn test_end_with_map() {
    struct SimpleMap;

    impl ser::SerializeMap for SimpleMap {
        fn end(self) -> Result<()> {
            Ok(())
        }
    }

    enum Compound {
        Map(SimpleMap),
        #[cfg(feature = "arbitrary_precision")]
        Number,
        #[cfg(feature = "raw_value")]
        RawValue,
    }

    let compound = Compound::Map(SimpleMap);
    let result = compound.end();
    assert!(result.is_ok());
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_end_with_number() {
    enum Compound {
        #[cfg(feature = "arbitrary_precision")]
        Number,
    }

    #[allow(unused_variables)]
    let compound = Compound::Number;
    let result = compound.end();
    assert!(result.is_ok());
}

#[cfg(feature = "raw_value")]
#[test]
fn test_end_with_raw_value() {
    enum Compound {
        #[cfg(feature = "raw_value")]
        RawValue,
    }

    #[allow(unused_variables)]
    let compound = Compound::RawValue;
    let result = compound.end();
    assert!(result.is_ok());
}

