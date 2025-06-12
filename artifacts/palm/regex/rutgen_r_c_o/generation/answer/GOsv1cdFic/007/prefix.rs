// Answer 0

#[test]
fn test_forward_many_empty_text() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };
    
    let mut cache_inner = ProgramCacheInner {
        pikevm: pikevm::Cache::new(),
        backtrack: backtrack::Cache::new(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    };
    
    let cache = ProgramCache::new(&mut cache_inner);
    let mut matches = vec![false; prog.matches.len()];
    let text = b"";
    let at = 0;
    
    forward_many(&prog, &cache, &mut matches, text, at);
}

#[test]
fn test_forward_many_single_match() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };
    
    let mut cache_inner = ProgramCacheInner {
        pikevm: pikevm::Cache::new(),
        backtrack: backtrack::Cache::new(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    };
    
    let cache = ProgramCache::new(&mut cache_inner);
    let mut matches = vec![false; prog.matches.len()];
    let text = b"";
    let at = 0;
    
    forward_many(&prog, &cache, &mut matches, text, at);
}

#[test]
fn test_forward_many_multiple_matches() {
    let prog = Program {
        insts: vec![Inst::Char(InstChar::new(b'a')), Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };
    
    let mut cache_inner = ProgramCacheInner {
        pikevm: pikevm::Cache::new(),
        backtrack: backtrack::Cache::new(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    };
    
    let cache = ProgramCache::new(&mut cache_inner);
    let mut matches = vec![false; prog.matches.len()];
    let text = b"abc";
    let at = 0;
    
    forward_many(&prog, &cache, &mut matches, text, at);
} 

#[test]
fn test_forward_many_no_match() {
    let prog = Program {
        insts: vec![Inst::Char(InstChar::new(b'a')), Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };
    
    let mut cache_inner = ProgramCacheInner {
        pikevm: pikevm::Cache::new(),
        backtrack: backtrack::Cache::new(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    };
    
    let cache = ProgramCache::new(&mut cache_inner);
    let mut matches = vec![false; prog.matches.len()];
    let text = b"xyz";
    let at = 0;
    
    forward_many(&prog, &cache, &mut matches, text, at);
}

