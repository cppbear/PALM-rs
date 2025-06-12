// Answer 0

#[derive(Debug)]
struct InputAt {
    position: usize,
}

impl InputAt {
    fn pos(&self) -> usize {
        self.position
    }
}

#[derive(Debug)]
struct InstEmptyLook {
    look: EmptyLook,
}

#[derive(Debug)]
enum EmptyLook {
    WordBoundary,
}

#[derive(Debug)]
struct TestInput {
    data: Vec<char>,
}

impl TestInput {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn previous_char(&self, at: InputAt) -> char {
        if at.pos() > 0 {
            self.data[at.pos() - 1]
        } else {
            '\0'
        }
    }

    fn next_char(&self, at: InputAt) -> char {
        if at.pos() < self.len() {
            self.data[at.pos()]
        } else {
            '\0'
        }
    }

    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
        use EmptyLook::*;
        match empty.look {
            WordBoundary => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                c1.is_alphabetic() != c2.is_alphabetic()
            }
            _ => false,
        }
    }
}

#[test]
fn test_word_boundary() {
    let input = TestInput {
        data: "hello world".chars().collect(),
    };

    let empty = InstEmptyLook {
        look: EmptyLook::WordBoundary,
    };

    let at_start = InputAt { position: 0 }; // before 'h'
    assert_eq!(input.is_empty_match(at_start, &empty), true); // Boundary check (not a word char vs. word char)

    let at_middle = InputAt { position: 5 }; // after 'o'
    assert_eq!(input.is_empty_match(at_middle, &empty), true); // Boundary check (word char vs. space)

    let at_end = InputAt { position: 11 }; // at the end
    assert_eq!(input.is_empty_match(at_end, &empty), true); // Boundary check (word char vs terminal)

    let at_space = InputAt { position: 6 }; // at the space
    assert_eq!(input.is_empty_match(at_space, &empty), false); // Boundary check (space vs word char)
}

