// Answer 0

#[test]
fn test_to_raw_capacity_normal_value() {
    let n = 3;
    to_raw_capacity(n);
}

#[test]
fn test_to_raw_capacity_large_value() {
    let n = 4294967295;
    to_raw_capacity(n);
}

#[test]
fn test_to_raw_capacity_mid_value() {
    let n = 100;
    to_raw_capacity(n);
}

#[test]
fn test_to_raw_capacity_edge_case() {
    let n = 1073741823; // Close to the maximum before overflow
    to_raw_capacity(n);
}

