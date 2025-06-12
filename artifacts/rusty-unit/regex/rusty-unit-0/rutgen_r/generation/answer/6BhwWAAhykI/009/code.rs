// Answer 0

#[derive(Debug)]
struct Interval {
    lower: i32,
    upper: i32,
}

impl Interval {
    fn new(lower: i32, upper: i32) -> Self {
        Interval { lower, upper }
    }

    fn intersect(&self, other: &Interval) -> Option<Interval> {
        if self.lower > other.upper || other.lower > self.upper {
            None
        } else {
            Some(Interval::new(self.lower.max(other.lower), self.upper.min(other.upper)))
        }
    }
    
    fn upper(&self) -> i32 {
        self.upper
    }
}

#[derive(Debug)]
struct IntervalSet {
    ranges: Vec<Interval>,
}

impl IntervalSet {
    fn new() -> Self {
        IntervalSet { ranges: Vec::new() }
    }
    
    fn add_range(&mut self, interval: Interval) {
        self.ranges.push(interval);
    }
    
    fn intersect(&mut self, other: &IntervalSet) {
        // Given function implementation
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
    let mut set_a = IntervalSet::new();
    set_a.add_range(Interval::new(1, 5));
    set_a.add_range(Interval::new(6, 10));

    let mut set_b = IntervalSet::new();
    set_b.add_range(Interval::new(4, 8));
    set_b.add_range(Interval::new(9, 12));

    set_a.intersect(&set_b);

    assert_eq!(set_a.ranges.len(), 3);
    assert_eq!(set_a.ranges[0], Interval::new(4, 5));
    assert_eq!(set_a.ranges[1], Interval::new(6, 8));
    assert_eq!(set_a.ranges[2], Interval::new(9, 10));
}

#[test]
fn test_intersect_same_ranges() {
    let mut set_a = IntervalSet::new();
    set_a.add_range(Interval::new(1, 5));

    let mut set_b = IntervalSet::new();
    set_b.add_range(Interval::new(1, 5));

    set_a.intersect(&set_b);

    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0], Interval::new(1, 5));
}

#[test]
fn test_intersect_no_overlap() {
    let mut set_a = IntervalSet::new();
    set_a.add_range(Interval::new(1, 2));

    let mut set_b = IntervalSet::new();
    set_b.add_range(Interval::new(3, 4));

    set_a.intersect(&set_b);

    assert_eq!(set_a.ranges.len(), 0);
}

#[test]
fn test_intersect_empty_set_b() {
    let mut set_a = IntervalSet::new();
    set_a.add_range(Interval::new(1, 5));

    let set_b = IntervalSet::new();

    set_a.intersect(&set_b);

    assert_eq!(set_a.ranges.len(), 0);
}

