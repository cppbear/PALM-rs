// Answer 0

#[test]
fn test_fill_char() {
    struct InstPtr;
    enum InstHole {
        Char { c: char },
    }
    enum Inst {
        Char(InstChar),
    }
    struct InstChar {
        goto: InstPtr,
        c: char,
    }
    
    let goto = InstPtr;
    let c = 'a';
    let inst_hole = InstHole::Char { c };

    match inst_hole {
        InstHole::Char { c } => {
            let result = Inst::Char(InstChar { goto, c });
            if let Inst::Char(inst_char) = result {
                assert_eq!(inst_char.goto as *const _ as usize, goto as *const _ as usize);
                assert_eq!(inst_char.c, c);
            } else {
                panic!("Expected Inst::Char variant.");
            }
        }
        _ => panic!("Unexpected InstHole variant."),
    }
}

