// Answer 0

#[derive(Debug)]
enum MaybeInst {
    Split,
    Compiled(Inst),
}

#[derive(Debug)]
enum Inst {
    Split(InstSplit),
}

#[derive(Debug)]
struct InstSplit {
    goto1: InstPtr,
    goto2: InstPtr,
}

#[derive(Debug)]
struct InstPtr;

#[test]
#[should_panic(expected = "must be called on Split instruction")]
fn test_fill_split_not_split() {
    let mut maybe_inst = MaybeInst::Compiled(Inst::Split(InstSplit { goto1: InstPtr, goto2: InstPtr }));

    // The parameters here are irrelevant since we are testing the panic condition
    let goto1 = InstPtr;
    let goto2 = InstPtr;

    fill_split(&mut maybe_inst, goto1, goto2);
}

