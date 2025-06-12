// Answer 0

#[test]
fn test_new_with_non_zero_limit() {
    struct DummyInner;

    let take = new(DummyInner, 10);
    assert_eq!(take.limit, 10);
}

#[test]
fn test_new_with_zero_limit() {
    struct DummyInner;

    let take = new(DummyInner, 0);
    assert_eq!(take.limit, 0);
}

#[test]
#[should_panic]
fn test_new_with_large_limit() {
    struct DummyInner;

    let _take = new(DummyInner, usize::MAX);
}

#[test]
fn test_new_with_small_limit() {
    struct DummyInner;

    let take = new(DummyInner, 1);
    assert_eq!(take.limit, 1);
}

