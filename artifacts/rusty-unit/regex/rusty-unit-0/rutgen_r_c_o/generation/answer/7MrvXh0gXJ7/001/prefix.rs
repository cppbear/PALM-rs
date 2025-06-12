// Answer 0

#[test]
fn test_start_flags_case1() {
    let text = b"Hello, world!";
    let at = 1;
    let fsm = Fsm { 
        prog: &Program::default(), 
        start: STATE_START, 
        at, 
        quit_after_match: false, 
        last_match_si: STATE_UNKNOWN, 
        last_cache_flush: 0, 
        cache: &mut CacheInner::default()
    };
    fsm.start_flags(text, at);
}

#[test]
fn test_start_flags_case2() {
    let text = b"Hello";
    let at = 3;
    let fsm = Fsm { 
        prog: &Program::default(), 
        start: STATE_START, 
        at, 
        quit_after_match: false, 
        last_match_si: STATE_UNKNOWN, 
        last_cache_flush: 0, 
        cache: &mut CacheInner::default()
    };
    fsm.start_flags(text, at);
}

#[test]
fn test_start_flags_case3() {
    let text = b"Hello there!";
    let at = 6;
    let fsm = Fsm { 
        prog: &Program::default(), 
        start: STATE_START, 
        at, 
        quit_after_match: false, 
        last_match_si: STATE_UNKNOWN, 
        last_cache_flush: 0, 
        cache: &mut CacheInner::default()
    };
    fsm.start_flags(text, at);
}

#[test]
fn test_start_flags_case4() {
    let text = b"A quick brown fox";
    let at = 5;
    let fsm = Fsm { 
        prog: &Program::default(), 
        start: STATE_START, 
        at, 
        quit_after_match: false, 
        last_match_si: STATE_UNKNOWN, 
        last_cache_flush: 0, 
        cache: &mut CacheInner::default()
    };
    fsm.start_flags(text, at);
}

#[test]
fn test_start_flags_case5() {
    let text = b"Word_boundary_test";
    let at = 4;
    let fsm = Fsm { 
        prog: &Program::default(), 
        start: STATE_START, 
        at, 
        quit_after_match: false, 
        last_match_si: STATE_UNKNOWN, 
        last_cache_flush: 0, 
        cache: &mut CacheInner::default()
    };
    fsm.start_flags(text, at);
}

