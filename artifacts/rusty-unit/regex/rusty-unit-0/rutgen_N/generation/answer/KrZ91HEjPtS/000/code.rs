// Answer 0

#[derive(Debug, Clone)]
enum InstHole {
    Save { slot: usize },
    EmptyLook { look: char },
    Char { c: char },
    Ranges { ranges: Vec<char> },
    Bytes { start: u8, end: u8 },
}

#[derive(Debug, Clone)]
struct InstSave {
    goto: InstPtr,
    slot: usize,
}

#[derive(Debug, Clone)]
struct InstEmptyLook {
    goto: InstPtr,
    look: char,
}

#[derive(Debug, Clone)]
struct InstChar {
    goto: InstPtr,
    c: char,
}

#[derive(Debug, Clone)]
struct InstRanges {
    goto: InstPtr,
    ranges: Vec<char>,
}

#[derive(Debug, Clone)]
struct InstBytes {
    goto: InstPtr,
    start: u8,
    end: u8,
}

#[derive(Debug, Clone)]
enum Inst {
    Save(InstSave),
    EmptyLook(InstEmptyLook),
    Char(InstChar),
    Ranges(InstRanges),
    Bytes(InstBytes),
}

#[derive(Debug, Clone)]
struct InstPtr;

impl InstHole {
    fn fill(&self, goto: InstPtr) -> Inst {
        match *self {
            InstHole::Save { slot } => Inst::Save(InstSave {
                goto: goto,
                slot: slot,
            }),
            InstHole::EmptyLook { look } => Inst::EmptyLook(InstEmptyLook {
                goto: goto,
                look: look,
            }),
            InstHole::Char { c } => Inst::Char(InstChar {
                goto: goto,
                c: c,
            }),
            InstHole::Ranges { ref ranges } => Inst::Ranges(InstRanges {
                goto: goto,
                ranges: ranges.clone(),
            }),
            InstHole::Bytes { start, end } => Inst::Bytes(InstBytes {
                goto: goto,
                start: start,
                end: end,
            }),
        }
    }
}

#[test]
fn test_fill_save() {
    let inst_hole = InstHole::Save { slot: 1 };
    let goto = InstPtr;
    let result = inst_hole.fill(goto);
    if let Inst::Save(InstSave { goto: _, slot }) = result {
        assert_eq!(slot, 1);
    } else {
        panic!("Expected Inst::Save");
    }
}

#[test]
fn test_fill_empty_look() {
    let inst_hole = InstHole::EmptyLook { look: 'a' };
    let goto = InstPtr;
    let result = inst_hole.fill(goto);
    if let Inst::EmptyLook(InstEmptyLook { goto: _, look }) = result {
        assert_eq!(look, 'a');
    } else {
        panic!("Expected Inst::EmptyLook");
    }
}

#[test]
fn test_fill_char() {
    let inst_hole = InstHole::Char { c: 'b' };
    let goto = InstPtr;
    let result = inst_hole.fill(goto);
    if let Inst::Char(InstChar { goto: _, c }) = result {
        assert_eq!(c, 'b');
    } else {
        panic!("Expected Inst::Char");
    }
}

#[test]
fn test_fill_ranges() {
    let inst_hole = InstHole::Ranges { ranges: vec!['c', 'd'] };
    let goto = InstPtr;
    let result = inst_hole.fill(goto);
    if let Inst::Ranges(InstRanges { goto: _, ranges }) = result {
        assert_eq!(ranges, vec!['c', 'd']);
    } else {
        panic!("Expected Inst::Ranges");
    }
}

#[test]
fn test_fill_bytes() {
    let inst_hole = InstHole::Bytes { start: 1, end: 10 };
    let goto = InstPtr;
    let result = inst_hole.fill(goto);
    if let Inst::Bytes(InstBytes { goto: _, start, end }) = result {
        assert_eq!(start, 1);
        assert_eq!(end, 10);
    } else {
        panic!("Expected Inst::Bytes");
    }
}

