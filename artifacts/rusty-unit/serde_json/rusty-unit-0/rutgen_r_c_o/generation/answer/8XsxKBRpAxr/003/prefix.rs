// Answer 0

#[test]
fn test_end_array_with_indent_one() {
    let mut buffer = Vec::new();
    let mut formatter = PrettyFormatter {
        current_indent: 1,
        has_value: true,
        indent: b"  ",
    };
    formatter.end_array(&mut buffer).unwrap();
}

#[test]
fn test_end_array_with_indent_five() {
    let mut buffer = Vec::new();
    let mut formatter = PrettyFormatter {
        current_indent: 5,
        has_value: true,
        indent: b"  ",
    };
    formatter.end_array(&mut buffer).unwrap();
}

#[test]
fn test_end_array_with_indent_ten() {
    let mut buffer = Vec::new();
    let mut formatter = PrettyFormatter {
        current_indent: 10,
        has_value: true,
        indent: b"  ",
    };
    formatter.end_array(&mut buffer).unwrap();
}

#[test]
fn test_end_array_no_value() {
    let mut buffer = Vec::new();
    let mut formatter = PrettyFormatter {
        current_indent: 3,
        has_value: false,
        indent: b"  ",
    };
    formatter.end_array(&mut buffer).unwrap();
}

