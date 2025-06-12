// Answer 0

#[test]
fn test_exec_with_constraints() {
    struct InputAt {
        position: usize,
        input_length: usize,
    }

    impl InputAt {
        fn is_start(&self) -> bool {
            self.position == 0
        }

        fn is_end(&self) -> bool {
            self.position >= self.input_length
        }

        fn next_pos(&self) -> usize {
            self.position + 1
        }
    }

    struct Prog {
        is_anchored_start: bool,
        prefixes: Vec<String>,
        matches: Vec<String>,
    }

    struct Regex {
        prog: Prog,
        input: InputData,
    }

    struct InputData {
        input_length: usize,
    }

    impl Regex {
        fn clear(&mut self) {
            // Clear any state
        }

        fn backtrack(&self, _at: InputAt) -> bool {
            // Simulating backtrack success
            true
        }

        fn exec_(&mut self, mut at: InputAt) -> bool {
            self.clear();
            if self.prog.is_anchored_start {
                return if !at.is_start() {
                    false
                } else {
                    self.backtrack(at)
                };
            }
            let mut matched = false;
            loop {
                if !self.prog.prefixes.is_empty() {
                    at = match self.input.prefix_at(&self.prog.prefixes, at) {
                        None => break,
                        Some(at) => at,
                    };
                }
                matched = self.backtrack(at) || matched;
                if matched && self.prog.matches.len() == 1 {
                    return true;
                }
                if at.is_end() {
                    break;
                }
                at = self.input.at(at.next_pos());
            }
            matched
        }
    }

    impl InputData {
        fn prefix_at(&self, _prefixes: &[String], _at: InputAt) -> Option<InputAt> {
            None
        }

        fn at(&self, position: usize) -> InputAt {
            InputAt {
                position,
                input_length: self.input_length,
            }
        }
    }

    let input_data = InputData { input_length: 5 };
    let program = Prog {
        is_anchored_start: false,
        prefixes: Vec::new(),
        matches: vec![String::from("match1")],
    };
    let mut regex = Regex { prog: program, input: input_data };

    let start_at = InputAt { position: 5, input_length: 5 };
    let result = regex.exec_(start_at);

    assert_eq!(result, true);
}

