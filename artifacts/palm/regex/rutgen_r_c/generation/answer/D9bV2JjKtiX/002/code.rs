// Answer 0

#[test]
fn test_backtrack_with_empty_jobs() {
    struct TestInput;

    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 0, c: 'a', byte: Some(97), len: 1 }
        }
        fn next_char(&self, _: InputAt) -> Char { 'a' }
        fn previous_char(&self, _: InputAt) -> Char { 'a' }
        fn is_empty_match(&self, _: InputAt, _: &InstEmptyLook) -> bool { false }
        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> { None }
        fn len(&self) -> usize { 1 }
        fn is_empty(&self) -> bool { false }
        fn as_bytes(&self) -> &[u8] { b"a" }
    }

    let program = Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 0 };
    let mut cache = Cache { clist: Threads::new(), nlist: Threads::new(), stack: vec![] };
    let mut matches = vec![false; 1];
    let mut slots = vec![None; 1];
    
    let mut bounded = Bounded { prog: &program, input: TestInput, matches: &mut matches, slots: &mut slots, m: &mut cache };

    let result = bounded.backtrack(InputAt { pos: 0, c: 'a', byte: Some(97), len: 1 });
    assert!(!result);
}

#[test]
fn test_backtrack_with_jobs_and_save_restore() {
    struct TestInput;

    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 0, c: 'a', byte: Some(97), len: 1 }
        }
        fn next_char(&self, _: InputAt) -> Char { 'a' }
        fn previous_char(&self, _: InputAt) -> Char { 'a' }
        fn is_empty_match(&self, _: InputAt, _: &InstEmptyLook) -> bool { false }
        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> { None }
        fn len(&self) -> usize { 1 }
        fn is_empty(&self) -> bool { false }
        fn as_bytes(&self) -> &[u8] { b"a" }
    }

    let program = Program { insts: vec![], matches: vec![InstPtr(0)], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 0 };
    let mut cache = Cache { clist: Threads::new(), nlist: Threads::new(), stack: vec![], jobs: vec![] };
    let mut matches = vec![false; 1];
    let mut slots = vec![Some(0); 1]; 

    let mut bounded = Bounded { prog: &program, input: TestInput, matches: &mut matches, slots: &mut slots, m: &mut cache };

    bounded.m.jobs.push(Job::SaveRestore { slot: 0, old_pos: Some(1) });

    let result = bounded.backtrack(InputAt { pos: 0, c: 'a', byte: Some(97), len: 1 });
    assert!(!result);
}

