// Answer 0

#[test]
fn test_negate_when_ranges_not_empty_lower_equal_min() {
    struct Bound(i32);
    
    impl Bound {
        fn min_value() -> Self {
            Bound(i32::MIN)
        }

        fn max_value() -> Self {
            Bound(i32::MAX)
        }
        
        fn decrement(&self) -> Self {
            Bound(self.0 - 1)
        }

        fn increment(&self) -> Self {
            Bound(self.0 + 1)
        }
    }

    struct Range {
        lower: Bound,
        upper: Bound,
    }

    impl Range {
        fn create(lower: Bound, upper: Bound) -> Self {
            Range { lower, upper }
        }

        fn lower(&self) -> &Bound {
            &self.lower
        }

        fn upper(&self) -> &Bound {
            &self.upper
        }
    }

    struct IntervalSet {
        ranges: Vec<Range>,
    }

    impl IntervalSet {
        fn new() -> Self {
            IntervalSet { ranges: Vec::new() }
        }
        
        fn negate(&mut self) {
            if self.ranges.is_empty() {
                let (min, max) = (Bound::min_value(), Bound::max_value());
                self.ranges.push(Range::create(min, max));
                return;
            }

            let drain_end = self.ranges.len();

            if self.ranges[0].lower().0 > Bound::min_value().0 {
                let upper = self.ranges[0].lower().decrement();
                self.ranges.push(Range::create(Bound::min_value(), upper));
            }
            for i in 1..drain_end {
                let lower = self.ranges[i - 1].upper().increment();
                let upper = self.ranges[i].lower().decrement();
                self.ranges.push(Range::create(lower, upper));
            }
            if self.ranges[drain_end - 1].upper().0 < Bound::max_value().0 {
                let lower = self.ranges[drain_end - 1].upper().increment();
                self.ranges.push(Range::create(lower, Bound::max_value()));
            }
            self.ranges.drain(..drain_end);
        }
    }

    let mut interval_set = IntervalSet::new();
    interval_set.ranges.push(Range::create(Bound(i32::MIN), Bound(i32::MIN + 10)));

    interval_set.negate();
    assert_eq!(interval_set.ranges.len(), 1);
    assert_eq!(interval_set.ranges[0].lower().0, i32::MIN);
    assert_eq!(interval_set.ranges[0].upper().0, i32::MIN + 10);
}

#[test]
fn test_negate_when_ranges_empty() {
    struct Bound(i32); // Same definition as before, omitted for brevity

    struct Range; // Same definition as before, omitted for brevity

    struct IntervalSet {
        ranges: Vec<Range>,
    } // Same definition as before, omitted for brevity

    let mut interval_set = IntervalSet::new();
    interval_set.negate();
    
    assert_eq!(interval_set.ranges.len(), 1);
    assert_eq!(interval_set.ranges[0].lower().0, i32::MIN);
    assert_eq!(interval_set.ranges[0].upper().0, i32::MAX);
}

#[test]
#[should_panic]
fn test_negate_when_panic_conditions_met() {
    struct Bound(i32); // Same definition as before, omitted for brevity

    struct Range; // Same definition as before, omitted for brevity

    struct IntervalSet {
        ranges: Vec<Range>,
    } // Same definition as before, omitted for brevity

    let mut interval_set = IntervalSet::new();
    // Populate in such a way to trigger panic conditions
    interval_set.ranges.push(Range::create(Bound(i32::MIN), Bound(i32::MAX)));
    interval_set.negate(); // Depending on the state may panic
}

