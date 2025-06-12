// Answer 0

fn equivalent_test() {
    struct TestKey;
    impl Equivalent<usize> for TestKey {
        fn equivalent(&self, other: &usize) -> bool {
            *other == 42
        }
    }

    let entries = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 42, value: "b" },
        Bucket { hash: HashValue::default(), key: 100, value: "c" },
    ];

    let key = TestKey;
    let equivalence_fn = equivalent(&key, &entries);

    assert_eq!(equivalence_fn(&0), false); // entry[0].key = 1
    assert_eq!(equivalence_fn(&1), true);  // entry[1].key = 42
    assert_eq!(equivalence_fn(&2), false); // entry[2].key = 100
}

fn equivalent_panic_test() {
    struct InvalidKey;
    impl Equivalent<usize> for InvalidKey {
        fn equivalent(&self, _other: &usize) -> bool {
            panic!("This should panic")
        }
    }

    let entries = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 42, value: "b" },
    ];

    let key = InvalidKey;
    let equivalence_fn = equivalent(&key, &entries);
    
    // This test should panic
    let _ = std::panic::catch_unwind(|| {
        equivalence_fn(&0);
    }).unwrap_err();
}

