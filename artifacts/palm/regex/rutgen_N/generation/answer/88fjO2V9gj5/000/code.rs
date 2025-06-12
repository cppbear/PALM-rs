// Answer 0

#[test]
fn test_build_translator() {
    struct Config {
        flags: u32,
        allow_invalid_utf8: bool,
    }

    impl Config {
        fn new(flags: u32, allow_invalid_utf8: bool) -> Self {
            Config { flags, allow_invalid_utf8 }
        }

        fn build(&self) -> Translator {
            Translator {
                stack: RefCell::new(vec![]),
                flags: Cell::new(self.flags),
                allow_invalid_utf8: self.allow_invalid_utf8,
            }
        }
    }

    struct Translator {
        stack: RefCell<Vec<u8>>,
        flags: Cell<u32>,
        allow_invalid_utf8: bool,
    }

    let config = Config::new(0, true);
    let translator = config.build();

    assert_eq!(translator.flags.get(), 0);
    assert!(translator.allow_invalid_utf8);
    assert_eq!(translator.stack.borrow().len(), 0);
}

#[test]
fn test_build_translator_with_invalid_utf8() {
    struct Config {
        flags: u32,
        allow_invalid_utf8: bool,
    }

    impl Config {
        fn new(flags: u32, allow_invalid_utf8: bool) -> Self {
            Config { flags, allow_invalid_utf8 }
        }

        fn build(&self) -> Translator {
            Translator {
                stack: RefCell::new(vec![]),
                flags: Cell::new(self.flags),
                allow_invalid_utf8: self.allow_invalid_utf8,
            }
        }
    }

    struct Translator {
        stack: RefCell<Vec<u8>>,
        flags: Cell<u32>,
        allow_invalid_utf8: bool,
    }

    let config = Config::new(1, false);
    let translator = config.build();

    assert_eq!(translator.flags.get(), 1);
    assert!(!translator.allow_invalid_utf8);
    assert_eq!(translator.stack.borrow().len(), 0);
}

