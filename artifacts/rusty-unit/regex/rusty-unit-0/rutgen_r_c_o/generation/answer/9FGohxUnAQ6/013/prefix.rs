// Answer 0

#[test]
fn test_difference_non_overlapping_ranges() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct MyBound(u32);
    impl Bound for MyBound {
        fn increment(&self) -> Self {
            MyBound(self.0 + 1)
        }
        fn decrement(&self) -> Self {
            MyBound(self.0 - 1)
        }
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct MyInterval {
        lower: MyBound,
        upper: MyBound,
    }
    impl Interval for MyInterval {
        type Bound = MyBound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, _: &Self) -> bool { false }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![
        MyInterval { lower: MyBound(1), upper: MyBound(10) }
    ]);

    let set_b = IntervalSet::new(vec![
        MyInterval { lower: MyBound(10), upper: MyBound(20) }
    ]);

    set_a.difference(&set_b);
}

#[test]
fn test_difference_intersecting_ranges() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct MyBound(u32);
    impl Bound for MyBound {
        fn increment(&self) -> Self {
            MyBound(self.0 + 1)
        }
        fn decrement(&self) -> Self {
            MyBound(self.0 - 1)
        }
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct MyInterval {
        lower: MyBound,
        upper: MyBound,
    }
    impl Interval for MyInterval {
        type Bound = MyBound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, other: &Self) -> bool { self.upper.0 >= other.lower.0 && self.lower.0 <= other.upper.0 }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper.0 < other.lower.0 || self.lower.0 > other.upper.0 }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![
        MyInterval { lower: MyBound(5), upper: MyBound(15) }
    ]);

    let set_b = IntervalSet::new(vec![
        MyInterval { lower: MyBound(10), upper: MyBound(20) }
    ]);

    set_a.difference(&set_b);
}

#[test]
fn test_difference_exact_overlap() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct MyBound(u32);
    impl Bound for MyBound {
        fn increment(&self) -> Self {
            MyBound(self.0 + 1)
        }
        fn decrement(&self) -> Self {
            MyBound(self.0 - 1)
        }
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct MyInterval {
        lower: MyBound,
        upper: MyBound,
    }
    impl Interval for MyInterval {
        type Bound = MyBound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, _: &Self) -> bool { false }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![
        MyInterval { lower: MyBound(5), upper: MyBound(15) }
    ]);

    let set_b = IntervalSet::new(vec![
        MyInterval { lower: MyBound(5), upper: MyBound(15) }
    ]);

    set_a.difference(&set_b);
}

#[test]
fn test_difference_multiple_ranges() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct MyBound(u32);
    impl Bound for MyBound {
        fn increment(&self) -> Self {
            MyBound(self.0 + 1)
        }
        fn decrement(&self) -> Self {
            MyBound(self.0 - 1)
        }
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct MyInterval {
        lower: MyBound,
        upper: MyBound,
    }
    impl Interval for MyInterval {
        type Bound = MyBound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, _: &Self) -> bool { false }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![
        MyInterval { lower: MyBound(1), upper: MyBound(5) },
        MyInterval { lower: MyBound(6), upper: MyBound(8) }
    ]);

    let set_b = IntervalSet::new(vec![
        MyInterval { lower: MyBound(4), upper: MyBound(7) }
    ]);

    set_a.difference(&set_b);
}

