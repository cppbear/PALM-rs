// Answer 0

#[test]
fn test_negate_when_ranges_is_empty() {
    struct Bound {
        value: i32,
    }

    impl Bound {
        fn min_value() -> Self {
            Bound { value: i32::MIN }
        }

        fn max_value() -> Self {
            Bound { value: i32::MAX }
        }
    }

    struct Interval {
        lower: Bound,
        upper: Bound,
    }

    impl Interval {
        fn create(lower: Bound, upper: Bound) -> Self {
            Interval { lower, upper }
        }

        fn lower(&self) -> &Bound {
            &self.lower
        }

        fn upper(&self) -> &Bound {
            &self.upper
        }
    }

    struct IntervalSet {
        ranges: Vec<Interval>,
    }

    impl IntervalSet {
        fn new() -> Self {
            IntervalSet { ranges: Vec::new() }
        }

        fn negate(&mut self) {
            if self.ranges.is_empty() {
                let (min, max) = (Bound::min_value(), Bound::max_value());
                self.ranges.push(Interval::create(min, max));
                return;
            }

            let drain_end = self.ranges.len();

            if self.ranges[0].lower().value > Bound::min_value().value {
                let upper = Bound { value: self.ranges[0].lower().value - 1 };
                self.ranges.push(Interval::create(Bound::min_value(), upper));
            }
            for i in 1..drain_end {
                let lower = Bound { value: self.ranges[i - 1].upper().value + 1 };
                let upper = Bound { value: self.ranges[i].lower().value - 1 };
                self.ranges.push(Interval::create(lower, upper));
            }
            if self.ranges[drain_end - 1].upper().value < Bound::max_value().value {
                let lower = Bound { value: self.ranges[drain_end - 1].upper().value + 1 };
                self.ranges.push(Interval::create(lower, Bound::max_value()));
            }
            self.ranges.drain(..drain_end);
        }
    }

    let mut interval_set = IntervalSet::new();
    interval_set.negate();

    assert_eq!(interval_set.ranges.len(), 1);
    assert_eq!(interval_set.ranges[0].lower().value, i32::MIN);
    assert_eq!(interval_set.ranges[0].upper().value, i32::MAX);
}

