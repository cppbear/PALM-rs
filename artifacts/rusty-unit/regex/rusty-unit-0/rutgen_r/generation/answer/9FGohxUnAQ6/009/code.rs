// Answer 0

#[derive(Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn upper(&self) -> usize {
        self.end
    }

    fn lower(&self) -> usize {
        self.start
    }

    fn is_intersection_empty(&self, other: &Range) -> bool {
        self.upper() < other.lower() || self.lower() > other.upper()
    }

    fn difference(&self, other: &Range) -> (Option<Range>, Option<Range>) {
        if self.is_intersection_empty(other) {
            return (Some(self.clone()), None);
        }
        if self.lower() < other.lower() {
            let new_range = Range {
                start: self.lower(),
                end: other.lower(),
            };
            if self.upper() > other.upper() {
                return (Some(new_range), Some(Range {
                    start: other.upper(),
                    end: self.upper(),
                }));
            }
            return (Some(new_range), None);
        } else if self.upper() > other.upper() {
            return (None, Some(Range {
                start: other.upper(),
                end: self.upper(),
            }));
        }
        (None, None)
    }
}

#[derive(Debug, Default)]
struct IntervalSet<I> {
    ranges: Vec<I>,
}

impl<I> IntervalSet<I> {
    pub fn new(ranges: Vec<I>) -> Self {
        Self { ranges }
    }

    pub fn difference(&mut self, other: &IntervalSet<I>) {
        // (implementation omitted for brevity; use the function provided)
    }
}

#[test]
fn test_difference_removes_overlapping_ranges() {
    let mut set_a = IntervalSet::new(vec![
        Range { start: 1, end: 5 },
        Range { start: 10, end: 15 },
    ]);
    let set_b = IntervalSet::new(vec![
        Range { start: 3, end: 4 },
        Range { start: 12, end: 13 },
    ]);

    set_a.difference(&set_b);

    assert_eq!(set_a.ranges.len(), 2);
    assert_eq!(set_a.ranges[0], Range { start: 1, end: 2 });
    assert_eq!(set_a.ranges[1], Range { start: 10, end: 11 });
}

#[test]
fn test_difference_removes_all_elements() {
    let mut set_a = IntervalSet::new(vec![
        Range { start: 1, end: 5 },
    ]);
    let set_b = IntervalSet::new(vec![
        Range { start: 1, end: 5 },
    ]);

    set_a.difference(&set_b);

    assert!(set_a.ranges.is_empty());
}

#[test]
fn test_difference_no_change_when_non_overlapping() {
    let mut set_a = IntervalSet::new(vec![
        Range { start: 1, end: 2 },
        Range { start: 5, end: 6 },
    ]);
    let set_b = IntervalSet::new(vec![
        Range { start: 3, end: 4 },
    ]);

    set_a.difference(&set_b);

    assert_eq!(set_a.ranges.len(), 2);
    assert_eq!(set_a.ranges[0], Range { start: 1, end: 2 });
    assert_eq!(set_a.ranges[1], Range { start: 5, end: 6 });
}

#[test]
fn test_difference_edge_cases() {
    let mut set_a = IntervalSet::new(vec![
        Range { start: 2, end: 5 },
    ]);
    let set_b = IntervalSet::new(vec![
        Range { start: 5, end: 6 },
    ]);

    set_a.difference(&set_b);

    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0], Range { start: 2, end: 5 });
}

