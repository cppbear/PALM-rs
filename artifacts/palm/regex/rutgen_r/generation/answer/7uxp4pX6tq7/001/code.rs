// Answer 0

#[derive(Debug)]
enum InstPtr {}

#[derive(Debug)]
struct InstSplit {
    goto1: InstPtr,
    goto2: InstPtr,
}

#[derive(Debug)]
enum Inst {
    Split(InstSplit),
}

#[derive(Debug)]
enum MaybeInst {
    Compiled(Inst),
    Uncompiled(InstPtr),
    Split1(InstPtr),
    Split2(InstPtr),
}

impl MaybeInst {
    fn fill(&mut self, goto: InstPtr) {
        let filled = match *self {
            MaybeInst::Uncompiled(ref inst) => inst.fill(goto),
            MaybeInst::Split1(goto1) => {
                Inst::Split(InstSplit { goto1: goto1, goto2: goto })
            }
            MaybeInst::Split2(goto2) => {
                Inst::Split(InstSplit { goto1: goto, goto2: goto2 })
            }
            _ => unreachable!("not all instructions were compiled! \
                               found uncompiled instruction: {:?}", self),
        };
        *self = MaybeInst::Compiled(filled);
    }
}

#[test]
#[should_panic(expected = "not all instructions were compiled!")]
fn test_fill_uncompiled() {
    let mut inst = MaybeInst::Uncompiled(InstPtr {});
    inst.fill(InstPtr {});
}

#[test]
#[should_panic(expected = "not all instructions were compiled!")]
fn test_fill_split1() {
    let mut inst = MaybeInst::Split1(InstPtr {});
    inst.fill(InstPtr {});
}

#[test]
#[should_panic(expected = "not all instructions were compiled!")]
fn test_fill_split2() {
    let mut inst = MaybeInst::Split2(InstPtr {});
    inst.fill(InstPtr {});
}

#[test]
fn test_fill_compiled() {
    let mut inst = MaybeInst::Compiled(Inst::Split(InstSplit { goto1: InstPtr {}, goto2: InstPtr {} }));
    // In this case the function will not panic, it will set inst to Compiled again but with the new goes.
    inst.fill(InstPtr {});
    assert!(matches!(inst, MaybeInst::Compiled(_)));
}

