// Answer 0

#[derive(Default)]
struct Versioned {
    version: usize,
}

#[test]
fn test_clear_increments_version() {
    let mut item = Versioned::default();
    assert_eq!(item.version, 0);
    item.clear();
    assert_eq!(item.version, 1);
    item.clear();
    assert_eq!(item.version, 2);
}

#[test]
fn test_clear_multiple_calls() {
    let mut item = Versioned::default();
    for _ in 0..5 {
        item.clear();
    }
    assert_eq!(item.version, 5);
}

#[test]
fn test_clear_after_reaching_boundary() {
    let mut item = Versioned { version: usize::MAX - 1 };
    item.clear();
    assert_eq!(item.version, usize::MAX);
}

