// Answer 0

#[test]
fn test_intersect_empty_self_ranges() {
    let mut interval_set_a: IntervalSet<SomeIntervalType> = IntervalSet::new(vec![]);
    let interval_set_b: IntervalSet<SomeIntervalType> = IntervalSet::new(vec![SomeIntervalType::create(1, 5)]);
    interval_set_a.intersect(&interval_set_b);
}

#[test]
fn test_intersect_empty_other_ranges() {
    let mut interval_set_a: IntervalSet<SomeIntervalType> = IntervalSet::new(vec![SomeIntervalType::create(1, 3)]);
    let interval_set_b: IntervalSet<SomeIntervalType> = IntervalSet::new(vec![]);
    interval_set_a.intersect(&interval_set_b);
}

#[test]
fn test_intersect_both_empty_ranges() {
    let mut interval_set_a: IntervalSet<SomeIntervalType> = IntervalSet::new(vec![]);
    let interval_set_b: IntervalSet<SomeIntervalType> = IntervalSet::new(vec![]);
    interval_set_a.intersect(&interval_set_b);
}

#[test]
fn test_intersect_self_empty_other_non_empty() {
    let mut interval_set_a: IntervalSet<SomeIntervalType> = IntervalSet::new(vec![]);
    let interval_set_b: IntervalSet<SomeIntervalType> = IntervalSet::new(vec![SomeIntervalType::create(4, 6)]);
    interval_set_a.intersect(&interval_set_b);
}

#[test]
fn test_intersect_non_empty_self_non_empty_other() {
    let mut interval_set_a: IntervalSet<SomeIntervalType> = IntervalSet::new(vec![SomeIntervalType::create(1, 3)]);
    let interval_set_b: IntervalSet<SomeIntervalType> = IntervalSet::new(vec![SomeIntervalType::create(2, 4)]);
    interval_set_a.intersect(&interval_set_b);
}

