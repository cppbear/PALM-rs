// Answer 0

#[test]
fn test_case_fold_simple_empty() {
    let mut interval_set: IntervalSet<YourIntervalType> = IntervalSet::new(vec![]);
    interval_set.case_fold_simple();
}

#[test]
fn test_case_fold_simple_single() {
    let mut interval_set: IntervalSet<YourIntervalType> = IntervalSet::new(vec![YourIntervalType::create(1, 1)]);
    interval_set.case_fold_simple();
}

