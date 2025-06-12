// Answer 0

#[test]
fn test_next_after_empty_basic() {
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &Arc::new(ExecReadOnly {}),
        cache: &RefCell::new(ProgramCacheInner::default()),
    });
    let text = "hello";
    let index = 0;
    let result = exec.next_after_empty(text, index);
    assert_eq!(result, 1); // 'h' (1 byte)
}

#[test]
fn test_next_after_empty_utf8() {
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &Arc::new(ExecReadOnly {}),
        cache: &RefCell::new(ProgramCacheInner::default()),
    });
    let text = "こんにちは"; // "Hello" in Japanese (3 bytes each for 'こ', 'ん', 'に')
    let index = 0;
    let result = exec.next_after_empty(text, index);
    assert_eq!(result, 3); // 'こ'
    
    let index = 3; // after first 'こ'
    let result = exec.next_after_empty(text, index);
    assert_eq!(result, 6); // 'ん'
    
    let index = 6; // after 'ん'
    let result = exec.next_after_empty(text, index);
    assert_eq!(result, 9); // 'に'
}

#[test]
fn test_next_after_empty_invalid_index() {
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &Arc::new(ExecReadOnly {}),
        cache: &RefCell::new(ProgramCacheInner::default()),
    });
    let text = "abc";
    let index = 3; // Index equal to text length
    let result = exec.next_after_empty(text, index);
    assert_eq!(result, 4); // Should return length + 1 (out of bounds)
}

#[test]
fn test_next_after_empty_empty_string() {
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &Arc::new(ExecReadOnly {}),
        cache: &RefCell::new(ProgramCacheInner::default()),
    });
    let text = "";
    let index = 0; // Start at 0
    let result = exec.next_after_empty(text, index);
    assert_eq!(result, 1); // Out of bounds so return 1
}

