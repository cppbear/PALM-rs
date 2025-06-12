// Answer 0

#[test]
fn test_multi_line_with_some_value() {
    struct TestFlags {
        multi_line: Option<bool>,
    }
    
    impl Flags {
        fn new(multi_line: Option<bool>) -> Self {
            Flags {
                multi_line,
                ..Default::default()
            }
        }
    }

    let flags = TestFlags::new(Some(true));
    assert_eq!(flags.multi_line(), true);
}

#[test]
fn test_multi_line_with_none() {
    struct TestFlags {
        multi_line: Option<bool>,
    }
    
    impl Flags {
        fn new(multi_line: Option<bool>) -> Self {
            Flags {
                multi_line,
                ..Default::default()
            }
        }
    }

    let flags = TestFlags::new(None);
    assert_eq!(flags.multi_line(), false);
}

#[test]
fn test_multi_line_with_false() {
    struct TestFlags {
        multi_line: Option<bool>,
    }
    
    impl Flags {
        fn new(multi_line: Option<bool>) -> Self {
            Flags {
                multi_line,
                ..Default::default()
            }
        }
    }

    let flags = TestFlags::new(Some(false));
    assert_eq!(flags.multi_line(), false);
}

