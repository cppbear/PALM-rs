// Answer 0

#[test]
fn test_fill_with_split1() {
    let goto = 0;
    let mut self_instance = MaybeInst::Split1(1);
    self_instance.fill(goto);
}

#[test]
fn test_fill_with_split1_alternate() {
    let goto = 1;
    let mut self_instance = MaybeInst::Split1(2);
    self_instance.fill(goto);
}

#[test]
fn test_fill_with_split2() {
    let goto = 0;
    let mut self_instance = MaybeInst::Split2(1);
    self_instance.fill(goto);
}

#[test]
fn test_fill_with_split2_alternate() {
    let goto = 2;
    let mut self_instance = MaybeInst::Split2(1);
    self_instance.fill(goto);
}

