// Answer 0

#[test]
fn test_empty_literals_default_values() {
    struct ArgsTest {
        flag_literal_limit: usize,
        flag_class_limit: usize,
    }

    impl ArgsTest {
        fn empty_literals(&self) -> Literals {
            let mut lits = Literals::empty();
            lits.set_limit_size(self.flag_literal_limit);
            lits.set_limit_class(self.flag_class_limit);
            lits
        }
    }

    let args = ArgsTest {
        flag_literal_limit: 250,
        flag_class_limit: 10,
    };

    let literals = args.empty_literals();
    assert_eq!(literals.size_limit(), 250);
    assert_eq!(literals.class_limit(), 10);
}

#[test]
fn test_empty_literals_modified_limits() {
    struct ArgsTest {
        flag_literal_limit: usize,
        flag_class_limit: usize,
    }

    impl ArgsTest {
        fn empty_literals(&self) -> Literals {
            let mut lits = Literals::empty();
            lits.set_limit_size(self.flag_literal_limit);
            lits.set_limit_class(self.flag_class_limit);
            lits
        }
    }

    let args = ArgsTest {
        flag_literal_limit: 300,
        flag_class_limit: 15,
    };

    let literals = args.empty_literals();
    assert_eq!(literals.size_limit(), 300);
    assert_eq!(literals.class_limit(), 15);
}

