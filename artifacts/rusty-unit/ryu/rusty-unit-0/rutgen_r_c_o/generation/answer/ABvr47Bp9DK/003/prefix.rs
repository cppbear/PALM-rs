// Answer 0

#[should_panic]
fn test_log10_pow5_negative_input() {
    let _ = log10_pow5(-1);
}

#[should_panic]
fn test_log10_pow5_minimum_negative_input() {
    let _ = log10_pow5(-100);
}

#[should_panic]
fn test_log10_pow5_large_negative_input() {
    let _ = log10_pow5(-5000);
}

