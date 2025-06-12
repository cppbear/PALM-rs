// Answer 0

#[test]
fn test_shortest_match_at_found() {
    struct ExecReadOnlyMock;
    let exec_read_only = Arc::new(ExecReadOnlyMock {});
    let cache = RefCell::new(ProgramCacheInner(HashMap::new()));
    let exec_no_sync = ExecNoSync { ro: &exec_read_only, cache: &cache };

    impl ExecReadOnlyMock {
        fn shortest_match_at(&self, text: &[u8], start: usize) -> Option<usize> {
            if start < text.len() && text[start] == b'a' {
                return Some(start);
            }
            None
        }
    }

    let exec_no_sync_str = ExecNoSyncStr(exec_no_sync);
    let result = exec_no_sync_str.shortest_match_at("abcde", 0);
    assert_eq!(result, Some(0));
}

#[test]
fn test_shortest_match_at_not_found() {
    struct ExecReadOnlyMock;
    let exec_read_only = Arc::new(ExecReadOnlyMock {});
    let cache = RefCell::new(ProgramCacheInner(HashMap::new()));
    let exec_no_sync = ExecNoSync { ro: &exec_read_only, cache: &cache };

    impl ExecReadOnlyMock {
        fn shortest_match_at(&self, text: &[u8], start: usize) -> Option<usize> {
            if start < text.len() && text[start] == b'a' {
                return Some(start);
            }
            None
        }
    }

    let exec_no_sync_str = ExecNoSyncStr(exec_no_sync);
    let result = exec_no_sync_str.shortest_match_at("bcdef", 0);
    assert_eq!(result, None);
}

#[test]
fn test_shortest_match_at_boundary_start() {
    struct ExecReadOnlyMock;
    let exec_read_only = Arc::new(ExecReadOnlyMock {});
    let cache = RefCell::new(ProgramCacheInner(HashMap::new()));
    let exec_no_sync = ExecNoSync { ro: &exec_read_only, cache: &cache };

    impl ExecReadOnlyMock {
        fn shortest_match_at(&self, text: &[u8], start: usize) -> Option<usize> {
            if start < text.len() && text[start] == b'a' {
                return Some(start);
            }
            None
        }
    }

    let exec_no_sync_str = ExecNoSyncStr(exec_no_sync);
    let result = exec_no_sync_str.shortest_match_at("a", 0);
    assert_eq!(result, Some(0));
}

#[test]
fn test_shortest_match_at_boundary_out_of_bounds() {
    struct ExecReadOnlyMock;
    let exec_read_only = Arc::new(ExecReadOnlyMock {});
    let cache = RefCell::new(ProgramCacheInner(HashMap::new()));
    let exec_no_sync = ExecNoSync { ro: &exec_read_only, cache: &cache };

    impl ExecReadOnlyMock {
        fn shortest_match_at(&self, text: &[u8], start: usize) -> Option<usize> {
            if start < text.len() && text[start] == b'a' {
                return Some(start);
            }
            None
        }
    }

    let exec_no_sync_str = ExecNoSyncStr(exec_no_sync);
    let result = exec_no_sync_str.shortest_match_at("abc", 3);
    assert_eq!(result, None);
}

