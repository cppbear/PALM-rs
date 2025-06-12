// Answer 0

#[test]
fn test_from_name_alpha_success() {
    let result = ClassAsciiKind::from_name("alpha");
}

#[test]
fn test_from_name_alnum_failure() {
    let result = ClassAsciiKind::from_name("alnum");
}

#[test]
fn test_from_name_not_a_match() {
    let result = ClassAsciiKind::from_name("not_a_match");
}

#[test]
fn test_from_name_uppercase_alpha_failure() {
    let result = ClassAsciiKind::from_name("ALPHA");
}

#[test]
fn test_from_name_mixed_case_alpha_failure() {
    let result = ClassAsciiKind::from_name("ALPHa");
}

