// Answer 0

#[test]
fn test_iter_fmt_debug() {
    use core::fmt::Formatter;

    struct TestBucket {
        hash: u64,
        key: &'static str,
        value: i32,
    }

    struct TestIter<'a> {
        iter: SliceIter<'a, TestBucket>,
    }

    impl<'a> fmt::Debug for TestIter<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.clone()).finish()
        }
    }

    let buckets: Vec<TestBucket> = vec![
        TestBucket { hash: 1, key: "a", value: 10 },
        TestBucket { hash: 2, key: "b", value: 20 },
    ];
    
    let slice_iter = buckets.iter().map(|bucket| Bucket {
        hash: bucket.hash,
        key: bucket.key,
        value: bucket.value,
    });

    let test_iter = TestIter { iter: slice_iter };

    let mut output = core::fmt::Formatter::debug_list();
    let result = test_iter.fmt(&mut output);
    
    assert!(result.is_ok());
}

