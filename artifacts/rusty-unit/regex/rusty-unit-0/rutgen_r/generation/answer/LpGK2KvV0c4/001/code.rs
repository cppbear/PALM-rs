// Answer 0

#[test]
fn test_step_has_visited_true() {
    struct FakeProg {
        inst: Vec<prog::Inst>,
    }

    impl FakeProg {
        fn new(inst: Vec<prog::Inst>) -> Self {
            Self { inst }
        }
    }

    struct InputAt {
        pos: usize,
        chars: Vec<char>,
    }

    impl InputAt {
        fn new(chars: Vec<char>, pos: usize) -> Self {
            Self { chars, pos }
        }

        fn char(&self) -> char {
            self.chars[self.pos]
        }

        fn next_pos(&self) -> usize {
            self.pos + 1
        }
    }

    struct Matcher {
        prog: FakeProg,
        matches: Vec<bool>,
        slots: Vec<Option<usize>>,
        input: InputAt,
        visited: Vec<(usize, usize)>,
    }

    impl Matcher {
        fn has_visited(&self, ip: usize, at: InputAt) -> bool {
            self.visited.contains(&(ip, at.pos))
        }

        fn step(&mut self, mut ip: usize, mut at: InputAt) -> bool {
            use prog::Inst::*;
            loop {
                if self.has_visited(ip, at) {
                    return false;
                }
                match self.prog.inst[ip] {
                    Match(slot) => {
                        if slot < self.matches.len() {
                            self.matches[slot] = true;
                        }
                        return true;
                    }
                    // Other match cases omitted for brevity
                    _ => return false,
                }
            }
        }
    }

    let fake_prog = FakeProg::new(vec![
        prog::Inst::Save(prog::SaveInst { slot: 0, goto: 1 }),
        prog::Inst::Match(0),
    ]);

    let input_at = InputAt::new(vec!['a', 'b', 'c'], 0);
    let mut matcher = Matcher {
        prog: fake_prog,
        matches: vec![false],
        slots: vec![None],
        input: input_at,
        visited: vec![(0, 0)], // Ensure has_visited is true
    };

    let result = matcher.step(0, matcher.input);
    assert_eq!(result, false);
}

