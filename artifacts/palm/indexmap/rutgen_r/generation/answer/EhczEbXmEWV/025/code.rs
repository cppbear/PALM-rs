// Answer 0

fn test_simplify_range_included_start_end_with_len() {
    let range = 0..5; // Included start (0) and included end (5), len is also 5
    let len = 5;
    let simplified = simplify_range(range, len);
    assert_eq!(simplified, 0..5);
}

fn test_simplify_range_included_start_excluded_end() {
    let range = 0..4; // Included start (0) and excluded end (4), len is 5
    let len = 5;
    let simplified = simplify_range(range, len);
    assert_eq!(simplified, 0..4);
}

fn test_simplify_range_excluded_start_included_end() {
    let range = 1..=5; // Excluded start (1) and included end (5), len is 5
    let len = 5;
    let simplified = simplify_range(range, len);
    assert_eq!(simplified, 2..6);
}

fn test_simplify_range_excluded_start_excluded_end() {
    let range = 1..4; // Excluded start (1) and excluded end (4), len is 5
    let len = 5;
    let simplified = simplify_range(range, len);
    assert_eq!(simplified, 2..4);
}

fn test_simplify_range_unbounded_start() {
    let range = ..4; // Unbounded start and included end (4), len is 5
    let len = 5;
    let simplified = simplify_range(range, len);
    assert_eq!(simplified, 0..4);
}

fn test_simplify_range_unbounded_end() {
    let range = 2..; // Included start (2) and unbounded end, len is 5
    let len = 5;
    let simplified = simplify_range(range, len);
    assert_eq!(simplified, 2..5);
}

fn test_simplify_range_start_out_of_bounds() {
    let range = 6..10; // Included start (6) out of bounds, len is 5
    let len = 5;
    let result = std::panic::catch_unwind(|| {
        simplify_range(range, len);
    });
    assert!(result.is_err());
}

fn test_simplify_range_end_out_of_bounds() {
    let range = 0..=5; // Included end (5) out of bounds, len is 5
    let len = 5;
    let result = std::panic::catch_unwind(|| {
        simplify_range(range, len);
    });
    assert!(result.is_err());
}

fn test_simplify_range_start_greater_than_end() {
    let range = 5..=3; // Included start (5) greater than included end (3)
    let len = 5;
    let result = std::panic::catch_unwind(|| {
        simplify_range(range, len);
    });
    assert!(result.is_err());
}

