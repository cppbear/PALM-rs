// Answer 0

#[test]
fn test_serialize_i128_out_of_range() {
    use serde_json::Value;
    use std::convert::TryFrom;
    use std::i64;
    
    struct Serializer;

    impl Serializer {
        fn serialize_i128(self, value: i128) -> Result<Value, serde_json::Error> {
            #[cfg(feature = "arbitrary_precision")]
            {
                Ok(Value::Number(value.into()))
            }

            #[cfg(not(feature = "arbitrary_precision"))]
            {
                if let Ok(value) = u64::try_from(value) {
                    Ok(Value::Number(value.into()))
                } else if let Ok(value) = i64::try_from(value) {
                    Ok(Value::Number(value.into()))
                } else {
                    Err(serde_json::Error::syntax(serde_json::error::ErrorCode::NumberOutOfRange, 0, 0))
                }
            }
        }
    }

    let serializer = Serializer;

    assert!(serializer.serialize_i128(-i128::MAX).is_err());
    assert!(serializer.serialize_i128(i128::MAX).is_err());
}

