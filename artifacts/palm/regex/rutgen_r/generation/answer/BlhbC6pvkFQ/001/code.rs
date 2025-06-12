// Answer 0

#[test]
fn test_clear_cache_false_return() {
    struct TestDFA {
        at: usize,
        last_cache_flush: usize,
        cache: TestCache,
        start: usize,
        last_match_si: usize,
    }

    struct TestCache {
        flush_count: usize,
        states: Vec<usize>,
        start_states: Vec<usize>,
        trans: Vec<usize>,
        compiled: Vec<usize>,
    }

    impl TestDFA {
        fn clear_cache(&mut self) -> bool {
            let nstates = self.cache.states.len();
            if self.cache.flush_count >= 3
                && self.at >= self.last_cache_flush
                && (self.at - self.last_cache_flush) <= 10 * nstates {
                return false;
            }
            self.last_cache_flush = self.at;
            self.cache.flush_count += 1;
            let start = self.start;
            self.cache.trans.clear();
            self.cache.states.clear();
            self.cache.start_states.iter_mut().for_each(|s| *s = 0);
            true
        }
    }

    let mut dfa = TestDFA {
        at: 30,
        last_cache_flush: 30,
        cache: TestCache {
            flush_count: 3,
            states: vec![1, 2, 3],
            start_states: vec![0; 3],
            trans: vec![],
            compiled: vec![],
        },
        start: 0,
        last_match_si: 0,
    };

    let result = dfa.clear_cache();
    assert_eq!(result, false);
}

