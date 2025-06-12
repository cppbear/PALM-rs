// Answer 0

#[test]
fn test_set_with_true_value() {
    let once_bool = OnceBool::new();
    let result = once_bool.set(true);
}

#[test]
fn test_set_with_false_value() {
    let once_bool = OnceBool::new();
    let result = once_bool.set(false);
}

#[test]
#[should_panic]
fn test_set_after_set_true() {
    let once_bool = OnceBool::new();
    let _ = once_bool.set(true);
    let result = once_bool.set(true);
}

#[test]
#[should_panic]
fn test_set_after_set_false() {
    let once_bool = OnceBool::new();
    let _ = once_bool.set(false);
    let result = once_bool.set(false);
}

