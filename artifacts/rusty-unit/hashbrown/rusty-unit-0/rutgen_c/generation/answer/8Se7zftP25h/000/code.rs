// Answer 0

#[test]
fn test_get_existing_value() {
    struct SimpleEq;
    impl Equivalent<i32> for SimpleEq {
        fn equivalent(&self, _: &i32) -> bool {
            true // stub implementation for testing
        }
    }

    let set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    assert_eq!(set.get(&2), Some(&2));
}

#[test]
fn test_get_non_existing_value() {
    struct SimpleEq;
    impl Equivalent<i32> for SimpleEq {
        fn equivalent(&self, _: &i32) -> bool {
            true // stub implementation for testing
        }
    }

    let set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    assert_eq!(set.get(&4), None);
}

#[test]
fn test_get_with_different_borrowed_type() {
    struct SimpleEq;
    impl Equivalent<i32> for SimpleEq {
        fn equivalent(&self, _: &i32) -> bool {
            true // stub implementation for testing
        }
    }

    let set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let borrowed_value: &i32 = &2;
    assert_eq!(set.get(borrowed_value), Some(&2));
}

