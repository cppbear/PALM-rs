// Answer 0

#[test]
fn test_error_fmt_syntax() {
    let error = Error::Syntax("Invalid syntax near '}'.".to_string());
    let mut output = vec![];
    let result = {
        let formatter = &mut fmt::Formatter::new(&mut output);
        error.fmt(formatter)
    };
    
    assert!(result.is_ok());
    let expected = format!("Syntax(\n{}\n{}\n{})", 
        repeat('~').take(79).collect::<String>(), 
        "Invalid syntax near '}'.".to_string(), 
        repeat('~').take(79).collect::<String>());
    assert_eq!(String::from_utf8(output).unwrap().trim(), expected);
}

#[test]
fn test_error_fmt_compiled_too_big() {
    let error = Error::CompiledTooBig(1024);
    let mut output = vec![];
    let result = {
        let formatter = &mut fmt::Formatter::new(&mut output);
        error.fmt(formatter)
    };
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap().trim(), "CompiledTooBig(1024)");
}

#[test]
fn test_error_fmt_nonexhaustive() {
    let error = Error::__Nonexhaustive;
    let mut output = vec![];
    let result = {
        let formatter = &mut fmt::Formatter::new(&mut output);
        error.fmt(formatter)
    };
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap().trim(), "__Nonexhaustive");
}

