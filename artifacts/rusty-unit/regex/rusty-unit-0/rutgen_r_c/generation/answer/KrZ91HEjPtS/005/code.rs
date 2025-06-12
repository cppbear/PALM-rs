// Answer 0

#[test]
fn test_fill_inst_hole_save() {
    struct MockInstHole {
        slot: usize,
    }

    impl MockInstHole {
        fn fill(&self, goto: usize) -> Inst {
            InstHole::Save { slot: self.slot }.fill(goto)
        }
    }

    let inst_hole = MockInstHole { slot: 5 };
    let goto: InstPtr = 10;

    match inst_hole.fill(goto) {
        Inst::Save(inst_save) => {
            assert_eq!(inst_save.goto, goto);
            assert_eq!(inst_save.slot, 5);
        },
        _ => panic!("Expected Inst::Save, but got something else"),
    }
}

#[test]
fn test_fill_inst_hole_empty_look() {
    struct MockInstHole {
        look: EmptyLook,
    }

    impl MockInstHole {
        fn fill(&self, goto: usize) -> Inst {
        InstHole::EmptyLook { look: self.look }.fill(goto)
        }
    }

    let inst_hole = MockInstHole { look: EmptyLook::StartLine };
    let goto: InstPtr = 20;

    match inst_hole.fill(goto) {
        Inst::EmptyLook(inst_empty_look) => {
            assert_eq!(inst_empty_look.goto, goto);
            assert_eq!(inst_empty_look.look, EmptyLook::StartLine);
        },
        _ => panic!("Expected Inst::EmptyLook, but got something else"),
    }
}

#[test]
fn test_fill_inst_hole_char() {
    struct MockInstHole {
        c: char,
    }

    impl MockInstHole {
        fn fill(&self, goto: usize) -> Inst {
            InstHole::Char { c: self.c }.fill(goto)
        }
    }

    let inst_hole = MockInstHole { c: 'a' };
    let goto: InstPtr = 30;

    match inst_hole.fill(goto) {
        Inst::Char(inst_char) => {
            assert_eq!(inst_char.goto, goto);
            assert_eq!(inst_char.c, 'a');
        },
        _ => panic!("Expected Inst::Char, but got something else"),
    }
}

#[test]
fn test_fill_inst_hole_ranges() {
    struct MockInstHole {
        ranges: Vec<(char, char)>,
    }

    impl MockInstHole {
        fn fill(&self, goto: usize) -> Inst {
            InstHole::Ranges { ranges: self.ranges.clone() }.fill(goto)
        }
    }

    let inst_hole = MockInstHole { ranges: vec![('a', 'z'), ('A', 'Z')] };
    let goto: InstPtr = 40;

    match inst_hole.fill(goto) {
        Inst::Ranges(inst_ranges) => {
            assert_eq!(inst_ranges.goto, goto);
            assert_eq!(inst_ranges.ranges, vec![('a', 'z'), ('A', 'Z')]);
        },
        _ => panic!("Expected Inst::Ranges, but got something else"),
    }
}

#[test]
fn test_fill_inst_hole_bytes() {
    struct MockInstHole {
        start: u8,
        end: u8,
    }

    impl MockInstHole {
        fn fill(&self, goto: usize) -> Inst {
            InstHole::Bytes { start: self.start, end: self.end }.fill(goto)
        }
    }

    let inst_hole = MockInstHole { start: 100, end: 200 };
    let goto: InstPtr = 50;

    match inst_hole.fill(goto) {
        Inst::Bytes(inst_bytes) => {
            assert_eq!(inst_bytes.goto, goto);
            assert_eq!(inst_bytes.start, 100);
            assert_eq!(inst_bytes.end, 200);
        },
        _ => panic!("Expected Inst::Bytes, but got something else"),
    }
}

