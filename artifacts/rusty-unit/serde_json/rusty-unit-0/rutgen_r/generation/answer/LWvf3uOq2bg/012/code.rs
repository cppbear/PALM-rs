// Answer 0

#[test]
fn test_error_code_invalid_number() {
    // Define a struct to implement Display for simulating an I/O error
    struct IoError;
    
    impl std::fmt::Display for IoError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("I/O error simulation")
        }
    }

    // Define the ErrorCode enum to match the cases mentioned
    enum ErrorCode {
        Message(&'static str),
        Io(IoError),
        EofWhileParsingList,
        EofWhileParsingObject,
        EofWhileParsingString,
        EofWhileParsingValue,
        ExpectedColon,
        ExpectedListCommaOrEnd,
        ExpectedObjectCommaOrEnd,
        ExpectedSomeIdent,
        ExpectedSomeValue,
        ExpectedDoubleQuote,
        InvalidEscape,
        InvalidNumber,
        NumberOutOfRange,
        InvalidUnicodeCodePoint,
        ControlCharacterWhileParsingString,
        KeyMustBeAString,
        ExpectedNumericKey,
        FloatKeyMustBeFinite,
        LoneLeadingSurrogateInHexEscape,
        TrailingComma,
        TrailingCharacters,
        UnexpectedEndOfHexEscape,
        RecursionLimitExceeded,
    }

    // Implement the fmt function to match the one under test
    use std::fmt::{self, Display};

    fn fmt(f: &ErrorCode, formatter: &mut fmt::Formatter) -> fmt::Result {
        match f {
            ErrorCode::Message(msg) => formatter.write_str(msg),
            ErrorCode::Io(err) => Display::fmt(err, formatter),
            ErrorCode::EofWhileParsingList => formatter.write_str("EOF while parsing a list"),
            ErrorCode::EofWhileParsingObject => formatter.write_str("EOF while parsing an object"),
            ErrorCode::EofWhileParsingString => formatter.write_str("EOF while parsing a string"),
            ErrorCode::EofWhileParsingValue => formatter.write_str("EOF while parsing a value"),
            ErrorCode::ExpectedColon => formatter.write_str("expected `:`"),
            ErrorCode::ExpectedListCommaOrEnd => formatter.write_str("expected `,` or `]`"),
            ErrorCode::ExpectedObjectCommaOrEnd => formatter.write_str("expected `,` or `}`"),
            ErrorCode::ExpectedSomeIdent => formatter.write_str("expected ident"),
            ErrorCode::ExpectedSomeValue => formatter.write_str("expected value"),
            ErrorCode::ExpectedDoubleQuote => formatter.write_str("expected `\"`"),
            ErrorCode::InvalidEscape => formatter.write_str("invalid escape"),
            ErrorCode::InvalidNumber => formatter.write_str("invalid number"),
            ErrorCode::NumberOutOfRange => formatter.write_str("number out of range"),
            ErrorCode::InvalidUnicodeCodePoint => formatter.write_str("invalid unicode code point"),
            ErrorCode::ControlCharacterWhileParsingString => {
                formatter.write_str("control character (\\u0000-\\u001F) found while parsing a string")
            }
            ErrorCode::KeyMustBeAString => formatter.write_str("key must be a string"),
            ErrorCode::ExpectedNumericKey => {
                formatter.write_str("invalid value: expected key to be a number in quotes")
            }
            ErrorCode::FloatKeyMustBeFinite => {
                formatter.write_str("float key must be finite (got NaN or +/-inf)")
            }
            ErrorCode::LoneLeadingSurrogateInHexEscape => {
                formatter.write_str("lone leading surrogate in hex escape")
            }
            ErrorCode::TrailingComma => formatter.write_str("trailing comma"),
            ErrorCode::TrailingCharacters => formatter.write_str("trailing characters"),
            ErrorCode::UnexpectedEndOfHexEscape => formatter.write_str("unexpected end of hex escape"),
            ErrorCode::RecursionLimitExceeded => formatter.write_str("recursion limit exceeded"),
        }
    }

    // Create instances to test ErrorCode::InvalidNumber
    let invalid_number_error = ErrorCode::InvalidNumber;

    // Capture the output
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    
    // Call the fmt function
    fmt(&invalid_number_error, formatter).unwrap();

    // Assert the expected output
    assert_eq!(output, "invalid number");
}

