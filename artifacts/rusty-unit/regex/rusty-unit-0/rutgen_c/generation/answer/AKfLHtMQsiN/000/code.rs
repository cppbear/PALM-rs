// Answer 0

#[test]
fn test_read_captures_at_valid_case() {
    struct ExecReadOnlyStub;
    impl ExecReadOnlyStub {
        fn read_captures_at(&self, locs: &mut Locations, text: &[u8], start: usize) -> Option<(usize, usize)> {
            if start < text.len() {
                Some((start, start + 1)) // Dummy implementation for testing
            } else {
                None
            }
        }
    }
    
    let read_only = Arc::new(ExecReadOnlyStub {});
    let cache = RefCell::new(ProgramCacheInner(HashMap::new()));
    let exec = ExecNoSync { ro: &read_only, cache: &cache };
    let exec_no_sync_str = ExecNoSyncStr(exec);
    
    let mut locations = Locations(vec![]);
    let result = exec_no_sync_str.read_captures_at(&mut locations, "test", 0);
    assert_eq!(result, Some((0, 1)));
}

#[test]
fn test_read_captures_at_out_of_bounds() {
    struct ExecReadOnlyStub;
    impl ExecReadOnlyStub {
        fn read_captures_at(&self, locs: &mut Locations, text: &[u8], start: usize) -> Option<(usize, usize)> {
            if start < text.len() {
                Some((start, start + 1))
            } else {
                None
            }
        }
    }
    
    let read_only = Arc::new(ExecReadOnlyStub {});
    let cache = RefCell::new(ProgramCacheInner(HashMap::new()));
    let exec = ExecNoSync { ro: &read_only, cache: &cache };
    let exec_no_sync_str = ExecNoSyncStr(exec);
    
    let mut locations = Locations(vec![]);
    let result = exec_no_sync_str.read_captures_at(&mut locations, "test", 4);
    assert_eq!(result, None);
}

