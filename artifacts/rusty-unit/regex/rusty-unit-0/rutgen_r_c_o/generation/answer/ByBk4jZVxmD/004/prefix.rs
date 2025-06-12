// Answer 0

#[test]
fn test_error_syntax_max_length() {
    let error = Error::Syntax("~".repeat(80));
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", error);
}

#[test]
fn test_error_syntax_empty_string() {
    let error = Error::Syntax("");
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", error);
}

#[test]
fn test_error_syntax_newline_character() {
    let error = Error::Syntax("\n".to_string());
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", error);
} 

#[test]
fn test_error_syntax_long_string() {
    let error = Error::Syntax("This is a long error message to check how it formats in the debug string.".to_string());
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", error);
} 

#[test]
fn test_error_syntax_special_characters() {
    let error = Error::Syntax("Special char $%^&*@!".to_string());
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", error);
} 

