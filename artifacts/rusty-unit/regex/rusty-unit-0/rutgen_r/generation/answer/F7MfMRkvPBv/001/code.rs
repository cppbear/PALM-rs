// Answer 0

#[test]
fn test_symmetric_difference_no_overlap() {
    struct IntervalSet<I> {
        intervals: Vec<I>,
    }

    impl<I: Clone + PartialEq> IntervalSet<I> {
        fn new() -> Self {
            IntervalSet {
                intervals: Vec::new(),
            }
        }

        fn union(&mut self, other: &IntervalSet<I>) {
            self.intervals.extend(other.intervals.clone());
        }

        fn difference(&mut self, other: &IntervalSet<I>) {
            self.intervals.retain(|i| !other.intervals.contains(i));
        }

        fn intersect(&mut self, other: &IntervalSet<I>) {
            self.intervals.retain(|i| other.intervals.contains(i));
        }

        fn clone(&self) -> Self {
            IntervalSet {
                intervals: self.intervals.clone(),
            }
        }

        fn symmetric_difference(&mut self, other: &IntervalSet<I>) {
            let mut intersection = self.clone();
            intersection.intersect(other);
            self.union(other);
            self.difference(&intersection);
        }
    }

    let mut set_a = IntervalSet::new();
    set_a.intervals.push(1);
    set_a.intervals.push(2);
    
    let mut set_b = IntervalSet::new();
    set_b.intervals.push(3);
    set_b.intervals.push(4);
    
    set_a.symmetric_difference(&set_b);
    
    assert_eq!(set_a.intervals, vec![1, 2, 3, 4]);
}

#[test]
fn test_symmetric_difference_with_overlap() {
    struct IntervalSet<I> {
        intervals: Vec<I>,
    }

    impl<I: Clone + PartialEq> IntervalSet<I> {
        fn new() -> Self {
            IntervalSet {
                intervals: Vec::new(),
            }
        }

        fn union(&mut self, other: &IntervalSet<I>) {
            self.intervals.extend(other.intervals.clone());
        }

        fn difference(&mut self, other: &IntervalSet<I>) {
            self.intervals.retain(|i| !other.intervals.contains(i));
        }

        fn intersect(&mut self, other: &IntervalSet<I>) {
            self.intervals.retain(|i| other.intervals.contains(i));
        }

        fn clone(&self) -> Self {
            IntervalSet {
                intervals: self.intervals.clone(),
            }
        }

        fn symmetric_difference(&mut self, other: &IntervalSet<I>) {
            let mut intersection = self.clone();
            intersection.intersect(other);
            self.union(other);
            self.difference(&intersection);
        }
    }

    let mut set_a = IntervalSet::new();
    set_a.intervals.push(1);
    set_a.intervals.push(2);
    
    let mut set_b = IntervalSet::new();
    set_b.intervals.push(2);
    set_b.intervals.push(3);
    
    set_a.symmetric_difference(&set_b);
    
    assert_eq!(set_a.intervals, vec![1, 3]);
}

#[test]
fn test_symmetric_difference_empty_sets() {
    struct IntervalSet<I> {
        intervals: Vec<I>,
    }

    impl<I: Clone + PartialEq> IntervalSet<I> {
        fn new() -> Self {
            IntervalSet {
                intervals: Vec::new(),
            }
        }

        fn union(&mut self, other: &IntervalSet<I>) {
            self.intervals.extend(other.intervals.clone());
        }

        fn difference(&mut self, other: &IntervalSet<I>) {
            self.intervals.retain(|i| !other.intervals.contains(i));
        }

        fn intersect(&mut self, other: &IntervalSet<I>) {
            self.intervals.retain(|i| other.intervals.contains(i));
        }

        fn clone(&self) -> Self {
            IntervalSet {
                intervals: self.intervals.clone(),
            }
        }

        fn symmetric_difference(&mut self, other: &IntervalSet<I>) {
            let mut intersection = self.clone();
            intersection.intersect(other);
            self.union(other);
            self.difference(&intersection);
        }
    }

    let mut set_a = IntervalSet::new();
    let mut set_b = IntervalSet::new();
    
    set_a.symmetric_difference(&set_b);
    
    assert_eq!(set_a.intervals, Vec::<i32>::new());
}

#[test]
fn test_symmetric_difference_self() {
    struct IntervalSet<I> {
        intervals: Vec<I>,
    }

    impl<I: Clone + PartialEq> IntervalSet<I> {
        fn new() -> Self {
            IntervalSet {
                intervals: Vec::new(),
            }
        }

        fn union(&mut self, other: &IntervalSet<I>) {
            self.intervals.extend(other.intervals.clone());
        }

        fn difference(&mut self, other: &IntervalSet<I>) {
            self.intervals.retain(|i| !other.intervals.contains(i));
        }

        fn intersect(&mut self, other: &IntervalSet<I>) {
            self.intervals.retain(|i| other.intervals.contains(i));
        }

        fn clone(&self) -> Self {
            IntervalSet {
                intervals: self.intervals.clone(),
            }
        }

        fn symmetric_difference(&mut self, other: &IntervalSet<I>) {
            let mut intersection = self.clone();
            intersection.intersect(other);
            self.union(other);
            self.difference(&intersection);
        }
    }

    let mut set_a = IntervalSet::new();
    set_a.intervals.push(1);
    set_a.intervals.push(2);
    
    set_a.symmetric_difference(&set_a);
    
    assert_eq!(set_a.intervals, Vec::<i32>::new());
}

