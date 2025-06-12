// Answer 0

#[derive(Debug)]
struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn upper(&self) -> i32 {
        self.end
    }
    
    fn lower(&self) -> i32 {
        self.start
    }
    
    fn is_intersection_empty(&self, other: &Interval) -> bool {
        self.upper() < other.lower() || self.lower() > other.upper()
    }
    
    fn difference(&self, other: &Interval) -> (Option<Interval>, Option<Interval>) {
        if self.is_intersection_empty(other) {
            return (Some(*self), None);
        }
        
        if self.lower() < other.lower() {
            return (Some(Interval { start: self.start, end: other.lower() }), Some(Interval { start: other.lower(), end: self.end }));
        } else if self.upper() > other.upper() {
            return (Some(Interval { start: other.upper(), end: self.end }), Some(Interval { start: self.start, end: other.upper() }));
        }

        (None, None)
    }
}

#[derive(Debug)]
struct IntervalSet<I> {
    ranges: Vec<I>,
}

impl<I: IntervalTrait> IntervalSet<I> {
    pub fn new() -> Self {
        IntervalSet { ranges: Vec::new() }
    }
    
    pub fn difference(&mut self, other: &IntervalSet<I>) {
        if self.ranges.is_empty() || other.ranges.is_empty() {
            return;
        }

        let drain_end = self.ranges.len();
        let (mut a, mut b) = (0, 0);
'LOOP:
        while a < drain_end && b < other.ranges.len() {
            if other.ranges[b].upper() < self.ranges[a].lower() {
                b += 1;
                continue;
            }
            if self.ranges[a].upper() < other.ranges[b].lower() {
                let range = self.ranges[a];
                self.ranges.push(range);
                a += 1;
                continue;
            }
            assert!(!self.ranges[a].is_intersection_empty(&other.ranges[b]));

            let mut range = self.ranges[a];
            while b < other.ranges.len() && !range.is_intersection_empty(&other.ranges[b]) {
                let old_range = range;
                range = match range.difference(&other.ranges[b]) {
                    (None, None) => {
                        a += 1;
                        continue 'LOOP;
                    }
                    (Some(range1), None) | (None, Some(range1)) => range1,
                    (Some(range1), Some(range2)) => {
                        self.ranges.push(range1);
                        range2
                    }
                };
                if other.ranges[b].upper() > old_range.upper() {
                    break;
                }
                b += 1;
            }
            self.ranges.push(range);
            a += 1;
        }
        while a < drain_end {
            let range = self.ranges[a];
            self.ranges.push(range);
            a += 1;
        }
        self.ranges.drain(..drain_end);
    }
}

#[test]
fn test_difference_non_empty_sets() {
    let mut set_a = IntervalSet::new();
    set_a.ranges.push(Interval { start: 1, end: 5 });
    set_a.ranges.push(Interval { start: 6, end: 10 });

    let mut set_b = IntervalSet::new();
    set_b.ranges.push(Interval { start: 3, end: 7 });

    set_a.difference(&set_b);
    
    assert_eq!(set_a.ranges.len(), 3);
    assert_eq!(set_a.ranges[0].start, 1);
    assert_eq!(set_a.ranges[0].end, 3);
    assert_eq!(set_a.ranges[1].start, 7);
    assert_eq!(set_a.ranges[1].end, 10);
}

#[test]
fn test_difference_complete_overlap() {
    let mut set_a = IntervalSet::new();
    set_a.ranges.push(Interval { start: 1, end: 5 });

    let mut set_b = IntervalSet::new();
    set_b.ranges.push(Interval { start: 1, end: 5 });

    set_a.difference(&set_b);
    
    assert_eq!(set_a.ranges.len(), 0);
}

#[test]
fn test_difference_edge_case() {
    let mut set_a = IntervalSet::new();
    set_a.ranges.push(Interval { start: 1, end: 5 });

    let mut set_b = IntervalSet::new();
    set_b.ranges.push(Interval { start: 5, end: 6 });

    set_a.difference(&set_b);
    
    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0].start, 1);
    assert_eq!(set_a.ranges[0].end, 5);
}

#[test]
fn test_difference_no_intersection() {
    let mut set_a = IntervalSet::new();
    set_a.ranges.push(Interval { start: 1, end: 2 });

    let mut set_b = IntervalSet::new();
    set_b.ranges.push(Interval { start: 3, end: 4 });

    set_a.difference(&set_b);
    
    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0].start, 1);
    assert_eq!(set_a.ranges[0].end, 2);
}

