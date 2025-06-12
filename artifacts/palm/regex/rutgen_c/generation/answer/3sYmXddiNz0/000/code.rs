// Answer 0

#[test]
fn test_no_expansion_default() {
    #[derive(Debug)]
    struct SimpleReplacer;

    impl Replacer for SimpleReplacer {
        fn replace_append(&mut self, _caps: &Captures, _dst: &mut String) {
            // no-op for testing
        }
    }

    let mut simple_replacer = SimpleReplacer;
    let replacer_ref = ReplacerRef(&mut simple_replacer);
    
    assert_eq!(replacer_ref.no_expansion(), None);
}

#[test]
fn test_no_expansion_with_impl() {
    #[derive(Debug)]
    struct CustomReplacer;

    impl Replacer for CustomReplacer {
        fn replace_append(&mut self, _caps: &Captures, _dst: &mut String) {
            // no-op for testing
        }

        fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>> {
            Some(Cow::Borrowed("test"))
        }
    }

    let mut custom_replacer = CustomReplacer;
    let replacer_ref = ReplacerRef(&mut custom_replacer);
    
    assert_eq!(replacer_ref.no_expansion(), Some(Cow::Borrowed("test")));
}

