// Answer 0

#[derive(Debug)]
struct CustomError {
    err: String,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}

#[test]
fn test_custom_error_fmt() {
    let error_message = "This is a custom error";
    let error_instance = CustomError {
        err: error_message.to_string(),
    };
    
    let result = format!("{}", error_instance);
    assert_eq!(result, error_message);
}

#[test]
fn test_custom_error_empty_message() {
    let error_instance = CustomError {
        err: String::new(),
    };

    let result = format!("{}", error_instance);
    assert_eq!(result, "");
}

#[test]
fn test_custom_error_long_message() {
    let error_message = "This is a very long error message to test formatting capability in the fmt implementation.";
    let error_instance = CustomError {
        err: error_message.to_string(),
    };
    
    let result = format!("{}", error_instance);
    assert_eq!(result, error_message);
}

#[should_panic]
#[test]
fn test_custom_error_panic() {
    let error_instance = CustomError {
        err: String::from("This will panic!"),
    };
    
    let _ = format!("{}", error_instance);
    panic!("We expect this to panic."); // Intentional panic statement for the test
}

