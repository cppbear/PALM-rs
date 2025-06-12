// Answer 0

#[test]
fn test_repetition_range_bounded_invalid() {
    let range1 = RepetitionRange::Bounded(1, 0);
    let result1 = range1.is_valid();

    let range2 = RepetitionRange::Bounded(10, 5);
    let result2 = range2.is_valid();

    let range3 = RepetitionRange::Bounded(1000, 999);
    let result3 = range3.is_valid();

    let range4 = RepetitionRange::Bounded(u32::MAX, u32::MAX - 1);
    let result4 = range4.is_valid();

    let range5 = RepetitionRange::Bounded(20, 15);
    let result5 = range5.is_valid();
}

