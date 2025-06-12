// Answer 0

#[test]
fn test_from_escape_table_quote() {
    struct TestStruct;

    impl TestStruct {
        fn from_escape_table(escape: u8, byte: u8) -> CharEscape {
            match escape {
                BB => CharEscape::Backspace,
                TT => CharEscape::Tab,
                NN => CharEscape::LineFeed,
                FF => CharEscape::FormFeed,
                RR => CharEscape::CarriageReturn,
                QU => CharEscape::Quote,
                BS => CharEscape::ReverseSolidus,
                UU => CharEscape::AsciiControl(byte),
                _ => unreachable!(),
            }
        }
    }

    let result = TestStruct::from_escape_table(QU, 0);
    match result {
        CharEscape::Quote => {}
        _ => panic!("Expected CharEscape::Quote"),
    }
}

#[test]
fn test_from_escape_table_backspace() {
    struct TestStruct;

    impl TestStruct {
        fn from_escape_table(escape: u8, byte: u8) -> CharEscape {
            match escape {
                BB => CharEscape::Backspace,
                TT => CharEscape::Tab,
                NN => CharEscape::LineFeed,
                FF => CharEscape::FormFeed,
                RR => CharEscape::CarriageReturn,
                QU => CharEscape::Quote,
                BS => CharEscape::ReverseSolidus,
                UU => CharEscape::AsciiControl(byte),
                _ => unreachable!(),
            }
        }
    }

    let result = TestStruct::from_escape_table(BB, 0);
    match result {
        CharEscape::Backspace => {}
        _ => panic!("Expected CharEscape::Backspace"),
    }
}

#[test]
fn test_from_escape_table_tab() {
    struct TestStruct;

    impl TestStruct {
        fn from_escape_table(escape: u8, byte: u8) -> CharEscape {
            match escape {
                BB => CharEscape::Backspace,
                TT => CharEscape::Tab,
                NN => CharEscape::LineFeed,
                FF => CharEscape::FormFeed,
                RR => CharEscape::CarriageReturn,
                QU => CharEscape::Quote,
                BS => CharEscape::ReverseSolidus,
                UU => CharEscape::AsciiControl(byte),
                _ => unreachable!(),
            }
        }
    }

    let result = TestStruct::from_escape_table(TT, 0);
    match result {
        CharEscape::Tab => {}
        _ => panic!("Expected CharEscape::Tab"),
    }
}

#[test]
fn test_from_escape_table_line_feed() {
    struct TestStruct;

    impl TestStruct {
        fn from_escape_table(escape: u8, byte: u8) -> CharEscape {
            match escape {
                BB => CharEscape::Backspace,
                TT => CharEscape::Tab,
                NN => CharEscape::LineFeed,
                FF => CharEscape::FormFeed,
                RR => CharEscape::CarriageReturn,
                QU => CharEscape::Quote,
                BS => CharEscape::ReverseSolidus,
                UU => CharEscape::AsciiControl(byte),
                _ => unreachable!(),
            }
        }
    }

    let result = TestStruct::from_escape_table(NN, 0);
    match result {
        CharEscape::LineFeed => {}
        _ => panic!("Expected CharEscape::LineFeed"),
    }
}

#[test]
fn test_from_escape_table_form_feed() {
    struct TestStruct;

    impl TestStruct {
        fn from_escape_table(escape: u8, byte: u8) -> CharEscape {
            match escape {
                BB => CharEscape::Backspace,
                TT => CharEscape::Tab,
                NN => CharEscape::LineFeed,
                FF => CharEscape::FormFeed,
                RR => CharEscape::CarriageReturn,
                QU => CharEscape::Quote,
                BS => CharEscape::ReverseSolidus,
                UU => CharEscape::AsciiControl(byte),
                _ => unreachable!(),
            }
        }
    }

    let result = TestStruct::from_escape_table(FF, 0);
    match result {
        CharEscape::FormFeed => {}
        _ => panic!("Expected CharEscape::FormFeed"),
    }
}

#[test]
fn test_from_escape_table_carriage_return() {
    struct TestStruct;

    impl TestStruct {
        fn from_escape_table(escape: u8, byte: u8) -> CharEscape {
            match escape {
                BB => CharEscape::Backspace,
                TT => CharEscape::Tab,
                NN => CharEscape::LineFeed,
                FF => CharEscape::FormFeed,
                RR => CharEscape::CarriageReturn,
                QU => CharEscape::Quote,
                BS => CharEscape::ReverseSolidus,
                UU => CharEscape::AsciiControl(byte),
                _ => unreachable!(),
            }
        }
    }

    let result = TestStruct::from_escape_table(RR, 0);
    match result {
        CharEscape::CarriageReturn => {}
        _ => panic!("Expected CharEscape::CarriageReturn"),
    }
}

#[test]
fn test_from_escape_table_reverse_solidus() {
    struct TestStruct;

    impl TestStruct {
        fn from_escape_table(escape: u8, byte: u8) -> CharEscape {
            match escape {
                BB => CharEscape::Backspace,
                TT => CharEscape::Tab,
                NN => CharEscape::LineFeed,
                FF => CharEscape::FormFeed,
                RR => CharEscape::CarriageReturn,
                QU => CharEscape::Quote,
                BS => CharEscape::ReverseSolidus,
                UU => CharEscape::AsciiControl(byte),
                _ => unreachable!(),
            }
        }
    }

    let result = TestStruct::from_escape_table(BS, 0);
    match result {
        CharEscape::ReverseSolidus => {}
        _ => panic!("Expected CharEscape::ReverseSolidus"),
    }
}

#[test]
fn test_from_escape_table_ascii_control() {
    struct TestStruct;

    impl TestStruct {
        fn from_escape_table(escape: u8, byte: u8) -> CharEscape {
            match escape {
                BB => CharEscape::Backspace,
                TT => CharEscape::Tab,
                NN => CharEscape::LineFeed,
                FF => CharEscape::FormFeed,
                RR => CharEscape::CarriageReturn,
                QU => CharEscape::Quote,
                BS => CharEscape::ReverseSolidus,
                UU => CharEscape::AsciiControl(byte),
                _ => unreachable!(),
            }
        }
    }

    let byte_value: u8 = 65; // ASCII value for 'A'
    let result = TestStruct::from_escape_table(UU, byte_value);
    match result {
        CharEscape::AsciiControl(65) => {}
        _ => panic!("Expected CharEscape::AsciiControl with byte 65"),
    }
}

