pub fn contains_simple_case_mapping(start: char, end: char) -> bool {
    assert!(start <= end);
    CASE_FOLDING_SIMPLE
        .binary_search_by(|&(c, _)| {
            if start <= c && c <= end {
                Ordering::Equal
            } else if c > end {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }).is_ok()
}