// Answer 0

#[test]
fn test_start_flags_at_zero_empty_text() {
    let text = b"";
    let at = 0;
    let fsm = Fsm { 
        prog: &Program::default(), 
        start: STATE_START, 
        at: 0, 
        quit_after_match: false, 
        last_match_si: STATE_UNKNOWN, 
        last_cache_flush: 0, 
        cache: &mut CacheInner::default()
    };
    let (empty_flags, state_flags) = fsm.start_flags(text, at);
}

#[test]
fn test_start_flags_at_zero_space() {
    let text = b" ";
    let at = 0;
    let fsm = Fsm { 
        prog: &Program::default(), 
        start: STATE_START, 
        at: 0, 
        quit_after_match: false, 
        last_match_si: STATE_UNKNOWN, 
        last_cache_flush: 0, 
        cache: &mut CacheInner::default()
    };
    let (empty_flags, state_flags) = fsm.start_flags(text, at);
}

#[test]
fn test_start_flags_at_one_single_char() {
    let text = b"a";
    let at = 1;
    let fsm = Fsm { 
        prog: &Program::default(), 
        start: STATE_START, 
        at: 1, 
        quit_after_match: false, 
        last_match_si: STATE_UNKNOWN, 
        last_cache_flush: 0, 
        cache: &mut CacheInner::default()
    };
    let (empty_flags, state_flags) = fsm.start_flags(text, at);
}

#[test]
fn test_start_flags_at_one_newline() {
    let text = b"\n";
    let at = 1;
    let fsm = Fsm { 
        prog: &Program::default(), 
        start: STATE_START, 
        at: 1, 
        quit_after_match: false, 
        last_match_si: STATE_UNKNOWN, 
        last_cache_flush: 0, 
        cache: &mut CacheInner::default()
    };
    let (empty_flags, state_flags) = fsm.start_flags(text, at);
}

#[test]
fn test_start_flags_at_two_ab() {
    let text = b"ab";
    let at = 2;
    let fsm = Fsm { 
        prog: &Program::default(), 
        start: STATE_START, 
        at: 2, 
        quit_after_match: false, 
        last_match_si: STATE_UNKNOWN, 
        last_cache_flush: 0, 
        cache: &mut CacheInner::default()
    };
    let (empty_flags, state_flags) = fsm.start_flags(text, at);
}

#[test]
fn test_start_flags_at_one_space() {
    let text = b" ";
    let at = 1;
    let fsm = Fsm { 
        prog: &Program::default(), 
        start: STATE_START, 
        at: 1, 
        quit_after_match: false, 
        last_match_si: STATE_UNKNOWN, 
        last_cache_flush: 0, 
        cache: &mut CacheInner::default()
    };
    let (empty_flags, state_flags) = fsm.start_flags(text, at);
}

#[test]
fn test_start_flags_at_two_spaces() {
    let text = b"  ";
    let at = 2;
    let fsm = Fsm { 
        prog: &Program::default(), 
        start: STATE_START, 
        at: 2, 
        quit_after_match: false, 
        last_match_si: STATE_UNKNOWN, 
        last_cache_flush: 0, 
        cache: &mut CacheInner::default()
    };
    let (empty_flags, state_flags) = fsm.start_flags(text, at);
}

#[test]
fn test_start_flags_at_two_space_a() {
    let text = b"a ";
    let at = 2;
    let fsm = Fsm { 
        prog: &Program::default(), 
        start: STATE_START, 
        at: 2, 
        quit_after_match: false, 
        last_match_si: STATE_UNKNOWN, 
        last_cache_flush: 0, 
        cache: &mut CacheInner::default()
    };
    let (empty_flags, state_flags) = fsm.start_flags(text, at);
}

#[test]
fn test_start_flags_at_two_newline_space() {
    let text = b"  \n";
    let at = 2;
    let fsm = Fsm { 
        prog: &Program::default(), 
        start: STATE_START, 
        at: 2, 
        quit_after_match: false, 
        last_match_si: STATE_UNKNOWN, 
        last_cache_flush: 0, 
        cache: &mut CacheInner::default()
    };
    let (empty_flags, state_flags) = fsm.start_flags(text, at);
}

#[test]
fn test_start_flags_at_one_tab_space() {
    let text = b"\tf";
    let at = 1;
    let fsm = Fsm { 
        prog: &Program::default(), 
        start: STATE_START, 
        at: 1, 
        quit_after_match: false, 
        last_match_si: STATE_UNKNOWN, 
        last_cache_flush: 0, 
        cache: &mut CacheInner::default()
    };
    let (empty_flags, state_flags) = fsm.start_flags(text, at);
}

#[test]
fn test_start_flags_at_zero_tab() {
    let text = b"\t";
    let at = 0;
    let fsm = Fsm { 
        prog: &Program::default(), 
        start: STATE_START, 
        at: 0, 
        quit_after_match: false, 
        last_match_si: STATE_UNKNOWN, 
        last_cache_flush: 0, 
        cache: &mut CacheInner::default()
    };
    let (empty_flags, state_flags) = fsm.start_flags(text, at);
}

