// Answer 0

#[test]
fn test_fmt() {
    struct Bucket {
        key: i32,
    }

    impl Bucket {
        fn key_ref(&self) -> &i32 {
            &self.key
        }
    }

    struct Iter {
        iter: Vec<Bucket>,
    }

    impl Iter {
        fn new(keys: Vec<i32>) -> Self {
            let iter = keys.into_iter().map(|key| Bucket { key }).collect();
            Iter { iter }
        }
    }

    use std::fmt;

    impl fmt::Debug for Iter {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let iter = self.iter.iter().map(Bucket::key_ref);
            f.debug_list().entries(iter).finish()
        }
    }

    let iter = Iter::new(vec![1, 2, 3]);
    let result = format!("{:?}", iter);
    assert_eq!(result, "[1, 2, 3]");
}

