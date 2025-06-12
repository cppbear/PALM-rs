// Answer 0

#[test]
fn test_is_match_at_empty_string() {
    let ro = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync { ro: &ro, cache: &cache });
    exec.is_match_at("", 0);
}

#[test]
fn test_is_match_at_single_character() {
    let ro = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync { ro: &ro, cache: &cache });
    exec.is_match_at("a", 0);
}

#[test]
fn test_is_match_at_multiple_characters() {
    let ro = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync { ro: &ro, cache: &cache });
    exec.is_match_at("abc", 2);
}

#[test]
fn test_is_match_at_start_out_of_bounds() {
    let ro = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync { ro: &ro, cache: &cache });
    exec.is_match_at("abc", 3); // out of bounds
}

#[test]
#[should_panic]
fn test_is_match_at_start_max() {
    let ro = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync { ro: &ro, cache: &cache });
    exec.is_match_at("abc", usize::MAX); // panic condition
}

#[test]
fn test_is_match_at_partial_string() {
    let ro = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync { ro: &ro, cache: &cache });
    exec.is_match_at("abcd", 1);
}

#[test]
#[should_panic]
fn test_is_match_at_single_character_out_of_bounds() {
    let ro = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync { ro: &ro, cache: &cache });
    exec.is_match_at("a", 1); // panic condition
}

#[test]
fn test_is_match_at_two_characters() {
    let ro = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync { ro: &ro, cache: &cache });
    exec.is_match_at("ab", 0);
}

#[test]
fn test_is_match_at_showing_progress() {
    let ro = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync { ro: &ro, cache: &cache });
    exec.is_match_at("abc", 1);
}

#[test]
#[should_panic]
fn test_is_match_at_too_far() {
    let ro = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSyncStr(ExecNoSync { ro: &ro, cache: &cache });
    exec.is_match_at("abc", 4); // panic condition
}

