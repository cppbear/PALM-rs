// Answer 0

#[test]
fn test_is_always_utf8_unicode() {
    struct Class {
        is_unicode: bool,
    }

    impl Class {
        fn is_always_utf8(&self) -> bool {
            match *self {
                Class { is_unicode: true } => true,
                _ => false,
            }
        }
    }

    let unicode_class = Class { is_unicode: true };
    assert!(unicode_class.is_always_utf8());
}

#[test]
fn test_is_always_utf8_bytes() {
    struct ByteClass {
        ascii: bool,
    }

    impl ByteClass {
        fn is_all_ascii(&self) -> bool {
            self.ascii
        }

        fn is_always_utf8(&self) -> bool {
            self.is_all_ascii()
        }
    }

    let bytes_class_ascii = ByteClass { ascii: true };
    assert!(bytes_class_ascii.is_always_utf8());
}

#[test]
fn test_is_always_utf8_non_ascii() {
    struct ByteClass {
        ascii: bool,
    }

    impl ByteClass {
        fn is_all_ascii(&self) -> bool {
            self.ascii
        }

        fn is_always_utf8(&self) -> bool {
            self.is_all_ascii()
        }
    }

    let bytes_class_non_ascii = ByteClass { ascii: false };
    assert!(!bytes_class_non_ascii.is_always_utf8());
}

