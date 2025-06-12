// Answer 0

#[derive(Debug)]
enum MaybeInst {
    Compiled(Inst),
    NotCompiled,
}

#[derive(Debug)]
struct Inst;

impl MaybeInst {
    fn unwrap(self) -> Inst {
        match self {
            MaybeInst::Compiled(inst) => inst,
            _ => unreachable!("must be called on a compiled instruction, instead it was called on: {:?}", self),
        }
    }
}

#[test]
fn test_unwrap_with_compiled() {
    let inst = Inst;
    let maybe_inst = MaybeInst::Compiled(inst);
    let unwrapped = maybe_inst.unwrap();
    // Assuming we have a way to assert the type, here we can just verify it compiles.
    let _ = unwrapped; // Just using it to ensure there are no type errors
}

#[test]
#[should_panic(expected = "must be called on a compiled instruction")]
fn test_unwrap_with_not_compiled() {
    let maybe_inst = MaybeInst::NotCompiled;
    maybe_inst.unwrap(); // This should panic
}

