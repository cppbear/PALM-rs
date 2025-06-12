// Answer 0

#[test]
fn test_fill_inst_hole_ranges() {
    #[derive(Clone)]
    struct InstPtr;

    #[derive(Clone)]
    struct InstRanges {
        goto: InstPtr,
        ranges: Vec<(char, char)>,
    }

    enum Inst {
        Ranges(InstRanges),
    }

    enum InstHole {
        Ranges { ranges: Vec<(char, char)> },
    }

    impl InstHole {
        fn fill(&self, goto: InstPtr) -> Inst {
            match *self {
                InstHole::Ranges { ref ranges } => Inst::Ranges(InstRanges {
                    goto: goto,
                    ranges: ranges.clone(),
                }),
            }
        }
    }

    let goto = InstPtr;
    let ranges = vec![('a', 'z'), ('0', '9')];
    let inst_hole_ranges = InstHole::Ranges { ranges };

    if let Inst::Ranges(inst_ranges) = inst_hole_ranges.fill(goto.clone()) {
        assert_eq!(inst_ranges.goto, goto);
        assert_eq!(inst_ranges.ranges.len(), 2);
        assert_eq!(inst_ranges.ranges[0], ('a', 'z'));
        assert_eq!(inst_ranges.ranges[1], ('0', '9'));
    } else {
        panic!("Expected Inst::Ranges variant");
    }
}

