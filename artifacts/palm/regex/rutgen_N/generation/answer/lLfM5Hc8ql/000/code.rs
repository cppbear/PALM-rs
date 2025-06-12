// Answer 0

#[derive(Default)]
struct MockMemory {
    jobs: Vec<u8>,
    visited: Vec<u64>,
}

struct BacktrackEngine<'a> {
    prog: &'a [u8],
    input: &'a [u8],
    m: MockMemory,
}

impl<'a> BacktrackEngine<'a> {
    fn clear(&mut self) {
        self.m.jobs.clear();
        let visited_len = (self.prog.len() * (self.input.len() + 1) + 63) / 64;
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

#[test]
fn test_clear_empty_jobs_and_visited() {
    let mut engine = BacktrackEngine {
        prog: &[1, 2, 3],
        input: &[4, 5, 6],
        m: MockMemory::default(),
    };
    
    engine.clear();
    
    assert!(engine.m.jobs.is_empty());
    assert!(engine.m.visited.is_empty());
}

#[test]
fn test_clear_with_jobs() {
    let mut engine = BacktrackEngine {
        prog: &[1, 2, 3],
        input: &[4, 5, 6],
        m: MockMemory {
            jobs: vec![1, 2, 3],
            visited: vec![1, 2],
        },
    };
    
    engine.clear();
    
    assert!(engine.m.jobs.is_empty());
    assert!(engine.m.visited.is_empty());
}

#[test]
fn test_clear_with_visited_length() {
    let mut engine = BacktrackEngine {
        prog: &[1, 2, 3],
        input: &[4, 5],
        m: MockMemory {
            jobs: vec![],
            visited: vec![1, 2, 3, 4],
        },
    };
    
    engine.clear();
    
    assert_eq!(engine.m.visited.len(), 5); // 5 = (3 * (2 + 1) + 63) / 64
    assert!(engine.m.visited.iter().all(|&v| v == 0));
}

