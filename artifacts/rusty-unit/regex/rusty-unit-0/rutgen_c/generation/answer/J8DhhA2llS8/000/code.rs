// Answer 0

#[test]
fn test_push_inst_ptr_with_positive_delta() {
    let mut data = Vec::new();
    let mut prev: InstPtr = 0;
    let ip: InstPtr = 5;

    push_inst_ptr(&mut data, &mut prev, ip);

    assert_eq!(data.len(), 1); // Ensure data has been written
    assert_eq!(prev, ip); // Check that prev is updated to ip
}

#[test]
fn test_push_inst_ptr_with_negative_delta() {
    let mut data = Vec::new();
    let mut prev: InstPtr = 10;
    let ip: InstPtr = 5;

    push_inst_ptr(&mut data, &mut prev, ip);

    assert_eq!(data.len(), 1); // Ensure data has been written
    assert_eq!(prev, ip); // Check that prev is updated to ip
}

#[test]
fn test_push_inst_ptr_with_zero_delta() {
    let mut data = Vec::new();
    let mut prev: InstPtr = 5;
    let ip: InstPtr = 5;

    push_inst_ptr(&mut data, &mut prev, ip);

    assert_eq!(data.len(), 1); // Ensure data has been written
    assert_eq!(prev, ip); // Check that prev is updated to ip
}

#[test]
fn test_push_inst_ptr_multiple_updates() {
    let mut data = Vec::new();
    let mut prev: InstPtr = 0;

    let ips: [InstPtr; 3] = [3, 7, 2];
    for &ip in &ips {
        push_inst_ptr(&mut data, &mut prev, ip);
    }

    assert_eq!(data.len(), 3); // Ensure data has been written for each ip
}

