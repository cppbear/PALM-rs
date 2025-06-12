// Answer 0

#[test]
fn test_reverse_true() {
    struct TestStruct {
        compiled: Compiled,
    }

    struct Compiled {
        is_reverse: bool,
    }

    let mut instance = TestStruct {
        compiled: Compiled { is_reverse: false },
    };
    
    instance = instance.reverse(true);
    
    assert!(instance.compiled.is_reverse);
}

#[test]
fn test_reverse_false() {
    struct TestStruct {
        compiled: Compiled,
    }

    struct Compiled {
        is_reverse: bool,
    }

    let mut instance = TestStruct {
        compiled: Compiled { is_reverse: true },
    };
    
    instance = instance.reverse(false);
    
    assert!(!instance.compiled.is_reverse);
}

#[test]
fn test_reverse_no_effect() {
    struct TestStruct {
        compiled: Compiled,
    }

    struct Compiled {
        is_reverse: bool,
    }

    let instance = TestStruct {
        compiled: Compiled { is_reverse: false },
    };

    let result = instance.reverse(false);
    
    assert_eq!(result.compiled.is_reverse, false);
}

