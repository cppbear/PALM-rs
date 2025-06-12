// Answer 0

#[test]
fn test_debug_nonexhaustive() {
    #[derive(Clone, PartialEq)]
    pub struct NonExhaustive;

    impl fmt::Debug for NonExhaustive {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_tuple("__Nonexhaustive").finish()
        }
    }

    let error_variant = Error::__Nonexhaustive;
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", error_variant);
}

