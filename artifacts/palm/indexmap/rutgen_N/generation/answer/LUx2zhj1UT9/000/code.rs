// Answer 0

#[test]
fn test_fmt_with_empty_iter() {
    struct EmptyIter;

    impl Clone for EmptyIter {
        fn clone(&self) -> Self {
            EmptyIter
        }
    }

    use std::fmt;

    impl fmt::Debug for EmptyIter {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().finish()
        }
    }

    let iter = EmptyIter;
    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        iter.fmt(&mut formatter).unwrap();
    }
    assert_eq!(output, "");
}

#[test]
fn test_fmt_with_non_empty_iter() {
    struct NonEmptyIter {
        values: Vec<i32>,
    }

    impl Clone for NonEmptyIter {
        fn clone(&self) -> Self {
            NonEmptyIter {
                values: self.values.clone(),
            }
        }
    }

    use std::fmt;

    impl fmt::Debug for NonEmptyIter {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(&self.values).finish()
        }
    }

    let iter = NonEmptyIter {
        values: vec![1, 2, 3],
    };
    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        iter.fmt(&mut formatter).unwrap();
    }
    assert_eq!(output, "[1, 2, 3]");
}

