// Answer 0

#[derive(Debug)]
struct Hole {
    size: usize,
}

struct Insts {
    insts: Vec<Hole>,
}

impl Insts {
    fn new() -> Self {
        Insts { insts: Vec::new() }
    }

    fn fill(&mut self, hole: Hole, index: usize) {
        if index > self.insts.len() {
            panic!("Index out of bounds");
        }
        self.insts.insert(index, hole);
    }

    fn fill_to_next(&mut self, hole: Hole) {
        let next = self.insts.len();
        self.fill(hole, next);
    }
}

#[test]
fn test_fill_to_next_empty() {
    let mut insts = Insts::new();
    let hole = Hole { size: 1 };
    insts.fill_to_next(hole);
    assert_eq!(insts.insts.len(), 1);
    assert_eq!(insts.insts[0].size, 1);
}

#[test]
fn test_fill_to_next_multiple() {
    let mut insts = Insts::new();
    let hole1 = Hole { size: 1 };
    let hole2 = Hole { size: 2 };
    insts.fill_to_next(hole1);
    insts.fill_to_next(hole2);
    assert_eq!(insts.insts.len(), 2);
    assert_eq!(insts.insts[1].size, 2);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_fill_to_next_index_out_of_bounds() {
    let mut insts = Insts::new();
    let hole = Hole { size: 1 };
    insts.fill(hole, 1); // This should panic
}

#[test]
fn test_fill_to_next_boundary() {
    let mut insts = Insts::new();
    let hole = Hole { size: 3 };
    insts.fill_to_next(hole);
    assert_eq!(insts.insts.len(), 1);
    assert_eq!(insts.insts[0].size, 3);
    insts.fill_to_next(Hole { size: 4 });
    assert_eq!(insts.insts.len(), 2);
    assert_eq!(insts.insts[1].size, 4);
}

