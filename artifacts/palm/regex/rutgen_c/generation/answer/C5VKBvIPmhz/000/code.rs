// Answer 0

#[test]
fn test_next_after_empty() {
    struct TestExecReadOnly;
    let ro = Arc::new(TestExecReadOnly);
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    assert_eq!(exec.next_after_empty(b"test", 0), 1);
    assert_eq!(exec.next_after_empty(b"test", 1), 2);
    assert_eq!(exec.next_after_empty(b"test", 99), 100);
}

