// Answer 0

#[test]
fn test_clear_with_no_jobs_and_full_visited() {
    struct MockMemory {
        jobs: Vec<String>,
        visited: Vec<u32>,
    }

    struct BacktrackEngine {
        prog: Vec<u8>,
        input: Vec<u8>,
        m: MockMemory,
    }

    impl BacktrackEngine {
        fn new() -> Self {
            BacktrackEngine {
                prog: vec![1, 2, 3],
                input: vec![4, 5, 6, 7],
                m: MockMemory {
                    jobs: Vec::new(),
                    visited: vec![0; 10], // Initialize visited with 10 elements
                },
            }
        }

        fn clear(&mut self) {
            self.m.jobs.clear();
            let visited_len = (self.prog.len() * (self.input.len() + 1) + 31) / 32;
            self.m.visited.truncate(visited_len);
            for v in &mut self.m.visited {
                *v = 0;
            }
            if visited_len > self.m.visited.len() {
                let len = self.m.visited.len();
                self.m.visited.reserve_exact(visited_len - len);
                for _ in 0..(visited_len - len) {
                    self.m.visited.push(0);
                }
            }
        }
    }

    let mut engine = BacktrackEngine::new();
    engine.clear();

    assert_eq!(engine.m.jobs.len(), 0);
    assert_eq!(engine.m.visited, vec![0; 10]); // Verified visited has been cleared to zero
}

#[test]
fn test_clear_with_no_jobs_and_exceeding_visited() {
    struct MockMemory {
        jobs: Vec<String>,
        visited: Vec<u32>,
    }

    struct BacktrackEngine {
        prog: Vec<u8>,
        input: Vec<u8>,
        m: MockMemory,
    }

    impl BacktrackEngine {
        fn new() -> Self {
            BacktrackEngine {
                prog: vec![1, 2, 3],
                input: vec![4, 5, 6, 7],
                m: MockMemory {
                    jobs: Vec::new(),
                    visited: vec![0; 3], // Less than needed
                },
            }
        }

        fn clear(&mut self) {
            self.m.jobs.clear();
            let visited_len = (self.prog.len() * (self.input.len() + 1) + 31) / 32;
            self.m.visited.truncate(visited_len);
            for v in &mut self.m.visited {
                *v = 0;
            }
            if visited_len > self.m.visited.len() {
                let len = self.m.visited.len();
                self.m.visited.reserve_exact(visited_len - len);
                for _ in 0..(visited_len - len) {
                    self.m.visited.push(0);
                }
            }
        }
    }

    let mut engine = BacktrackEngine::new();
    engine.clear();

    let expected_length = (engine.prog.len() * (engine.input.len() + 1) + 31) / 32;
    assert_eq!(engine.m.jobs.len(), 0);
    assert_eq!(engine.m.visited.len(), expected_length); // Verified visited has grown to required size
    assert_eq!(engine.m.visited.iter().all(|&v| v == 0), true); // All visited should be zero
}

