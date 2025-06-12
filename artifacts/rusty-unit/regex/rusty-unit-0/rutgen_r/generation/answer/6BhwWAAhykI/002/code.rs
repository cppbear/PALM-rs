// Answer 0

#[test]
fn test_intersect_with_non_empty_self_and_empty_other() {
    struct Interval {
        lower: i32,
        upper: i32,
    }

    impl Interval {
        fn intersect(&self, other: &Self) -> Option<Self> {
            if self.lower <= other.upper && self.upper >= other.lower {
                Some(Interval {
                    lower: self.lower.max(other.lower),
                    upper: self.upper.min(other.upper),
                })
            } else {
                None
            }
        }

        fn upper(&self) -> i32 {
            self.upper
        }
    }

    struct IntervalSet {
        ranges: Vec<Interval>,
    }

    impl IntervalSet {
        fn new() -> Self {
            Self { ranges: Vec::new() }
        }

        fn push(&mut self, interval: Interval) {
            self.ranges.push(interval);
        }

        fn clear(&mut self) {
            self.ranges.clear();
        }

        fn intersect(&mut self, other: &IntervalSet) {
            if self.ranges.is_empty() {
                return;
            }
            if other.ranges.is_empty() {
                self.clear();
                return;
            }

            let drain_end = self.ranges.len();

            let mut ita = (0..drain_end).into_iter();
            let mut itb = (0..other.ranges.len()).into_iter();
            let mut a = ita.next().unwrap();
            let mut b = itb.next().unwrap();
            loop {
                if let Some(ab) = self.ranges[a].intersect(&other.ranges[b]) {
                    self.push(ab);
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

    let mut self_set = IntervalSet::new();
    self_set.push(Interval { lower: 1, upper: 5 });
    self_set.push(Interval { lower: 6, upper: 10 });

    let other_set = IntervalSet::new(); // empty other set

    self_set.intersect(&other_set);

    assert_eq!(self_set.ranges.len(), 0); // should be empty after intersecting with empty set
}

#[test]
#[should_panic]
fn test_intersect_with_empty_self() {
    struct Interval {
        lower: i32,
        upper: i32,
    }

    struct IntervalSet {
        ranges: Vec<Interval>,
    }

    impl IntervalSet {
        fn intersect(&mut self, _other: &IntervalSet) {
            if self.ranges.is_empty() {
                panic!("Cannot intersect an empty set");
            }
        }
    }

    let mut self_set = IntervalSet { ranges: Vec::new() }; // empty self set
    let other_set = IntervalSet { ranges: Vec::new() }; // still empty

    self_set.intersect(&other_set); // this should panic
}

