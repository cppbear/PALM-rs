// Answer 0

#[test]
fn test_shortest_match_at_empty_string() {
    let locs = Locations::new();
    let exec_read_only = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    });
    let _ = exec.shortest_match_at("", 0);
}

#[test]
fn test_shortest_match_at_single_character() {
    let locs = Locations::new();
    let exec_read_only = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    });
    let _ = exec.shortest_match_at("a", 0);
}

#[test]
fn test_shortest_match_at_out_of_bounds_start() {
    let locs = Locations::new();
    let exec_read_only = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    });
    let _ = exec.shortest_match_at("a", 1);
}

#[test]
fn test_shortest_match_at_invalid_start_negative() {
    let locs = Locations::new();
    let exec_read_only = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    });
    let _ = exec.shortest_match_at("a", usize::MAX);
}

