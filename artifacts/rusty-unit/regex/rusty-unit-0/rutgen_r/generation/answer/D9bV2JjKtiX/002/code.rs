// Answer 0

#[derive(Debug)]
struct InputAt;

#[derive(Debug)]
enum Job {
    Inst { ip: usize, at: InputAt },
    SaveRestore { slot: usize, old_pos: usize },
}

struct Machine {
    jobs: Vec<Job>,
}

struct Regex {
    m: Machine,
    slots: Vec<usize>,
    prog: Program,
}

struct Program {
    matches: Vec<usize>,
}

impl Regex {
    fn backtrack(&mut self, start: InputAt) -> bool {
        let mut matched = false;
        self.m.jobs.push(Job::Inst { ip: 0, at: start });
        while let Some(job) = self.m.jobs.pop() {
            match job {
                Job::Inst { ip, at } => {
                    if self.step(ip, at) {
                        if self.prog.matches.len() == 1 {
                            return true;
                        }
                        matched = true;
                    }
                }
                Job::SaveRestore { slot, old_pos } => {
                    if slot < self.slots.len() {
                        self.slots[slot] = old_pos;
                    }
                }
            }
        }
        matched
    }

    fn step(&self, _ip: usize, _at: InputAt) -> bool {
        // Dummy step implementation for the test to run.
        true
    }
}

#[test]
fn test_backtrack_success_with_inst_job() {
    let mut regex = Regex {
        m: Machine { jobs: vec![] },
        slots: vec![0, 1, 2],
        prog: Program { matches: vec![0] },
    };

    let input = InputAt;
    let result = regex.backtrack(input);
    assert_eq!(result, true);
}

#[test]
fn test_backtrack_success_with_save_restore_job() {
    let mut regex = Regex {
        m: Machine {
            jobs: vec![Job::SaveRestore { slot: 1, old_pos: 2 }],
        },
        slots: vec![0, 1, 2],
        prog: Program { matches: vec![0] },
    };

    let input = InputAt;
    let result = regex.backtrack(input);
    assert_eq!(result, true);
}

#[test]
fn test_backtrack_no_matches() {
    let mut regex = Regex {
        m: Machine {
            jobs: vec![Job::Inst { ip: 0, at: InputAt }],
        },
        slots: vec![0],
        prog: Program { matches: vec![] },
    };

    let input = InputAt;
    let result = regex.backtrack(input);
    assert_eq!(result, false);
}

