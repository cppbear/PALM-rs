// Answer 0

#[test]
fn test_into_values_debug_fmt() {
    struct TestBucket {
        hash: HashValue,
        key: char,
        value: i32,
    }

    impl Bucket<char, i32> {
        fn value_ref(&self) -> &i32 {
            &self.value
        }
    }

    let buckets = vec![
        TestBucket { hash: HashValue, key: 'a', value: 1 },
        TestBucket { hash: HashValue, key: 'b', value: 2 },
        TestBucket { hash: HashValue, key: 'c', value: 3 },
    ];

    let iter = vec![buckets[0].clone(), buckets[1].clone(), buckets[2].clone()].into_iter();

    let into_values = IntoValues { iter };

    let mut output = Vec::new();
    let result = into_values.fmt(&mut output);

    assert!(result.is_ok());
}

