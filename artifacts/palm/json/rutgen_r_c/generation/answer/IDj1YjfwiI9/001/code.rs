// Answer 0

fn test_invalid_type_i64() {
    use serde::de::{Expected, Unexpected};
    use crate::error::Error;
    use crate::number::ParserNumber;

    let exp: &dyn Expected = &Unexpected::Signed(42);
    
    let parser_number = ParserNumber::I64(42);
    let result = parser_number.invalid_type(exp);

    match result {
        Error { err } => {
            // Validate that the error encapsulates the expected type
            assert!(err.to_string().contains("invalid type: signed integer"));
            assert!(err.to_string().contains("expected signed integer"));
        }
    }
}

fn test_invalid_type_f64() {
    use serde::de::{Expected, Unexpected};
    use crate::error::Error;
    use crate::number::ParserNumber;

    let exp: &dyn Expected = &Unexpected::Float(3.14);
    
    let parser_number = ParserNumber::F64(3.14);
    let result = parser_number.invalid_type(exp);

    match result {
        Error { err } => {
            // Validate that the error encapsulates the expected type
            assert!(err.to_string().contains("invalid type: float"));
            assert!(err.to_string().contains("expected float"));
        }
    }
}

fn test_invalid_type_u64() {
    use serde::de::{Expected, Unexpected};
    use crate::error::Error;
    use crate::number::ParserNumber;

    let exp: &dyn Expected = &Unexpected::Unsigned(100);
    
    let parser_number = ParserNumber::U64(100);
    let result = parser_number.invalid_type(exp);

    match result {
        Error { err } => {
            // Validate that the error encapsulates the expected type
            assert!(err.to_string().contains("invalid type: unsigned integer"));
            assert!(err.to_string().contains("expected unsigned integer"));
        }
    }
}

