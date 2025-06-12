// Answer 0

#[test]
fn test_difference_non_empty_self_empty_other() {
    let mut interval_set_self = IntervalSet::new(vec![Interval::create(1, 5)]);
    let interval_set_other = IntervalSet::new(vec![]);

    interval_set_self.difference(&interval_set_other);
}

#[test]
fn test_difference_non_empty_self_empty_other_single_element() {
    let mut interval_set_self = IntervalSet::new(vec![Interval::create(10, 20)]);
    let interval_set_other = IntervalSet::new(vec![]);

    interval_set_self.difference(&interval_set_other);
}

#[test]
fn test_difference_non_empty_self_multiple_elements_other_empty() {
    let mut interval_set_self = IntervalSet::new(vec![Interval::create(1, 5), Interval::create(10, 15)]);
    let interval_set_other = IntervalSet::new(vec![]);

    interval_set_self.difference(&interval_set_other);
}

#[test]
fn test_difference_non_empty_self_with_large_range_other_empty() {
    let mut interval_set_self = IntervalSet::new(vec![Interval::create(100, 200)]);
    let interval_set_other = IntervalSet::new(vec![]);

    interval_set_self.difference(&interval_set_other);
}

#[test]
fn test_difference_non_empty_self_with_overlapping_ranges_other_empty() {
    let mut interval_set_self = IntervalSet::new(vec![Interval::create(1, 3), Interval::create(5, 7)]);
    let interval_set_other = IntervalSet::new(vec![]);

    interval_set_self.difference(&interval_set_other);
}

