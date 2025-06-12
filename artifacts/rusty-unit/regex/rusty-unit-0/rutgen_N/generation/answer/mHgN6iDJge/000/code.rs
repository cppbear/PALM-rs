// Answer 0

#[derive(Debug)]
struct Program;

struct Cache {
    jobs: Vec<i32>,
    visited: Vec<i32>,
}

impl Cache {
    pub fn new(_prog: &Program) -> Self {
        Cache { jobs: vec![], visited: vec![] }
    }
}

#[test]
fn test_new_cache_creates_empty_cache() {
    let prog = Program;
    let cache = Cache::new(&prog);
    assert_eq!(cache.jobs.len(), 0);
    assert_eq!(cache.visited.len(), 0);
}

