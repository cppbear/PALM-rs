// Answer 0

#[test]
fn test_step_returns_false_on_visited() {
    struct MockInputAt {
        pos: usize,
        character: char,
    }

    impl MockInputAt {
        fn new(pos: usize, character: char) -> Self {
            MockInputAt { pos, character }
        }

        fn char(&self) -> char {
            self.character
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn next_pos(&self) -> usize {
            self.pos + 1
        }

        fn byte(&self) -> Option<u8> {
            Some(self.character as u8)
        }
    }
    
    struct MockInst {
        c: char,
    }

    impl MockInst {
        fn matches(&self, ch: char) -> bool {
            self.c == ch
        }
    }

    struct MockProg {
        elements: Vec<Inst>,
    }

    enum Inst {
        Ranges(MockInst),
    }

    struct MockContext {
        prog: MockProg,
        visited: std::collections::HashSet<(usize, usize)>,
    }

    impl MockContext {
        fn has_visited(&self, ip: usize, at: MockInputAt) -> bool {
            self.visited.contains(&(ip, at.pos()))
        }

        fn step(&mut self, mut ip: usize, mut at: MockInputAt) -> bool {
            use Inst::*;
            loop {
                if self.has_visited(ip, at) {
                    return false;
                }
                match &self.prog.elements[ip] {
                    Ranges(inst) => {
                        if inst.matches(at.char()) {
                            self.visited.insert((ip, at.pos()));
                            return true;
                        }
                    }
                }
            }
        }
    }

    let mut context = MockContext {
        prog: MockProg { elements: vec![Inst::Ranges(MockInst { c: 'a' })] },
        visited: std::collections::HashSet::new(),
    };

    let ip = 0;
    let at = MockInputAt::new(0, 'a');

    context.step(ip, at);
    let result = context.step(ip, MockInputAt::new(0, 'a')); 
    assert_eq!(result, false);  
}

