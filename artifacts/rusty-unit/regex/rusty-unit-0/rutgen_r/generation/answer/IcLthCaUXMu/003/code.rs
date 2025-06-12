// Answer 0

#[derive(Debug)]
struct Literal {
    len: usize,
    is_cut: bool,
}

impl Literal {
    fn new(len: usize, is_cut: bool) -> Self {
        Self { len, is_cut }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_cut(&self) -> bool {
        self.is_cut
    }
}

struct RegexSet {
    limit_class: usize,
    limit_size: usize,
    lits: Vec<Literal>,
}

impl RegexSet {
    fn new(limit_class: usize, limit_size: usize, lits: Vec<Literal>) -> Self {
        Self {
            limit_class,
            limit_size,
            lits,
        }
    }

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
                    (lit.len() + 1) * size
                }
            })
        };
        new_byte_count > self.limit_size
    }
}

#[test]
fn test_class_exceeds_limits_bound_size() {
    let limit_class = 5;
    let limit_size = 20;
    let size = limit_class; // constraint: size == self.limit_class
    let lits = vec![
        Literal::new(2, false), // contributes (2 + 1) * size to byte count
        Literal::new(3, false), // contributes (3 + 1) * size to byte count
    ];
    
    let regex_set = RegexSet::new(limit_class, limit_size, lits);
    assert!(regex_set.class_exceeds_limits(size)); // expected: new_byte_count > self.limit_size
}

#[test]
fn test_class_exceeds_limits_multiple_literals() {
    let limit_class = 3;
    let limit_size = 50; 
    let size = limit_class; // constraint: size == self.limit_class
    let lits = vec![
        Literal::new(5, false), // contributes (5 + 1) * size to byte count
        Literal::new(2, false), // contributes (2 + 1) * size to byte count
        Literal::new(7, true),  // does not contribute as is_cut is true
    ];
    
    let regex_set = RegexSet::new(limit_class, limit_size, lits);
    assert!(regex_set.class_exceeds_limits(size)); // expected: new_byte_count > self.limit_size
}

