// Answer 0

#[test]
fn test_needs_dotstar_true() {
    struct TestStruct {
        is_dfa: bool,
        is_reverse: bool,
        is_anchored_start: bool,
    }

    let test_instance = TestStruct {
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
    };

    assert!(needs_dotstar(&test_instance));
}

#[test]
fn test_needs_dotstar_false_is_reverse() {
    struct TestStruct {
        is_dfa: bool,
        is_reverse: bool,
        is_anchored_start: bool,
    }

    let test_instance = TestStruct {
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
    };

    assert!(!needs_dotstar(&test_instance));
}

#[test]
fn test_needs_dotstar_false_is_anchored_start() {
    struct TestStruct {
        is_dfa: bool,
        is_reverse: bool,
        is_anchored_start: bool,
    }

    let test_instance = TestStruct {
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
    };

    assert!(!needs_dotstar(&test_instance));
}

#[test]
fn test_needs_dotstar_false_is_not_dfa() {
    struct TestStruct {
        is_dfa: bool,
        is_reverse: bool,
        is_anchored_start: bool,
    }

    let test_instance = TestStruct {
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
    };

    assert!(!needs_dotstar(&test_instance));
}

