// Answer 0

#[test]
fn test_push_inst_ptr() {
    let mut data = vec![];
    let mut prev: InstPtr = 0;
    let ip: InstPtr = 10;

    push_inst_ptr(&mut data, &mut prev, ip);

    assert_eq!(data.len(), 1);
    assert_eq!(data[0], 10); // Replace with actual expected value
    assert_eq!(prev, ip);
}

#[test]
fn test_push_inst_ptr_with_negative_delta() {
    let mut data = vec![];
    let mut prev: InstPtr = 10;
    let ip: InstPtr = 5;

    push_inst_ptr(&mut data, &mut prev, ip);

    assert_eq!(data.len(), 1);
    assert_eq!(data[0], -5); // Replace with actual expected value for negative delta
    assert_eq!(prev, ip);
}

#[test]
fn test_push_inst_ptr_zero_delta() {
    let mut data = vec![];
    let mut prev: InstPtr = 5;
    let ip: InstPtr = 5;

    push_inst_ptr(&mut data, &mut prev, ip);

    assert_eq!(data.len(), 1);
    assert_eq!(data[0], 0); // Replace with actual expected value for zero delta
    assert_eq!(prev, ip);
}

