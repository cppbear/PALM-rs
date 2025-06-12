// Answer 0

#[test]
fn test_drain_debug_fmt() {
    use core::fmt::Formatter;

    struct TestDrain<'a> {
        iter: Vec<Bucket<&'a str, i32>>,
    }

    impl<'a> Drain<'a, &'a str, i32> {
        fn new(iter: Vec<Bucket<&'a str, i32>>) -> Self {
            Drain { iter: iter.into_iter().collect() }
        }

        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let iter = self.iter.as_slice().iter().map(|bucket| &bucket.key);
            f.debug_list().entries(iter).finish()
        }
    }

    let buckets = vec![
        Bucket { hash: 1, key: "key1", value: 10 },
        Bucket { hash: 2, key: "key2", value: 20 },
    ];
    
    let drain = TestDrain::new(buckets);
    let mut output = String::new();
    let mut formatter = Formatter::new(&mut output);
    
    drain.fmt(&mut formatter).unwrap();

    assert_eq!(output, "[\"key1\", \"key2\"]");
}

