// Answer 0

#[test]
fn test_contains_simple_case_mapping_a_to_a() {
    contains_simple_case_mapping('a', 'a');
}

#[test]
fn test_contains_simple_case_mapping_z_to_z() {
    contains_simple_case_mapping('z', 'z');
}

#[test]
fn test_contains_simple_case_mapping_A_to_A() {
    contains_simple_case_mapping('A', 'A');
}

#[test]
fn test_contains_simple_case_mapping_Z_to_Z() {
    contains_simple_case_mapping('Z', 'Z');
}

#[test]
fn test_contains_simple_case_mapping_0_to_0() {
    contains_simple_case_mapping('0', '0');
}

#[test]
fn test_contains_simple_case_mapping_9_to_9() {
    contains_simple_case_mapping('9', '9');
}

#[test]
fn test_contains_simple_case_mapping_exclamation_to_exclamation() {
    contains_simple_case_mapping('!', '!');
}

#[test]
fn test_contains_simple_case_mapping_slash_to_slash() {
    contains_simple_case_mapping('/', '/');
}

#[test]
fn test_contains_simple_case_mapping_colon_to_colon() {
    contains_simple_case_mapping(':', ':');
}

#[test]
fn test_contains_simple_case_mapping_at_to_at() {
    contains_simple_case_mapping('@', '@');
}

#[test]
fn test_contains_simple_case_mapping_bracket_to_bracket() {
    contains_simple_case_mapping('[', '[');
}

#[test]
fn test_contains_simple_case_mapping_backtick_to_backtick() {
    contains_simple_case_mapping('`', '`');
}

#[test]
fn test_contains_simple_case_mapping_curly_to_curly() {
    contains_simple_case_mapping('{', '{');
}

#[test]
fn test_contains_simple_case_mapping_tilde_to_tilde() {
    contains_simple_case_mapping('~', '~');
}

