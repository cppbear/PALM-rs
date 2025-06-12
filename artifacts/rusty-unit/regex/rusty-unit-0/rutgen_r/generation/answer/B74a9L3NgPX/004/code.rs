// Answer 0

#[test]
fn test_unicode_fold_and_negate_non_case_insensitive_not_negated() {
    struct MockFlags {
        case_insensitive: bool,
    }

    impl MockFlags {
        fn case_insensitive(&self) -> bool {
            self.case_insensitive
        }
    }

    struct MockHir {
        flags: MockFlags,
    }

    impl MockHir {
        fn flags(&self) -> &MockFlags {
            &self.flags
        }

        fn unicode_fold_and_negate(
            &self,
            negated: bool,
            class: &mut ClassUnicode,
        ) {
            if self.flags().case_insensitive() {
                class.case_fold_simple();
            }
            if negated {
                class.negate();
            }
        }
    }

    struct ClassUnicode {
        is_case_folding_done: bool,
        is_negated: bool,
    }

    impl ClassUnicode {
        fn new() -> Self {
            ClassUnicode {
                is_case_folding_done: false,
                is_negated: false,
            }
        }

        fn case_fold_simple(&mut self) {
            self.is_case_folding_done = true;
        }

        fn negate(&mut self) {
            self.is_negated = true;
        }
    }

    let hir = MockHir {
        flags: MockFlags {
            case_insensitive: false,
        },
    };

    let mut class = ClassUnicode::new();
    hir.unicode_fold_and_negate(false, &mut class);

    assert!(!class.is_case_folding_done);
    assert!(!class.is_negated);
}

