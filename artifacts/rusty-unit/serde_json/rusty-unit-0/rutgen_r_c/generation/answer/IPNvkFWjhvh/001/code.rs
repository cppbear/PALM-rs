// Answer 0

#[test]
fn test_syntax_message_error() {
    let error_code = ErrorCode::Message(Box::from("Syntax error".to_string()));
    let line = 1;
    let column = 10;
    
    let result = Error::syntax(error_code, line, column);
    
    assert_eq!(
        *result.err.code,
        ErrorCode::Message(Box::from("Syntax error".to_string()))
    );
    assert_eq!(result.err.line, line);
    assert_eq!(result.err.column, column);
}

#[test]
fn test_syntax_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let line = 2;
    let column = 5;
    
    let result = Error::syntax(error_code, line, column);
    
    assert_eq!(result.err.code, ErrorCode::EofWhileParsingList);
    assert_eq!(result.err.line, line);
    assert_eq!(result.err.column, column);
}

#[test]
fn test_syntax_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let line = 3;
    let column = 15;
    
    let result = Error::syntax(error_code, line, column);
    
    assert_eq!(result.err.code, ErrorCode::EofWhileParsingObject);
    assert_eq!(result.err.line, line);
    assert_eq!(result.err.column, column);
}

#[test]
fn test_syntax_eof_while_parsing_string() {
    let error_code = ErrorCode::EofWhileParsingString;
    let line = 4;
    let column = 20;
    
    let result = Error::syntax(error_code, line, column);
    
    assert_eq!(result.err.code, ErrorCode::EofWhileParsingString);
    assert_eq!(result.err.line, line);
    assert_eq!(result.err.column, column);
}

#[test]
fn test_syntax_expected_colon() {
    let error_code = ErrorCode::ExpectedColon;
    let line = 5;
    let column = 25;
    
    let result = Error::syntax(error_code, line, column);
    
    assert_eq!(result.err.code, ErrorCode::ExpectedColon);
    assert_eq!(result.err.line, line);
    assert_eq!(result.err.column, column);
}

#[test]
fn test_syntax_invalid_number() {
    let error_code = ErrorCode::InvalidNumber;
    let line = 6;
    let column = 30;
    
    let result = Error::syntax(error_code, line, column);
    
    assert_eq!(result.err.code, ErrorCode::InvalidNumber);
    assert_eq!(result.err.line, line);
    assert_eq!(result.err.column, column);
}