#[test]
fn test_error_code_expected_numeric_key() {
    struct IoError;

    impl std::fmt::Display for IoError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("I/O error simulation")
        }
    }

    enum ErrorCode {
        Message(&'static str),
        Io(IoError),
        EofWhileParsingList,
        EofWhileParsingObject,
        EofWhileParsingString,
        EofWhileParsingValue,
        ExpectedColon,
        ExpectedListCommaOrEnd,
        ExpectedObjectCommaOrEnd,
        ExpectedSomeIdent,
        ExpectedSomeValue,
        ExpectedDoubleQuote,
        InvalidEscape,
        InvalidNumber,
        NumberOutOfRange,
        InvalidUnicodeCodePoint,
        ControlCharacterWhileParsingString,
        KeyMustBeAString,
        ExpectedNumericKey,
        FloatKeyMustBeFinite,
        LoneLeadingSurrogateInHexEscape,
        TrailingComma,
        TrailingCharacters,
        UnexpectedEndOfHexEscape,
        RecursionLimitExceeded,
    }

    fn fmt(f: &ErrorCode, formatter: &mut fmt::Formatter) -> fmt::Result {
        match f {
            ErrorCode::Message(msg) => formatter.write_str(msg),
            ErrorCode::Io(err) => Display::fmt(err, formatter),
            ErrorCode::EofWhileParsingList => formatter.write_str("EOF while parsing a list"),
            ErrorCode::EofWhileParsingObject => formatter.write_str("EOF while parsing an object"),
            ErrorCode::EofWhileParsingString => formatter.write_str("EOF while parsing a string"),
            ErrorCode::EofWhileParsingValue => formatter.write_str("EOF while parsing a value"),
            ErrorCode::ExpectedColon => formatter.write_str("expected `:`"),
            ErrorCode::ExpectedListCommaOrEnd => formatter.write_str("expected `,` or `]`"),
            ErrorCode::ExpectedObjectCommaOrEnd => formatter.write_str("expected `,` or `}`"),
            ErrorCode::ExpectedSomeIdent => formatter.write_str("expected ident"),
            ErrorCode::ExpectedSomeValue => formatter.write_str("expected value"),
            ErrorCode::ExpectedDoubleQuote => formatter.write_str("expected `\"`"),
            ErrorCode::InvalidEscape => formatter.write_str("invalid escape"),
            ErrorCode::InvalidNumber => formatter.write_str("invalid number"),
            ErrorCode::NumberOutOfRange => formatter.write_str("number out of range"),
            ErrorCode::InvalidUnicodeCodePoint => formatter.write_str("invalid unicode code point"),
            ErrorCode::ControlCharacterWhileParsingString => {
                formatter.write_str("control character (\\u0000-\\u001F) found while parsing a string")
            }
            ErrorCode::KeyMustBeAString => formatter.write_str("key must be a string"),
            ErrorCode::ExpectedNumericKey => {
                formatter.write_str("invalid value: expected key to be a number in quotes")
            }
            ErrorCode::FloatKeyMustBeFinite => {
                formatter.write_str("float key must be finite (got NaN or +/-inf)")
            }
            ErrorCode::LoneLeadingSurrogateInHexEscape => {
                formatter.write_str("lone leading surrogate in hex escape")
            }
            ErrorCode::TrailingComma => formatter.write_str("trailing comma"),
            ErrorCode::TrailingCharacters => formatter.write_str("trailing characters"),
            ErrorCode::UnexpectedEndOfHexEscape => formatter.write_str("unexpected end of hex escape"),
            ErrorCode::RecursionLimitExceeded => formatter.write_str("recursion limit exceeded"),
        }
    }

    let expected_numeric_key_error = ErrorCode::ExpectedNumericKey;

    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    
    fmt(&expected_numeric_key_error, formatter).unwrap();

    assert_eq!(output, "invalid value: expected key to be a number in quotes");
}

