// Answer 0

#[test]
fn test_next_after_empty_with_valid_index() {
    let exec_no_sync = ExecNoSyncStr(ExecNoSync {
        ro: &Arc::new(ExecReadOnly {}),
        cache: &RefCell::new(ProgramCacheInner {
            patterns: HashMap::new(),
        }),
    });

    let text = "Hello, 🌍!";
    let index = 5; // Index after ', '
    let result = exec_no_sync.next_after_empty(text, index);
    assert_eq!(result, 7); // Expected to skip the 2-byte emoji
}

#[test]
fn test_next_after_empty_with_index_at_end() {
    let exec_no_sync = ExecNoSyncStr(ExecNoSync {
        ro: &Arc::new(ExecReadOnly {}),
        cache: &RefCell::new(ProgramCacheInner {
            patterns: HashMap::new(),
        }),
    });

    let text = "Hello, 🌍!";
    let index = text.len(); // Index at the end
    let result = exec_no_sync.next_after_empty(text, index);
    assert_eq!(result, index + 1); // Expected to go beyond the end
}

#[test]
fn test_next_after_empty_with_invalid_index() {
    let exec_no_sync = ExecNoSyncStr(ExecNoSync {
        ro: &Arc::new(ExecReadOnly {}),
        cache: &RefCell::new(ProgramCacheInner {
            patterns: HashMap::new(),
        }),
    });

    let text = "Hello, 🌍!";
    let index = 100; // Out of bounds
    let result = exec_no_sync.next_after_empty(text, index);
    assert_eq!(result, index + 1); // Expected to go beyond the out of bounds index
}

#[test]
fn test_next_after_empty_with_utf8_character() {
    let exec_no_sync = ExecNoSyncStr(ExecNoSync {
        ro: &Arc::new(ExecReadOnly {}),
        cache: &RefCell::new(ProgramCacheInner {
            patterns: HashMap::new(),
        }),
    });

    let text = "🌍!";
    let index = 0; // Start at the beginning
    let result = exec_no_sync.next_after_empty(text, index);
    assert_eq!(result, 4); // Expected to skip the full emoji character
}

