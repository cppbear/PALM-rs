// Answer 0

#[test]
fn test_fmt_with_non_empty_iter() {
    struct Bucket {
        value: i32,
    }

    impl Bucket {
        fn value_ref(&self) -> &i32 {
            &self.value
        }
    }

    struct Iter {
        iter: Vec<Bucket>,
    }

    impl Iter {
        fn new(values: Vec<i32>) -> Self {
            let buckets = values.into_iter().map(|v| Bucket { value: v }).collect();
            Iter { iter: buckets }
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter.iter().map(Bucket::value_ref);
            f.debug_list().entries(iter).finish()
        }
    }

    let mut output = String::new();
    {
        let fmt_res = std::fmt::write(&mut output, |f| {
            let iter = Iter::new(vec![1, 2, 3, 4, 5]);
            iter.fmt(f)
        });
        assert!(fmt_res.is_ok());
    }
    assert_eq!(output, "[1, 2, 3, 4, 5]");
}

#[test]
fn test_fmt_with_empty_iter() {
    struct Bucket {
        value: i32,
    }

    impl Bucket {
        fn value_ref(&self) -> &i32 {
            &self.value
        }
    }

    struct Iter {
        iter: Vec<Bucket>,
    }

    impl Iter {
        fn new(values: Vec<i32>) -> Self {
            let buckets = values.into_iter().map(|v| Bucket { value: v }).collect();
            Iter { iter: buckets }
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter.iter().map(Bucket::value_ref);
            f.debug_list().entries(iter).finish()
        }
    }

    let mut output = String::new();
    {
        let fmt_res = std::fmt::write(&mut output, |f| {
            let iter = Iter::new(vec![]);
            iter.fmt(f)
        });
        assert!(fmt_res.is_ok());
    }
    assert_eq!(output, "[]");
}

