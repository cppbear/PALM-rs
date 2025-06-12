// Answer 0

#[test]
fn test_end_array_value_initial_state() {
    use alloc::vec::Vec;

    let mut writer = Vec::new();
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    let result = formatter.end_array_value(&mut writer);
}

#[test]
fn test_end_array_value_state_change() {
    use alloc::vec::Vec;

    let mut writer = Vec::new();
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    formatter.end_array_value(&mut writer);
    let result_after_call = formatter.has_value;
}

#[test]
fn test_end_array_value_with_empty_writer() {
    use alloc::vec::Vec;

    let mut writer = Vec::new();
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    let result = formatter.end_array_value(&mut writer);
}

