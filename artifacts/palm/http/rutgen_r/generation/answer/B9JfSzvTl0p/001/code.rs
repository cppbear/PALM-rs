// Answer 0


#[derive(Debug)]
struct DisplayableStruct {
    value: String,
}

impl std::fmt::Display for DisplayableStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[test]
fn test_displayable_struct_with_normal_value() {
    let displayable = DisplayableStruct {
        value: String::from("Normal Value"),
    };
    let result = format!("{}", displayable);
    assert_eq!(result, "Normal Value");
}

#[test]
fn test_displayable_struct_with_empty_value() {
    let displayable = DisplayableStruct {
        value: String::from(""),
    };
    let result = format!("{}", displayable);
    assert_eq!(result, "");
}

#[test]
fn test_displayable_struct_with_long_value() {
    let displayable = DisplayableStruct {
        value: String::from("A very long string that should be handled properly by the fmt function!"),
    };
    let result = format!("{}", displayable);
    assert_eq!(result, "A very long string that should be handled properly by the fmt function!");
}

#[should_panic]
fn test_displayable_struct_with_non_utf8_value() {
    let invalid_utf8 = vec![0xFF, 0xFE, 0xFD]; // Invalid UTF-8 bytes
    let displayable = DisplayableStruct {
        value: String::from_utf8(invalid_utf8).unwrap_err().to_string(),
    };
    let _ = format!("{}", displayable);
}


