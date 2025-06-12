// Answer 0

#[test]
fn test_only_utf8_true() {
    let program = Program {
        only_utf8: true,
        ..Program::new() // Initialize other fields with defaults
    };
    assert!(program.only_utf8());
}

#[test]
fn test_only_utf8_false() {
    let program = Program {
        only_utf8: false,
        ..Program::new() // Initialize other fields with defaults
    };
    assert!(!program.only_utf8());
}

