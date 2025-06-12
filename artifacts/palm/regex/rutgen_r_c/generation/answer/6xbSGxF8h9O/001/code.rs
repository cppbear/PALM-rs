// Answer 0

#[test]
fn test_is_empty_true() {
    let input_at = InputAt {
        pos: 0,
        c: Char(0),
        byte: None,
        len: 0,
    };
    assert!(input_at.is_empty());
}

#[test]
fn test_is_empty_false() {
    let input_at = InputAt {
        pos: 1,
        c: Char(1),
        byte: Some(1),
        len: 1,
    };
    assert!(!input_at.is_empty());
}

#[test]
fn test_is_empty_boundary_case() {
    let input_at = InputAt {
        pos: 2,
        c: Char(2),
        byte: Some(2),
        len: 0,
    };
    assert!(input_at.is_empty());
}

