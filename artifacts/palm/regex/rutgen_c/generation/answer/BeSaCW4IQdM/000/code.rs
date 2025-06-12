// Answer 0

#[test]
fn test_is_match_at_empty_string() {
    let ro = Arc::new(ExecReadOnly { /* initialize as needed */ });
    let cache: ProgramCache = RefCell::new(ProgramCacheInner { /* initialize as needed */ });
    let exec_no_sync = ExecNoSync { ro: &ro, cache: &cache };
    let regex = ExecNoSyncStr(exec_no_sync);
    
    let result = regex.is_match_at("", 0);
    assert!(!result, "Expected no match for empty string");
}

#[test]
fn test_is_match_at_no_match() {
    let ro = Arc::new(ExecReadOnly { /* initialize as needed */ });
    let cache: ProgramCache = RefCell::new(ProgramCacheInner { /* initialize as needed */ });
    let exec_no_sync = ExecNoSync { ro: &ro, cache: &cache };
    let regex = ExecNoSyncStr(exec_no_sync);
    
    let result = regex.is_match_at("abcdef", 0);
    assert!(!result, "Expected no match for non-matching string");
}

#[test]
fn test_is_match_at_exact_match() {
    let ro = Arc::new(ExecReadOnly { /* initialize as needed */ });
    let cache: ProgramCache = RefCell::new(ProgramCacheInner { /* initialize as needed */ });
    let exec_no_sync = ExecNoSync { ro: &ro, cache: &cache };
    let regex = ExecNoSyncStr(exec_no_sync);
    
    // Assume the regex is initialized in a way that will match "abc" at start 0
    let result = regex.is_match_at("abc", 0);
    assert!(result, "Expected match for exact string");
}

#[test]
fn test_is_match_at_partial_match() {
    let ro = Arc::new(ExecReadOnly { /* initialize as needed */ });
    let cache: ProgramCache = RefCell::new(ProgramCacheInner { /* initialize as needed */ });
    let exec_no_sync = ExecNoSync { ro: &ro, cache: &cache };
    let regex = ExecNoSyncStr(exec_no_sync);
    
    // Assume the regex can match partial strings in context
    let result = regex.is_match_at("abcdefgh", 0);
    assert!(result, "Expected match for matching prefix");
}

#[test]
fn test_is_match_at_boundary_conditions() {
    let ro = Arc::new(ExecReadOnly { /* initialize as needed */ });
    let cache: ProgramCache = RefCell::new(ProgramCacheInner { /* initialize as needed */ });
    let exec_no_sync = ExecNoSync { ro: &ro, cache: &cache };
    let regex = ExecNoSyncStr(exec_no_sync);
    
    let result = regex.is_match_at("abcabc", 3);
    assert!(result, "Expected match at boundary condition");
    
    let result = regex.is_match_at("abcabc", 6);
    assert!(!result, "Expected no match out of bounds");
}

