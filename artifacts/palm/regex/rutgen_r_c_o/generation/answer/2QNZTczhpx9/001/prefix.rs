// Answer 0

#[test]
fn test_find_at_empty_string_start_zero() {
    let exec_read_only = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    });
    exec.find_at("", 0);
}

#[test]
fn test_find_at_single_character_start_zero() {
    let exec_read_only = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    });
    exec.find_at("a", 0);
}

#[test]
fn test_find_at_single_character_start_one() {
    let exec_read_only = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    });
    exec.find_at("a", 1);
}

#[test]
fn test_find_at_long_string_start_zero() {
    let exec_read_only = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    });
    exec.find_at("a very long string of characters", 0);
}

#[test]
fn test_find_at_long_string_start_mid() {
    let exec_read_only = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    });
    exec.find_at("a very long string of characters", 10);
}

#[test]
fn test_find_at_long_string_start_end() {
    let exec_read_only = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    });
    exec.find_at("a very long string of characters", 30);
}

#[test]
fn test_find_at_long_string_start_out_of_bounds() {
    let exec_read_only = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    });
    exec.find_at("a very long string of characters", 31);
}

#[test]
fn test_find_at_up_to_2_16_characters() {
    let long_string = "a".repeat(65536); // 2^16 characters
    let exec_read_only = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    });
    exec.find_at(&long_string, 0);
}

