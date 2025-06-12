// Answer 0

#[derive(Debug)]
struct IndexHolder {
    index: u64,
}

impl IndexHolder {
    fn is_none(&self) -> bool {
        self.index == !0
    }
}

#[test]
fn test_is_none_with_max_value() {
    let holder = IndexHolder { index: !0 };
    assert!(holder.is_none());
}

#[test]
fn test_is_none_with_non_max_value() {
    let holder = IndexHolder { index: 0 };
    assert!(!holder.is_none());
}

#[test]
fn test_is_none_with_small_value() {
    let holder = IndexHolder { index: 1 };
    assert!(!holder.is_none());
}

#[test]
fn test_is_none_with_large_value() {
    let holder = IndexHolder { index: 100 };
    assert!(!holder.is_none());
}

#[test]
fn test_is_none_with_another_non_max_value() {
    let holder = IndexHolder { index: u64::MAX - 1 };
    assert!(!holder.is_none());
}

