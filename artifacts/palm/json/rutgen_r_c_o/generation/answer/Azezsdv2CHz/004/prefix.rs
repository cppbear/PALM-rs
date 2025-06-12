// Answer 0

#[test]
fn test_end_object_no_value_indent_zero() {
    let indent = b"  ";
    let mut formatter = PrettyFormatter { current_indent: 0, has_value: false, indent };
    let mut output = Vec::new();
    formatter.end_object(&mut output).unwrap();
    // Output can be checked manually
}

#[test]
fn test_end_object_no_value_indent_five() {
    let indent = b"  ";
    let mut formatter = PrettyFormatter { current_indent: 5, has_value: false, indent };
    let mut output = Vec::new();
    formatter.end_object(&mut output).unwrap();
    // Output can be checked manually
}

#[test]
fn test_end_object_no_value_indent_ten() {
    let indent = b"  ";
    let mut formatter = PrettyFormatter { current_indent: 10, has_value: false, indent };
    let mut output = Vec::new();
    formatter.end_object(&mut output).unwrap();
    // Output can be checked manually
}

#[test]
fn test_end_object_no_value_indent_random() {
    let indent = b"  ";
    let mut formatter = PrettyFormatter { current_indent: 3, has_value: false, indent };
    let mut output = Vec::new();
    formatter.end_object(&mut output).unwrap();
    // Output can be checked manually
}

#[test]
fn test_end_object_no_value_with_non_empty_indent() {
    let indent = b"\t";
    let mut formatter = PrettyFormatter { current_indent: 4, has_value: false, indent };
    let mut output = Vec::new();
    formatter.end_object(&mut output).unwrap();
    // Output can be checked manually
}

