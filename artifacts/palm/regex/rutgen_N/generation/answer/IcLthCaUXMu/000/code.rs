// Answer 0

#[derive(Debug)]
struct Literal {
    len: usize,
    cut: bool,
}

impl Literal {
    fn is_cut(&self) -> bool {
        self.cut
    }
}

#[derive(Debug)]
struct Class {
    limit_class: usize,
    limit_size: usize,
    lits: Vec<Literal>,
}

impl Class {
    fn class_exceeds_limits(&self, size: usize) -> bool {
        if size > self.limit_class {
            return true;
        }
        let new_byte_count = if self.lits.is_empty() {
            size
        } else {
            self.lits.iter().fold(0, |accum, lit| {
                accum + if lit.is_cut() {
                    0
                } else {
                    (lit.len + 1) * size
                }
            })
        };
        new_byte_count > self.limit_size
    }
}

#[test]
fn test_class_exceeds_limits_empty_literals() {
    let class = Class {
        limit_class: 5,
        limit_size: 10,
        lits: vec![],
    };
    assert_eq!(class.class_exceeds_limits(6), true);
    assert_eq!(class.class_exceeds_limits(4), false);
}

#[test]
fn test_class_exceeds_limits_with_literals() {
    let class = Class {
        limit_class: 5,
        limit_size: 20,
        lits: vec![Literal { len: 3, cut: false }, Literal { len: 2, cut: false }],
    };
    assert_eq!(class.class_exceeds_limits(6), false);
    assert_eq!(class.class_exceeds_limits(5), true); // (3 + 1) * 5 + (2 + 1) * 5 = 20 exceeds limit_size
}

#[test]
fn test_class_exceeds_limits_with_cut_literals() {
    let class = Class {
        limit_class: 5,
        limit_size: 20,
        lits: vec![Literal { len: 3, cut: true }, Literal { len: 2, cut: false }],
    };
    assert_eq!(class.class_exceeds_limits(6), false); // Only counts the second literal
}

