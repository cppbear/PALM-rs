// Answer 0

#[test]
fn test_to_usize_false() {
    let result = OnceBool::to_usize(false);
}

#[should_panic]
fn test_to_usize_false_panic() {
    let _result = OnceBool::to_usize(false);
}

