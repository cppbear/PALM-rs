// Answer 0

#[test]
fn test_serialize_i128_arbitrary_precision() {
    let serializer = Serializer;

    #[cfg(feature = "arbitrary_precision")]
    {
        let value_to_serialize: i128 = 12345678901234567890; // a large number for arbitrary precision
        let result = serializer.serialize_i128(value_to_serialize);
        assert!(result.is_ok());
        if let Ok(serialized_value) = result {
            match serialized_value {
                Value::Number(number) => {
                    // Assert that number properly encapsulates the large i128 value
                    assert_eq!(number.n, number::N::from(value_to_serialize));
                },
                _ => panic!("Expected Value::Number"),
            }
        }
    }
}

#[test]
fn test_serialize_i128_non_arbitrary_precision_u64() {
    let serializer = Serializer;

    #[cfg(not(feature = "arbitrary_precision"))]
    {
        let value_to_serialize: i128 = 18446744073709551615; // maximum u64 value
        let result = serializer.serialize_i128(value_to_serialize);
        assert!(result.is_ok());
        if let Ok(serialized_value) = result {
            match serialized_value {
                Value::Number(number) => {
                    // Assert that number properly encapsulates the maximum u64 value
                    assert_eq!(number.n, number::N::from(value_to_serialize as u64));
                },
                _ => panic!("Expected Value::Number"),
            }
        }
    }
}

#[test]
fn test_serialize_i128_non_arbitrary_precision_i64() {
    let serializer = Serializer;

    #[cfg(not(feature = "arbitrary_precision"))]
    {
        let value_to_serialize: i128 = -9223372036854775808; // minimum i64 value
        let result = serializer.serialize_i128(value_to_serialize);
        assert!(result.is_ok());
        if let Ok(serialized_value) = result {
            match serialized_value {
                Value::Number(number) => {
                    // Assert that number properly encapsulates the minimum i64 value
                    assert_eq!(number.n, number::N::from(value_to_serialize as i64));
                },
                _ => panic!("Expected Value::Number"),
            }
        }
    }
}

#[test]
fn test_serialize_i128_non_arbitrary_precision_out_of_range() {
    let serializer = Serializer;

    #[cfg(not(feature = "arbitrary_precision"))]
    {
        let value_to_serialize: i128 = 170141183460469231731687303715884105727; // out of range for u64 and i64
        let result = serializer.serialize_i128(value_to_serialize);
        assert!(result.is_err());
        if let Err(error) = result {
            // Checking the error code for NumberOutOfRange
            assert_eq!(error.err.code, ErrorCode::NumberOutOfRange);
        }
    }
}

