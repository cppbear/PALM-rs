// Answer 0

#[test]
fn test_iter_mut_debug_fmt() {
    use core::fmt::Write;

    struct DummyHasher;

    impl BuildHasher for DummyHasher {
        type Hasher = DummyHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DummyHasher
        }
    }

    impl Hash for DummyHasher {
        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {}
    }

    let buckets = vec![
        Bucket { hash: DummyHasher::default(), key: 1, value: "a" },
        Bucket { hash: DummyHasher::default(), key: 2, value: "b" },
    ];

    let mut iter = IterMut { iter: buckets.iter_mut() };

    let mut output = String::new();
    write!(&mut output, "{:?}", iter).unwrap();

    assert!(output.contains("1") && output.contains("a"));
    assert!(output.contains("2") && output.contains("b"));
}

#[test]
fn test_iter_mut_debug_fmt_empty() {
    let buckets: Vec<Bucket<i32, &str>> = Vec::new();
    let mut iter = IterMut { iter: buckets.iter_mut() };

    let mut output = String::new();
    write!(&mut output, "{:?}", iter).unwrap();

    assert_eq!(output, "[]");
}

