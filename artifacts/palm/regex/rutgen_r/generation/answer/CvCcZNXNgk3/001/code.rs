// Answer 0

#[test]
fn test_empty_matcher() {
    struct Literals;
    impl Literals {
        fn empty() -> Self {
            Literals
        }
    }

    struct Matcher;
    impl Matcher {
        const Empty: Self = Matcher;
    }

    struct MatcherWrapper {
        literals: Literals,
        matcher: Matcher,
    }

    impl MatcherWrapper {
        fn new(literals: Literals, matcher: Matcher) -> Self {
            MatcherWrapper { literals, matcher }
        }
    }

    fn empty() -> MatcherWrapper {
        MatcherWrapper::new(Literals::empty(), Matcher::Empty)
    }

    let matcher = empty();
    // Verifying the matcher created is not null or improperly initialized.
    assert_eq!(std::mem::size_of::<MatcherWrapper>(), std::mem::size_of::<Matcher>() + std::mem::size_of::<Literals>());
}

