// Answer 0

#[derive(Debug)]
enum MaybeInst {
    Compiled(Inst),
    NotCompiled,
}

#[derive(Debug)]
struct Inst;

#[test]
#[should_panic(expected = "must be called on a compiled instruction")]
fn test_unwrap_not_compiled() {
    let maybe_inst = MaybeInst::NotCompiled;
    let _ = unwrap(maybe_inst);
}

#[test]
#[should_panic(expected = "must be called on a compiled instruction")]
fn test_unwrap_another_variant() {
    #[derive(Debug)]
    enum AnotherVariant {
        SomeOtherVariant,
    }

    let maybe_inst = AnotherVariant::SomeOtherVariant;
    let _ = unwrap(maybe_inst);
}

fn unwrap(self: MaybeInst) -> Inst {
    match self {
        MaybeInst::Compiled(inst) => inst,
        _ => unreachable!("must be called on a compiled instruction, \
                           instead it was called on: {:?}", self),
    }
}

