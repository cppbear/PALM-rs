// Answer 0

#[test]
fn test_syntax_with_message_code() {
    let error_code = ErrorCode::Message(Box::from("Syntax error".to_string()));
    let line = 10;
    let column = 5;

    let error = Error::syntax(error_code, line, column);
    
    assert_eq!(match *error.err { 
        ErrorImpl { ref code, .. } => match code {
            ErrorCode::Message(msg) => msg.as_ref(), 
            _ => "",
        }
    }, "Syntax error");
    assert_eq!(error.err.line, line);
    assert_eq!(error.err.column, column);
}

#[test]
fn test_syntax_with_eof_while_parsing_value() {
    let error_code = ErrorCode::EofWhileParsingValue;
    let line = 1;
    let column = 2;

    let error = Error::syntax(error_code, line, column);
    
    assert!(matches!(*error.err.code, ErrorCode::EofWhileParsingValue));
    assert_eq!(error.err.line, line);
    assert_eq!(error.err.column, column);
}

#[test]
fn test_syntax_with_invalid_number() {
    let error_code = ErrorCode::InvalidNumber;
    let line = 3;
    let column = 15;

    let error = Error::syntax(error_code, line, column);
    
    assert!(matches!(*error.err.code, ErrorCode::InvalidNumber));
    assert_eq!(error.err.line, line);
    assert_eq!(error.err.column, column);
}

