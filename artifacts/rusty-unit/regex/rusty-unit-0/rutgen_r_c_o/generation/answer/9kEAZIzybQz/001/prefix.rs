// Answer 0

#[test]
fn test_canonical_script_valid_script_1() {
    let normalized_value = "valid_script_name_1";
    canonical_script(normalized_value);
}

#[test]
fn test_canonical_script_valid_script_2() {
    let normalized_value = "valid_script_name_2";
    canonical_script(normalized_value);
}

#[test]
fn test_canonical_script_invalid_script() {
    let normalized_value = "invalid_script_name";
    canonical_script(normalized_value);
}

#[test]
fn test_canonical_script_empty_string() {
    let normalized_value = "";
    canonical_script(normalized_value);
}

#[test]
fn test_canonical_script_space() {
    let normalized_value = " ";
    canonical_script(normalized_value);
}

#[test]
fn test_canonical_script_another_valid_script() {
    let normalized_value = "another_valid_script_name";
    canonical_script(normalized_value);
}

