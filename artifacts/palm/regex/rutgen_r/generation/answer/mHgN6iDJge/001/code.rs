// Answer 0

#[test]
fn test_new_cache_empty_program() {
    struct Program;

    struct Cache {
        jobs: Vec<u8>,
        visited: Vec<u8>,
    }

    impl Cache {
        pub fn new(_prog: &Program) -> Self {
            Cache { jobs: vec![], visited: vec![] }
        }
    }

    let program = Program;
    let cache = Cache::new(&program);

    assert_eq!(cache.jobs, vec![]);
    assert_eq!(cache.visited, vec![]);
}

#[test]
fn test_new_cache_different_program() {
    struct Program;

    struct Cache {
        jobs: Vec<u8>,
        visited: Vec<u8>,
    }

    impl Cache {
        pub fn new(_prog: &Program) -> Self {
            Cache { jobs: vec![], visited: vec![] }
        }
    }

    let program = Program;
    let cache = Cache::new(&program);

    assert_eq!(cache.jobs, vec![]);
    assert_eq!(cache.visited, vec![]);
}

