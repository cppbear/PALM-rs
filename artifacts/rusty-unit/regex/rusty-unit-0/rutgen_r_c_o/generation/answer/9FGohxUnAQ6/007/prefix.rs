// Answer 0

#[test]
fn test_difference_non_empty_intervals() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestInterval {
        lower_bound: usize,
        upper_bound: usize,
    }

    impl TestInterval {
        fn lower(&self) -> usize { self.lower_bound }
        fn upper(&self) -> usize { self.upper_bound }
        fn set_lower(&mut self, bound: usize) { self.lower_bound = bound; }
        fn set_upper(&mut self, bound: usize) { self.upper_bound = bound; }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper_bound < other.lower_bound || self.lower_bound > other.upper_bound
        }
        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            }
            let lower = self.lower();
            let upper = self.upper();
            let other_lower = other.lower();
            let other_upper = other.upper();
            if lower < other_lower {
                return (Some(Self::default()), Some(Self { lower_bound: other_upper + 1, upper_bound }));
            } else if upper > other_upper {
                return (Some(Self { lower_bound, upper_bound: other_lower - 1 }), None);
            }
            (None, None)
        }
    }
    
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower_bound: 10, upper_bound: 20 },
        TestInterval { lower_bound: 30, upper_bound: 40 },
    ]);
    
    let set_b = IntervalSet::new(vec![
        TestInterval { lower_bound: 20, upper_bound: 30 },
    ]);
    
    set_a.difference(&set_b);
}

#[test]
fn test_difference_intersecting_intervals() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestInterval {
        lower_bound: usize,
        upper_bound: usize,
    }

    impl TestInterval {
        fn lower(&self) -> usize { self.lower_bound }
        fn upper(&self) -> usize { self.upper_bound }
        fn set_lower(&mut self, bound: usize) { self.lower_bound = bound; }
        fn set_upper(&mut self, bound: usize) { self.upper_bound = bound; }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper_bound < other.lower_bound || self.lower_bound > other.upper_bound
        }
        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            }
            let lower = self.lower();
            let upper = self.upper();
            let other_lower = other.lower();
            let other_upper = other.upper();
            if lower < other_lower {
                return (Some(Self::default()), Some(Self { lower_bound: other_upper + 1, upper_bound }));
            } else if upper > other_upper {
                return (Some(Self { lower_bound, upper_bound: other_lower - 1 }), None);
            }
            (None, None)
        }
    }
    
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower_bound: 15, upper_bound: 35 },
    ]);
    
    let set_b = IntervalSet::new(vec![
        TestInterval { lower_bound: 25, upper_bound: 30 },
    ]);
    
    set_a.difference(&set_b);
} 

#[test]
fn test_difference_non_intersecting_higher() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestInterval {
        lower_bound: usize,
        upper_bound: usize,
    }

    impl TestInterval {
        fn lower(&self) -> usize { self.lower_bound }
        fn upper(&self) -> usize { self.upper_bound }
        fn set_lower(&mut self, bound: usize) { self.lower_bound = bound; }
        fn set_upper(&mut self, bound: usize) { self.upper_bound = bound; }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper_bound < other.lower_bound || self.lower_bound > other.upper_bound
        }
        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            }
            let lower = self.lower();
            let upper = self.upper();
            let other_lower = other.lower();
            let other_upper = other.upper();
            if lower < other_lower {
                return (Some(Self::default()), Some(Self { lower_bound: other_upper + 1, upper_bound }));
            } else if upper > other_upper {
                return (Some(Self { lower_bound, upper_bound: other_lower - 1 }), None);
            }
            (None, None)
        }
    }
    
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower_bound: 50, upper_bound: 60 },
    ]);
    
    let set_b = IntervalSet::new(vec![
        TestInterval { lower_bound: 30, upper_bound: 40 },
    ]);
    
    set_a.difference(&set_b);
}

#[test]
fn test_difference_completely_contained_intervals() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestInterval {
        lower_bound: usize,
        upper_bound: usize,
    }

    impl TestInterval {
        fn lower(&self) -> usize { self.lower_bound }
        fn upper(&self) -> usize { self.upper_bound }
        fn set_lower(&mut self, bound: usize) { self.lower_bound = bound; }
        fn set_upper(&mut self, bound: usize) { self.upper_bound = bound; }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper_bound < other.lower_bound || self.lower_bound > other.upper_bound
        }
        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            }
            let lower = self.lower();
            let upper = self.upper();
            let other_lower = other.lower();
            let other_upper = other.upper();
            if lower < other_lower {
                return (Some(Self::default()), Some(Self { lower_bound: other_upper + 1, upper_bound }));
            } else if upper > other_upper {
                return (Some(Self { lower_bound, upper_bound: other_lower - 1 }), None);
            }
            (None, None)
        }
    }
    
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower_bound: 10, upper_bound: 20 },
    ]);
    
    let set_b = IntervalSet::new(vec![
        TestInterval { lower_bound: 5, upper_bound: 15 },
    ]);
    
    set_a.difference(&set_b);
}

