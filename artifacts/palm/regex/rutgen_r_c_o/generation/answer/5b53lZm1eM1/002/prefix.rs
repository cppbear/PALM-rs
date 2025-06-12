// Answer 0

#[should_panic]
fn test_contains_simple_case_mapping_panic_1() {
    contains_simple_case_mapping('b', 'a');
}

#[should_panic]
fn test_contains_simple_case_mapping_panic_2() {
    contains_simple_case_mapping('A', 'A');
}

#[should_panic]
fn test_contains_simple_case_mapping_panic_3() {
    contains_simple_case_mapping('3', '2');
}

#[should_panic]
fn test_contains_simple_case_mapping_panic_4() {
    contains_simple_case_mapping('Z', 'Y');
}

#[should_panic]
fn test_contains_simple_case_mapping_panic_5() {
    contains_simple_case_mapping('x', 'w');
}

