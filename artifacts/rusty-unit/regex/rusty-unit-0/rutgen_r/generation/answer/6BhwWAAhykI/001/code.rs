// Answer 0

#[test]
fn test_intersect_empty_self() {
    struct Interval {
        start: i32,
        end: i32,
    }
    
    impl Interval {
        pub fn intersect(&self, other: &Interval) -> Option<Interval> {
            let start = self.start.max(other.start);
            let end = self.end.min(other.end);
            if start <= end {
                Some(Interval { start, end })
            } else {
                None
            }
        }
    }

    struct IntervalSet<I> {
        ranges: Vec<I>,
    }
    
    impl<I> IntervalSet<I> {
        pub fn new() -> Self {
            IntervalSet { ranges: Vec::new() }
        }

        pub fn intersect(&mut self, other: &IntervalSet<I>) {
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
                let (it, aorb) = if self.ranges[a].end < other.ranges[b].end {
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
    let other_set = IntervalSet {
        ranges: vec![Interval { start: 1, end: 5 }],
    };

    self_set.intersect(&other_set);
    assert!(self_set.ranges.is_empty());
}

