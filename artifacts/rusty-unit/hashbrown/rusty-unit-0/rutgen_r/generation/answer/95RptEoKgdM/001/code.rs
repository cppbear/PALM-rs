// Answer 0

#[derive(Debug)]
struct TestKey;

impl Equivalent<i32> for TestKey {
    fn equivalent(&self, _: &i32) -> bool {
        true
    }
}

#[test]
fn test_equivalent_key_with_matching_key() {
    let k = TestKey;
    let equivalent_fn = equivalent_key(&k);
    let input = (42, "value");
    assert!(equivalent_fn(&input));
}

#[test]
fn test_equivalent_key_with_non_matching_key() {
    let k = TestKey;
    let equivalent_fn = equivalent_key(&k);
    let input = (100, "value");
    assert!(equivalent_fn(&input));
}

#[test]
fn test_equivalent_key_with_different_types() {
    struct TestKeyDifferent;

    impl Equivalent<String> for TestKeyDifferent {
        fn equivalent(&self, _: &String) -> bool {
            false
        }
    }

    let k = TestKeyDifferent;
    let equivalent_fn = equivalent_key(&k);
    let input = (String::from("test"), "value");
    assert!(!equivalent_fn(&input));
}

#[test]
fn test_equivalent_key_empty_input() {
    let k = TestKey;
    let equivalent_fn = equivalent_key(&k);
    let input: (i32, &str) = (0, "value");
    assert!(equivalent_fn(&input));
}

