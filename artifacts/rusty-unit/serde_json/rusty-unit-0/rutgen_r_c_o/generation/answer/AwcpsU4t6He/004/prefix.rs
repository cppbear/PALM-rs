// Answer 0

#[test]
fn test_begin_array_value_first_false() {
    use std::io::Cursor;

    let mut buffer = Cursor::new(Vec::new());
    let indent: &[u8] = b"  ";
    
    let mut formatter = PrettyFormatter {
        current_indent: 2,
        has_value: false,
        indent,
    };

    formatter.begin_array_value(&mut buffer, false).unwrap();
}

#[test]
fn test_begin_array_value_first_false_multiple_calls() {
    use std::io::Cursor;

    let mut buffer = Cursor::new(Vec::new());
    let indent: &[u8] = b"  ";
    
    let mut formatter = PrettyFormatter {
        current_indent: 4,
        has_value: false,
        indent,
    };

    for _ in 0..5 {
        formatter.begin_array_value(&mut buffer, false).unwrap();
    }
}

#[test]
fn test_begin_array_value_first_false_with_different_indent() {
    use std::io::Cursor;

    let mut buffer = Cursor::new(Vec::new());
    let indent: &[u8] = b"\t";
    
    let mut formatter = PrettyFormatter {
        current_indent: 1,
        has_value: false,
        indent,
    };

    formatter.begin_array_value(&mut buffer, false).unwrap();
}

#[test]
fn test_begin_array_value_first_false_empty_writer() {
    use std::io::Cursor;

    let mut buffer = Cursor::new(Vec::new());
    let indent: &[u8] = b"  ";
    
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent,
    };

    formatter.begin_array_value(&mut buffer, false).unwrap();
}

