// Answer 0

#[test]
fn test_new_with_non_overlapping_intervals() {
    let intervals = vec![(1, 5), (6, 10)];
    let set = regex_syntax::hir::interval::new(intervals);
    assert_eq!(set.ranges.len(), 2);
    assert!(set.contains(&(3, 4)));
    assert!(!set.contains(&(11, 12)));
}

#[test]
fn test_new_with_overlapping_intervals() {
    let intervals = vec![(1, 5), (4, 10), (8, 12)];
    let set = regex_syntax::hir::interval::new(intervals);
    assert_eq!(set.ranges.len(), 1); // Should merge overlapping intervals
    assert!(set.contains(&(2, 6)));
    assert!(set.contains(&(9, 11)));
}

#[test]
fn test_new_with_single_interval() {
    let intervals = vec![(1, 5)];
    let set = regex_syntax::hir::interval::new(intervals);
    assert_eq!(set.ranges.len(), 1);
    assert!(set.contains(&(3, 4)));
    assert!(!set.contains(&(6, 7)));
}

#[test]
fn test_new_with_empty_intervals() {
    let intervals: Vec<(i32, i32)> = vec![];
    let set = regex_syntax::hir::interval::new(intervals);
    assert_eq!(set.ranges.len(), 0);
}

#[test]
fn test_new_with_full_range() {
    let intervals = vec![(1, 10), (2, 5), (6, 10)];
    let set = regex_syntax::hir::interval::new(intervals);
    assert_eq!(set.ranges.len(), 1); // Full range should be merged
    assert!(set.contains(&(1, 10)));
}

