pub fn decimal_length9(v: u32) -> u32 {
    // Function precondition: v is not a 10-digit number.
    // (f2s: 9 digits are sufficient for round-tripping.)
    debug_assert!(v < 1000000000);

    if v >= 100000000 {
        9
    } else if v >= 10000000 {
        8
    } else if v >= 1000000 {
        7
    } else if v >= 100000 {
        6
    } else if v >= 10000 {
        5
    } else if v >= 1000 {
        4
    } else if v >= 100 {
        3
    } else if v >= 10 {
        2
    } else {
        1
    }
}