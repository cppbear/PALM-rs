// Answer 0

#[test]
fn test_read_captures_at_empty_text_start_zero() {
    let text = "";
    let start = 0;
    let locs = &mut Locations(Vec::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &Arc::new(ExecReadOnly),
        cache: &RefCell::new(ProgramCacheInner::default()),
    });
    let _ = exec.read_captures_at(locs, text, start);
}

#[test]
fn test_read_captures_at_non_empty_text_start_zero() {
    let text = "sample text";
    let start = 0;
    let locs = &mut Locations(Vec::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &Arc::new(ExecReadOnly),
        cache: &RefCell::new(ProgramCacheInner::default()),
    });
    let _ = exec.read_captures_at(locs, text, start);
}

#[test]
fn test_read_captures_at_non_empty_text_start_within_bounds() {
    let text = "sample text";
    let start = 5;
    let locs = &mut Locations(Vec::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &Arc::new(ExecReadOnly),
        cache: &RefCell::new(ProgramCacheInner::default()),
    });
    let _ = exec.read_captures_at(locs, text, start);
}

#[test]
fn test_read_captures_at_full_text_length() {
    let text = "sample text";
    let start = text.len();
    let locs = &mut Locations(Vec::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &Arc::new(ExecReadOnly),
        cache: &RefCell::new(ProgramCacheInner::default()),
    });
    let _ = exec.read_captures_at(locs, text, start);
}

#[test]
fn test_read_captures_at_with_empty_locations() {
    let text = "sample text";
    let start = 0;
    let locs = &mut Locations(Vec::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &Arc::new(ExecReadOnly),
        cache: &RefCell::new(ProgramCacheInner::default()),
    });
    let _ = exec.read_captures_at(locs, text, start);
}

#[test]
fn test_read_captures_at_panic_start_greater_than_length() {
    let text = "sample text";
    let start = 11; // start exceeds text length
    let locs = &mut Locations(Vec::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &Arc::new(ExecReadOnly),
        cache: &RefCell::new(ProgramCacheInner::default()),
    });
    let _ = exec.read_captures_at(locs, text, start);
}

#[test]
fn test_read_captures_at_large_text() {
    let text = "a".repeat(2048);  // text length is 2048
    let start = 0;
    let locs = &mut Locations(Vec::new());
    let exec = ExecNoSyncStr(ExecNoSync {
        ro: &Arc::new(ExecReadOnly),
        cache: &RefCell::new(ProgramCacheInner::default()),
    });
    let _ = exec.read_captures_at(locs, text, start);
}

