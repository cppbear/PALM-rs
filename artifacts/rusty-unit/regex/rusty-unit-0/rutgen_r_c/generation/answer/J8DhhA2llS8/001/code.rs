// Answer 0

#[test]
fn test_push_inst_ptr_incremental() {
    let mut data = Vec::new();
    let mut prev: InstPtr = 0;
    let ip: InstPtr = 10;
    push_inst_ptr(&mut data, &mut prev, ip);
    assert_eq!(data.len(), 1); // One byte should be written
    assert_eq!(prev, ip); // prev should be updated to ip
}

#[test]
fn test_push_inst_ptr_negative_delta() {
    let mut data = Vec::new();
    let mut prev: InstPtr = 10;
    let ip: InstPtr = 5;
    push_inst_ptr(&mut data, &mut prev, ip);
    assert_eq!(data.len(), 1); // One byte should be written
    assert_eq!(prev, ip); // prev should be updated to ip
}

#[test]
fn test_push_inst_ptr_zero_delta() {
    let mut data = Vec::new();
    let mut prev: InstPtr = 5;
    let ip: InstPtr = 5;
    push_inst_ptr(&mut data, &mut prev, ip);
    assert_eq!(data.len(), 1); // One byte should be written
    assert_eq!(prev, ip); // prev should be updated to ip
}

#[test]
fn test_push_inst_ptr_large_positive_delta() {
    let mut data = Vec::new();
    let mut prev: InstPtr = 0;
    let ip: InstPtr = STATE_MAX; // Testing with a large value
    push_inst_ptr(&mut data, &mut prev, ip);
    assert_eq!(data.len(), 1); // One byte should be written
    assert_eq!(prev, ip); // prev should be updated to ip
}

#[test]
fn test_push_inst_ptr_large_negative_delta() {
    let mut data = Vec::new();
    let mut prev: InstPtr = STATE_MAX;
    let ip: InstPtr = 0; // Testing with a large negative delta
    push_inst_ptr(&mut data, &mut prev, ip);
    assert_eq!(data.len(), 1); // One byte should be written
    assert_eq!(prev, ip); // prev should be updated to ip
}

