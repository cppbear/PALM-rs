// Answer 0

#[test]
#[should_panic(expected = "must be called on Split instruction")]
fn test_half_fill_split_goto1_not_split() {
    struct MaybeInst {
        value: u32,
    }

    impl MaybeInst {
        fn new(value: u32) -> Self {
            MaybeInst { value }
        }
    }

    let mut instance = MaybeInst::new(1); // Not a Split variant
    let goto1 = 2; // Some InstPtr equivalent

    instance.half_fill_split_goto1(goto1);
}

#[test]
fn test_half_fill_split_goto1_valid_case() {
    enum MaybeInst {
        Split,
        Split1(usize),
    }

    impl MaybeInst {
        fn half_fill_split_goto1(&mut self, goto1: usize) {
            let half_filled = match *self {
                MaybeInst::Split => goto1,
                _ => unreachable!("must be called on Split instruction, instead it was called on: {:?}", self),
            };
            *self = MaybeInst::Split1(half_filled);
        }
    }

    let mut instance = MaybeInst::Split;
    let goto1 = 42;

    instance.half_fill_split_goto1(goto1);

    if let MaybeInst::Split1(value) = instance {
        assert_eq!(value, goto1);
    } else {
        panic!("Expected MaybeInst::Split1, but got a different variant.");
    }
}

