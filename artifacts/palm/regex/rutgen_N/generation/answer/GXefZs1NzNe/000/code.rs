// Answer 0

#[derive(Debug)]
struct MaybeInst {
    // Fields can be defined based on actual implementation
}

struct Regex {
    insts: Vec<MaybeInst>,
}

impl Regex {
    fn new() -> Self {
        Regex { insts: Vec::new() }
    }

    fn push_split_hole(&mut self) -> Hole {
        let hole = self.insts.len();
        self.insts.push(MaybeInst {});
        Hole::One(hole)
    }
}

#[derive(Debug, PartialEq)]
enum Hole {
    One(usize),
}

#[test]
fn test_push_split_hole_initial() {
    let mut regex = Regex::new();
    let hole = regex.push_split_hole();
    assert_eq!(hole, Hole::One(0));
    assert_eq!(regex.insts.len(), 1);
}

#[test]
fn test_push_split_hole_after_multiple_calls() {
    let mut regex = Regex::new();
    let hole1 = regex.push_split_hole();
    let hole2 = regex.push_split_hole();
    assert_eq!(hole1, Hole::One(0));
    assert_eq!(hole2, Hole::One(1));
    assert_eq!(regex.insts.len(), 2);
}

