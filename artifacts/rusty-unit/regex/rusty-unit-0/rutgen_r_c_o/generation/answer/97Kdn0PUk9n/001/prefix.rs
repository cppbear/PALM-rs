// Answer 0

#[test]
fn test_next_after_empty_empty_string() {
    let text = "";
    let i = 0;
    let exec = ExecNoSyncStr(ExecNoSync { ro: &Arc::new(ExecReadOnly {}), cache: &RefCell::new(ProgramCacheInner::new()) });
    exec.next_after_empty(text, i);
}

#[test]
fn test_next_after_empty_single_byte() {
    let text = "a";
    let i = 0;
    let exec = ExecNoSyncStr(ExecNoSync { ro: &Arc::new(ExecReadOnly {}), cache: &RefCell::new(ProgramCacheInner::new()) });
    exec.next_after_empty(text, i);
}

#[test]
fn test_next_after_empty_double_byte() {
    let text = "é";
    let i = 0;
    let exec = ExecNoSyncStr(ExecNoSync { ro: &Arc::new(ExecReadOnly {}), cache: &RefCell::new(ProgramCacheInner::new()) });
    exec.next_after_empty(text, i);
}

#[test]
fn test_next_after_empty_triple_byte() {
    let text = "€";
    let i = 0;
    let exec = ExecNoSyncStr(ExecNoSync { ro: &Arc::new(ExecReadOnly {}), cache: &RefCell::new(ProgramCacheInner::new()) });
    exec.next_after_empty(text, i);
}

#[test]
fn test_next_after_empty_four_byte() {
    let text = "𐍈"; // U+10300
    let i = 0;
    let exec = ExecNoSyncStr(ExecNoSync { ro: &Arc::new(ExecReadOnly {}), cache: &RefCell::new(ProgramCacheInner::new()) });
    exec.next_after_empty(text, i);
}

#[test]
fn test_next_after_empty_multi_byte() {
    let text = "abcé";
    let i = 3;
    let exec = ExecNoSyncStr(ExecNoSync { ro: &Arc::new(ExecReadOnly {}), cache: &RefCell::new(ProgramCacheInner::new()) });
    exec.next_after_empty(text, i);
}

#[test]
fn test_next_after_empty_out_of_bounds() {
    let text = "abc"; // Only 3 bytes
    let i = 3;
    let exec = ExecNoSyncStr(ExecNoSync { ro: &Arc::new(ExecReadOnly {}), cache: &RefCell::new(ProgramCacheInner::new()) });
    exec.next_after_empty(text, i);
}

