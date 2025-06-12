// Answer 0

fn unicode_fold_and_negate_test() {
    struct TestFlags {
        case_insensitive: bool,
    }

    impl TestFlags {
        fn case_insensitive(&self) -> bool {
            self.case_insensitive
        }
    }

    struct TestStruct {
        flags: TestFlags,
    }

    impl TestStruct {
        fn flags(&self) -> &TestFlags {
            &self.flags
        }

        fn unicode_fold_and_negate(&self, negated: bool, class: &mut ClassUnicode) {
            if self.flags().case_insensitive() {
                class.case_fold_simple();
            }
            if negated {
                class.negate();
            }
        }
    }

    struct ClassUnicode {
        case_folded: bool,
        negated: bool,
    }

    impl ClassUnicode {
        fn new() -> Self {
            ClassUnicode {
                case_folded: false,
                negated: false,
            }
        }

        fn case_fold_simple(&mut self) {
            self.case_folded = true;
        }

        fn negate(&mut self) {
            self.negated = true;
        }
    }

    #[test]
    fn test_unicode_fold_and_negate() {
        let flags = TestFlags { case_insensitive: true };
        let test_struct = TestStruct { flags };
        let mut class = ClassUnicode::new();

        test_struct.unicode_fold_and_negate(true, &mut class);

        assert!(class.case_folded);
        assert!(class.negated);
    }
}

