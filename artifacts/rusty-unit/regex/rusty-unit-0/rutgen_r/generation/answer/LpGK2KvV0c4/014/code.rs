// Answer 0

#[test]
fn test_step_with_match() {
    struct MockInput {
        input: Vec<char>,
    }

    impl MockInput {
        fn at(&self, pos: usize) -> char {
            self.input[pos]
        }

        fn is_empty_match(&self, _at: InputAt, _inst: &EmptyLookInst) -> bool {
            true
        }
    }

    struct MockProg {
        inst: Vec<Inst>,
    }

    enum Inst {
        Match(usize),
        Save(SaveInst),
        Split(SplitInst),
        EmptyLook(EmptyLookInst),
        Char(CharInst),
        Ranges(RangesInst),
        Bytes(BytesInst),
    }

    struct SaveInst {
        slot: usize,
        goto: InstPtr,
    }

    struct SplitInst {
        goto1: InstPtr,
        goto2: InstPtr,
    }

    struct EmptyLookInst {
        goto: InstPtr,
    }

    struct CharInst {
        c: char,
        goto: InstPtr,
    }

    struct RangesInst {
        ranges: std::ops::RangeInclusive<char>, // Example: 'a'..='z'
        goto: InstPtr,
    }

    struct BytesInst {
        matches: fn(u8) -> bool,
        goto: InstPtr,
    }

    struct InstPtr(usize);

    struct InputAt {
        char_index: usize,
    }

    impl InputAt {
        fn char(&self, input: &MockInput) -> char {
            input.at(self.char_index)
        }

        fn next_pos(&self) -> usize {
            self.char_index + 1
        }

        fn pos(&self) -> usize {
            self.char_index
        }

        fn byte(&self) -> Option<u8> {
            Some(self.char() as u8)
        }
    }

    struct Matcher {
        prog: MockProg,
        matches: Vec<bool>,
        slots: Vec<Option<usize>>,
        input: MockInput,
        has_visited: Vec<(InstPtr, InputAt)>,
    }

    impl Matcher {
        fn has_visited(&self, ip: InstPtr, at: InputAt) -> bool {
            self.has_visited.contains(&(ip, at))
        }

        fn step(&mut self, mut ip: InstPtr, mut at: InputAt) -> bool {
            use Inst::*;
            loop {
                if self.has_visited(ip, at) {
                    return false;
                }
                match self.prog.inst[ip.0] {
                    Match(slot) => {
                        if slot < self.matches.len() {
                            self.matches[slot] = true;
                        }
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
        }
    }

    let input = MockInput { input: "abc".chars().collect() };
    let prog = MockProg { inst: vec![Inst::Match(0)] };
    let matches = vec![false; 1]; // Enough slots to accommodate match
    let slots = vec![None];
    let mut matcher = Matcher {
        prog,
        matches,
        slots,
        input,
        has_visited: vec![],
    };
    let ip = InstPtr(0);
    let at = InputAt { char_index: 0 };

    let result = matcher.step(ip, at);
    assert!(result);
    assert!(matcher.matches[0]);
}

