// Answer 0

#[derive(Debug)]
struct Interval {
    lower: i32,
    upper: i32,
}

impl Interval {
    fn intersect(&self, other: &Interval) -> Option<Interval> {
        if self.upper < other.lower || other.upper < self.lower {
            None
        } else {
            Some(Interval {
                lower: self.lower.max(other.lower),
                upper: self.upper.min(other.upper),
            })
        }
    }

    fn upper(&self) -> i32 {
        self.upper
    }
}

#[derive(Debug)]
struct IntervalSet<I> {
    ranges: Vec<I>,
}

impl IntervalSet<Interval> {
    fn new(ranges: Vec<Interval>) -> Self {
        Self { ranges }
    }
    
    pub fn intersect(&mut self, other: &IntervalSet<Interval>) {
        if self.ranges.is_empty() {
            return;
        }
        if other.ranges.is_empty() {
            self.ranges.clear();
            return;
        }
        
        let drain_end = self.ranges.len();
        let mut ita = (0..drain_end).into_iter();
        let mut itb = (0..other.ranges.len()).into_iter();
        let mut a = ita.next().unwrap();
        let mut b = itb.next().unwrap();
        loop {
            if let Some(ab) = self.ranges[a].intersect(&other.ranges[b]) {
                self.ranges.push(ab);
            }
            let (it, aorb) =
                if self.ranges[a].upper() < other.ranges[b].upper() {
                    (&mut ita, &mut a)
                } else {
                    (&mut itb, &mut b)
                };
            match it.next() {
                Some(v) => *aorb = v,
                None => break,
            }
        }
        self.ranges.drain(..drain_end);
    }
}

#[test]
fn test_intersect_non_empty_sets() {
    let mut set_a = IntervalSet::new(vec![
        Interval { lower: 1, upper: 5 },
        Interval { lower: 10, upper: 15 },
    ]);

    let set_b = IntervalSet::new(vec![
        Interval { lower: 3, upper: 7 },
        Interval { lower: 12, upper: 14 },
    ]);

    set_a.intersect(&set_b);

    assert_eq!(set_a.ranges.len(), 2);
    assert_eq!(set_a.ranges[0], Interval { lower: 3, upper: 5 });
    assert_eq!(set_a.ranges[1], Interval { lower: 12, upper: 14 });
}

#[test]
fn test_intersect_with_no_overlap() {
    let mut set_a = IntervalSet::new(vec![
        Interval { lower: 1, upper: 2 },
        Interval { lower: 3, upper: 4 },
    ]);

    let set_b = IntervalSet::new(vec![
        Interval { lower: 5, upper: 6 },
        Interval { lower: 7, upper: 8 },
    ]);

    set_a.intersect(&set_b);

    assert_eq!(set_a.ranges.len(), 0);
}

#[test]
fn test_intersect_empty_self() {
    let mut set_a = IntervalSet::new(vec![]);
    let set_b = IntervalSet::new(vec![
        Interval { lower: 3, upper: 7 },
    ]);
    
    set_a.intersect(&set_b);
    
    assert_eq!(set_a.ranges.len(), 0);
}

#[test]
fn test_intersect_empty_other() {
    let mut set_a = IntervalSet::new(vec![
        Interval { lower: 1, upper: 2 },
    ]);
    let set_b = IntervalSet::new(vec![]);

    set_a.intersect(&set_b);

    assert_eq!(set_a.ranges.len(), 0);
}

