// Answer 0

#[test]
fn test_end_array_no_value_indent_zero() {
    let mut writer: Vec<u8> = Vec::new();
    let indent: &[u8] = b"  ";
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent,
    };
    formatter.end_array(&mut writer).unwrap();
}

#[test]
fn test_end_array_no_value_indent_non_zero() {
    let mut writer: Vec<u8> = Vec::new();
    let indent: &[u8] = b"    ";
    let mut formatter = PrettyFormatter {
        current_indent: 3,
        has_value: false,
        indent,
    };
    formatter.end_array(&mut writer).unwrap();
}

#[test]
fn test_end_array_no_value_indent_non_empty() {
    let mut writer: Vec<u8> = Vec::new();
    let indent: &[u8] = b"\t";
    let mut formatter = PrettyFormatter {
        current_indent: 2,
        has_value: false,
        indent,
    };
    formatter.end_array(&mut writer).unwrap();
}

