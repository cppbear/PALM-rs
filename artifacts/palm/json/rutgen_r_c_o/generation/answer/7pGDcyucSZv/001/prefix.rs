// Answer 0

#[test]
fn test_end_object_value_with_vector() {
    let mut writer = vec![];
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };
    formatter.end_object_value(&mut writer).unwrap();
}

#[test]
fn test_end_object_value_with_vec_writer() {
    let mut writer = vec![];
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };
    formatter.end_object_value(&mut writer).unwrap();
}

#[test]
fn test_end_object_value_with_buffer() {
    let mut writer = vec![];
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };
    formatter.end_object_value(&mut writer).unwrap();
}

#[test]
fn test_end_object_value_with_fixed_len_array() {
    let mut writer: [u8; 10] = [0; 10];
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };
    formatter.end_object_value(&mut writer).unwrap();
}

#[test]
fn test_end_object_value_multiple_calls() {
    let mut writer = vec![];
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };
    formatter.end_object_value(&mut writer).unwrap();
    formatter.end_object_value(&mut writer).unwrap();
    formatter.end_object_value(&mut writer).unwrap();
}

