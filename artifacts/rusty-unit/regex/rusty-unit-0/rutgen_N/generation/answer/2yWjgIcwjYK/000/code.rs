// Answer 0

#[test]
fn test_fmt_match() {
    struct MatchInst {
        slot: usize,
    }
    
    struct Instance {
        start: usize,
        instructions: Vec<Inst>,
    }
    
    enum Inst {
        Match(MatchInst),
        Save(SaveInst),
        Split(SplitInst),
        EmptyLook(EmptyLookInst),
        Char(CharInst),
        Ranges(RangesInst),
        Bytes(BytesInst),
    }

    struct SaveInst {
        slot: usize,
        goto: usize,
    }

    struct SplitInst {
        goto1: usize,
        goto2: usize,
    }

    struct EmptyLookInst {
        look: usize,
        goto: usize,
    }

    struct CharInst {
        c: char,
        goto: usize,
    }

    struct RangesInst {
        ranges: Vec<(char, char)>,
        goto: usize,
    }

    struct BytesInst {
        start: u8,
        end: u8,
        goto: usize,
    }

    let inst = Instance {
        start: 0,
        instructions: vec![
            Inst::Match(MatchInst { slot: 1 }),
            Inst::Save(SaveInst { slot: 2, goto: 1 }),
            Inst::Split(SplitInst { goto1: 3, goto2: 4 }),
        ],
    };

    let mut output = String::new();
    let result = inst.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "0000 Match(1) (start)\n0001 Save(2) (goto: 1)\n0002 Split(3, 4)\n");
}

#[test]
fn test_fmt_empty_look() {
    struct EmptyLookInst {
        look: usize,
        goto: usize,
    }

    struct Instance {
        start: usize,
        instructions: Vec<Inst>,
    }

    enum Inst {
        EmptyLook(EmptyLookInst),
    }

    let inst = Instance {
        start: 0,
        instructions: vec![
            Inst::EmptyLook(EmptyLookInst { look: 0, goto: 1 }),
        ],
    };

    let mut output = String::new();
    let result = inst.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "0000 (goto: 1)\n");
}

#[test]
fn test_fmt_char() {
    struct CharInst {
        c: char,
        goto: usize,
    }

    struct Instance {
        start: usize,
        instructions: Vec<Inst>,
    }

    enum Inst {
        Char(CharInst),
    }

    let inst = Instance {
        start: 0,
        instructions: vec![
            Inst::Char(CharInst { c: 'a', goto: 1 }),
        ],
    };

    let mut output = String::new();
    let result = inst.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "0000 (goto: 1)\n");
}

#[test]
fn test_fmt_ranges() {
    struct RangesInst {
        ranges: Vec<(char, char)>,
        goto: usize,
    }

    struct Instance {
        start: usize,
        instructions: Vec<Inst>,
    }

    enum Inst {
        Ranges(RangesInst),
    }

    let inst = Instance {
        start: 0,
        instructions: vec![
            Inst::Ranges(RangesInst {
                ranges: vec![('a', 'z'), ('0', '9')],
                goto: 1,
            }),
        ],
    };

    let mut output = String::new();
    let result = inst.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "0000 (goto: 1)\n");
}

