// Answer 0

#[test]
fn test_equivalent_with_integer_keys() {
    struct IntEquivalent;
    
    impl Equivalent<usize> for IntEquivalent {
        fn equivalent(&self, other: &usize) -> bool {
            // Implement a simple equivalency check (e.g., checking for equality)
            *other == 42
        }
    }
    
    let entries = vec![
        Bucket { hash: HashValue::default(), key: 41, value: "Value1" },
        Bucket { hash: HashValue::default(), key: 42, value: "Value2" },
        Bucket { hash: HashValue::default(), key: 43, value: "Value3" },
    ];
    
    let eq_fn = equivalent(&IntEquivalent, &entries);
    
    assert!(!eq_fn(&0)); // 41 vs 42 -> false
    assert!(eq_fn(&1));  // 42 vs 42 -> true
    assert!(!eq_fn(&2)); // 43 vs 42 -> false
}

#[test]
fn test_equivalent_with_string_keys() {
    struct StrEquivalent;

    impl Equivalent<str> for StrEquivalent {
        fn equivalent(&self, other: &str) -> bool {
            // Check if the other string is "hello"
            other == "hello"
        }
    }

    let entries = vec![
        Bucket { hash: HashValue::default(), key: "world", value: "Value1" },
        Bucket { hash: HashValue::default(), key: "hello", value: "Value2" },
        Bucket { hash: HashValue::default(), key: "rust", value: "Value3" },
    ];

    let eq_fn = equivalent(&StrEquivalent, &entries);
    
    assert!(!eq_fn(&0)); // "world" vs "hello" -> false
    assert!(eq_fn(&1));  // "hello" vs "hello" -> true
    assert!(!eq_fn(&2)); // "rust" vs "hello" -> false
}

#[test]
fn test_equivalent_with_empty_entries() {
    struct AlwaysFalseEquivalent;

    impl Equivalent<usize> for AlwaysFalseEquivalent {
        fn equivalent(&self, _: &usize) -> bool {
            false
        }
    }

    let entries: Vec<Bucket<usize, &str>> = vec![];

    let eq_fn = equivalent(&AlwaysFalseEquivalent, &entries);
    
    // There are no entries, so any index should not be equivalent
    assert!(!(eq_fn(&0))); // No entry to compare -> false
}

#[test]
fn test_equivalent_with_same_key_and_value() {
    struct IdentityEquivalent;

    impl Equivalent<usize> for IdentityEquivalent {
        fn equivalent(&self, other: &usize) -> bool {
            // Check if both keys are equivalent (here assuming identity)
            *other == *other
        }
    }

    let entries = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "Value1" },
        Bucket { hash: HashValue::default(), key: 2, value: "Value2" },
    ];

    let eq_fn = equivalent(&IdentityEquivalent, &entries);
    
    assert!(eq_fn(&0)); // 1 vs 1 -> true
    assert!(eq_fn(&1)); // 2 vs 2 -> true
}

