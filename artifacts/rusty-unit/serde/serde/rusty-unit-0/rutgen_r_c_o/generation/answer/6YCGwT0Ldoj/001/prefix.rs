// Answer 0

#[test]
fn test_new_empty_array() {
    let mut bytes: [u8; 0] = [];
    let buf = Buf::new(&mut bytes);
}

#[test]
fn test_new_single_element_array() {
    let mut bytes: [u8; 1] = [1];
    let buf = Buf::new(&mut bytes);
}

#[test]
fn test_new_full_capacity_array() {
    let mut bytes: [u8; 1024] = [0; 1024];
    let buf = Buf::new(&mut bytes);
}

#[test]
#[should_panic]
fn test_new_exceeding_capacity_array() {
    let mut bytes: [u8; 1025] = [0; 1025];
    let buf = Buf::new(&mut bytes);
}

