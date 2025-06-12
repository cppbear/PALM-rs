// Answer 0

#[test]
fn test_is_unicode_literal_unicode() {
    struct Literal {
        value: Option<char>,
    }

    impl Literal {
        fn unicode(c: char) -> Self {
            Literal { value: Some(c) }
        }

        fn byte(b: u8) -> Self {
            Literal { value: None }
        }

        pub fn is_unicode(&self) -> bool {
            match *self {
                Literal { value: Some(_) } => true,
                Literal { value: None } => false,
            }
        }
    }

    let literal_unicode = Literal::unicode('a');
    assert!(literal_unicode.is_unicode());

    let literal_unicode_special = Literal::unicode('😊');
    assert!(literal_unicode_special.is_unicode());
}

