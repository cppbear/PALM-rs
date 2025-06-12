// Answer 0

#[derive(Debug)]
enum MaybeInst {
    Split,
    Split2(InstPtr),
}

type InstPtr = usize;

struct Instruction {
    variant: MaybeInst,
}

impl Instruction {
    fn half_fill_split_goto2(&mut self, goto2: InstPtr) {
        let half_filled = match self.variant {
            MaybeInst::Split => goto2,
            _ => unreachable!("must be called on Split instruction, instead it was called on: {:?}", self),
        };
        self.variant = MaybeInst::Split2(half_filled);
    }
}

#[test]
fn test_half_fill_split_goto2_success() {
    let mut instruction = Instruction {
        variant: MaybeInst::Split,
    };
    let goto2: InstPtr = 42;

    instruction.half_fill_split_goto2(goto2);

    match instruction.variant {
        MaybeInst::Split2(value) => assert_eq!(value, goto2),
        _ => panic!("Expected Split2 variant."),
    }
}

#[test]
#[should_panic(expected = "must be called on Split instruction, instead it was called on:")]
fn test_half_fill_split_goto2_unreachable() {
    let mut instruction = Instruction {
        variant: MaybeInst::Split2(10),
    };
    let goto2: InstPtr = 42;

    instruction.half_fill_split_goto2(goto2);
}

