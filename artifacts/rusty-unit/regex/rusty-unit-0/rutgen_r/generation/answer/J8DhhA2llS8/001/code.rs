// Answer 0

#[test]
fn test_push_inst_ptr_increase() {
    let mut data: Vec<u8> = Vec::new();
    let mut prev: InstPtr = 0;
    let ip: InstPtr = 10;

    push_inst_ptr(&mut data, &mut prev, ip);

    assert_eq!(data.len(), 4);
    assert_eq!(prev, ip);
}

#[test]
fn test_push_inst_ptr_decrease() {
    let mut data: Vec<u8> = Vec::new();
    let mut prev: InstPtr = 10;
    let ip: InstPtr = 5;

    push_inst_ptr(&mut data, &mut prev, ip);

    assert_eq!(data.len(), 4);
    assert_eq!(prev, ip);
}

#[test]
fn test_push_inst_ptr_no_change() {
    let mut data: Vec<u8> = Vec::new();
    let mut prev: InstPtr = 5;
    let ip: InstPtr = 5;

    push_inst_ptr(&mut data, &mut prev, ip);

    assert_eq!(data.len(), 4);
    assert_eq!(prev, ip);
}

#[test]
#[should_panic]
fn test_push_inst_ptr_overflow() {
    let mut data: Vec<u8> = Vec::new();
    let mut prev: InstPtr = std::i32::MAX as InstPtr; 
    let ip: InstPtr = std::i32::MAX as InstPtr; 

    push_inst_ptr(&mut data, &mut prev, ip);
}

#[test]
#[should_panic]
fn test_push_inst_ptr_underflow() {
    let mut data: Vec<u8> = Vec::new();
    let mut prev: InstPtr = std::i32::MIN as InstPtr; 
    let ip: InstPtr = std::i32::MIN as InstPtr; 

    push_inst_ptr(&mut data, &mut prev, ip);
}

