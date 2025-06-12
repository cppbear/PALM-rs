// Answer 0

#[test]
fn test_find_at_valid_match() {
    struct ExecReadOnly;
    let ro = Arc::new(ExecReadOnly);
    let cache = RefCell::new(ProgramCacheInner(HashMap::new()));
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let regex = ExecNoSyncStr(exec);
    
    // Assuming there's a hypothetical implementation of `find_at` that looks for the pattern "ab" in the string.
    // This is a place to adjust if the actual pattern or functionality is different.
    let result = regex.find_at("abcde", 0);
    assert_eq!(result, Some((0, 2))); // Match at start
}

#[test]
fn test_find_at_no_match() {
    struct ExecReadOnly;
    let ro = Arc::new(ExecReadOnly);
    let cache = RefCell::new(ProgramCacheInner(HashMap::new()));
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let regex = ExecNoSyncStr(exec);
    
    let result = regex.find_at("abcde", 3); // Start searching from index 3
    assert_eq!(result, None); // No match expected beyond this point
}

#[test]
fn test_find_at_empty_string() {
    struct ExecReadOnly;
    let ro = Arc::new(ExecReadOnly);
    let cache = RefCell::new(ProgramCacheInner(HashMap::new()));
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let regex = ExecNoSyncStr(exec);
    
    let result = regex.find_at("", 0); // Searching in an empty string
    assert_eq!(result, None); // No match expected
}

#[test]
fn test_find_at_start_index_out_of_bounds() {
    struct ExecReadOnly;
    let ro = Arc::new(ExecReadOnly);
    let cache = RefCell::new(ProgramCacheInner(HashMap::new()));
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let regex = ExecNoSyncStr(exec);
    
    let text = "abcd";
    let result = regex.find_at(text, 5); // Start beyond the string length
    assert_eq!(result, None); // No match expected
}

#[test]
#[should_panic]
fn test_find_at_panic_on_negative_index() {
    struct ExecReadOnly;
    let ro = Arc::new(ExecReadOnly);
    let cache = RefCell::new(ProgramCacheInner(HashMap::new()));
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let regex = ExecNoSyncStr(exec);
    
    let _ = regex.find_at("abcd", usize::MAX); // Panic expected when using an invalid index
}

