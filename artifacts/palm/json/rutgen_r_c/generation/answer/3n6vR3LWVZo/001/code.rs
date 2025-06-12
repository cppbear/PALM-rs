// Answer 0

#[test]
fn test_begin_object_value_success() {
    use std::io::Cursor;
    
    // Create a writer with an in-memory buffer
    let mut buffer = Cursor::new(Vec::new());
    
    // Instantiate PrettyFormatter
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    // Call the function
    let result = formatter.begin_object_value(&mut buffer);
    
    // Check if the result is Ok
    assert!(result.is_ok());
    
    // Check if the buffer contains the expected output
    assert_eq!(buffer.get_ref().as_slice(), b": ");
}

#[test]
fn test_begin_object_value_empty_writer() {
    use std::io::Cursor;
    
    // Create an empty writer
    let mut buffer = Cursor::new(Vec::new());
    
    // Instantiate PrettyFormatter
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    // Call the function
    let result = formatter.begin_object_value(&mut buffer);
    
    // Check if the result is Ok
    assert!(result.is_ok());

    // Check that the buffer contains the expected output
    assert_eq!(buffer.get_ref().as_slice(), b": ");
}

#[test]
fn test_begin_object_value_check_empty_string() {
    use std::io::Cursor;

    // Create a writer with an in-memory buffer
    let mut buffer = Cursor::new(Vec::new());

    // Instantiate PrettyFormatter
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    // Call the function
    let result = formatter.begin_object_value(&mut buffer);

    // Check if the result is Ok
    assert!(result.is_ok());

    // Check that the buffer length is not zero
    assert!(buffer.get_ref().len() > 0);

    // Verify that it contains the correct bytes
    assert_eq!(buffer.get_ref().as_slice(), b": ");
}

