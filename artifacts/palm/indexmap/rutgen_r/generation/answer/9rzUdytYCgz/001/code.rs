// Answer 0

#[test]
fn test_hash_with_true_values() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct TrueValues<'a> {
        values: &'a [bool],
    }

    impl<'a> TrueValues<'a> {
        fn len(&self) -> usize {
            self.values.len()
        }
    }

    impl<'a> IntoIterator for TrueValues<'a> {
        type Item = &'a bool;
        type IntoIter = std::slice::Iter<'a, bool>;

        fn into_iter(self) -> Self::IntoIter {
            self.values.iter()
        }
    }

    let values = [true, true, true];
    let true_values = TrueValues { values: &values };
    let mut hasher = DefaultHasher::new();
    true_values.hash(&mut hasher);
    let hash_result = hasher.finish();

    assert!(hash_result > 0);
}

#[test]
fn test_hash_with_false_values() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct FalseValues<'a> {
        values: &'a [bool],
    }

    impl<'a> FalseValues<'a> {
        fn len(&self) -> usize {
            self.values.len()
        }
    }

    impl<'a> IntoIterator for FalseValues<'a> {
        type Item = &'a bool;
        type IntoIter = std::slice::Iter<'a, bool>;

        fn into_iter(self) -> Self::IntoIter {
            self.values.iter()
        }
    }

    let values = [false, false, false];
    let false_values = FalseValues { values: &values };
    let mut hasher = DefaultHasher::new();
    false_values.hash(&mut hasher);
    let hash_result = hasher.finish();

    assert!(hash_result > 0);
}

