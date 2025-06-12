// Answer 0

#[derive(Debug)]
enum MaybeInst {
    Compiled(Inst),
    // other variants can be added as needed
}

#[derive(Debug)]
struct Inst {
    value: i32, // example field
}

impl MaybeInst {
    fn unwrap(self) -> Inst {
        match self {
            MaybeInst::Compiled(inst) => inst,
            _ => unreachable!("must be called on a compiled instruction, instead it was called on: {:?}", self),
        }
    }
}

#[test]
fn test_unwrap_with_compiled_instance() {
    let compiled_inst = Inst { value: 42 };
    let maybe_inst = MaybeInst::Compiled(compiled_inst);
    let result = maybe_inst.unwrap();
    assert_eq!(result.value, 42);
}

#[test]
#[should_panic(expected = "must be called on a compiled instruction")]
fn test_unwrap_with_non_compiled_instance() {
    let maybe_inst = MaybeInst::Compiled(Inst { value: 0 });
    let _result = match maybe_inst {
        MaybeInst::Compiled(_) => unreachable!(),
    };
}

