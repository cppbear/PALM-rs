// Answer 0

#[test]
fn test_suffixes_valid_input() {
    struct DummyLiterals {
        data: Vec<String>,
    }

    impl DummyLiterals {
        fn new(data: Vec<String>) -> Self {
            DummyLiterals { data }
        }
    }

    struct SingleByteSet;

    impl SingleByteSet {
        fn suffixes(_lits: &DummyLiterals) -> Self {
            SingleByteSet
        }
    }

    struct Matcher {
        literals: DummyLiterals,
        set: SingleByteSet,
    }

    impl Matcher {
        fn new(lits: &DummyLiterals, set: SingleByteSet) -> Self {
            Matcher {
                literals: lits.clone(),
                set,
            }
        }
    }

    let literals = DummyLiterals::new(vec!["test".into(), "example".into(), "suffix".into()]);
    let matcher = suffixes(&literals);
    assert!(matcher.literals.data.len() > 0);
}

#[test]
#[should_panic]
fn test_suffixes_empty_input() {
    struct DummyLiterals {
        data: Vec<String>,
    }

    impl DummyLiterals {
        fn new(data: Vec<String>) -> Self {
            DummyLiterals { data }
        }
    }

    struct SingleByteSet;

    impl SingleByteSet {
        fn suffixes(_lits: &DummyLiterals) -> Self {
            SingleByteSet
        }
    }

    struct Matcher {
        literals: DummyLiterals,
        set: SingleByteSet,
    }

    impl Matcher {
        fn new(lits: &DummyLiterals, set: SingleByteSet) -> Self {
            Matcher {
                literals: lits.clone(),
                set,
            }
        }
    }

    let literals = DummyLiterals::new(vec![]);
    let _matcher = suffixes(&literals); // This call should panic if not handled inside the method
}

