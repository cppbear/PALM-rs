// Answer 0

#[test]
fn test_last_mut_with_non_empty_buffers() {
    let mut chain = Chain::new(&mut [1, 2, 3][..], &mut [4, 5, 6][..]);
    let last_buf = chain.last_mut();
}

#[test]
fn test_last_mut_with_empty_first_buffer() {
    let mut chain = Chain::new(&mut [][..], &mut [7, 8, 9][..]);
    let last_buf = chain.last_mut();
}

#[test]
fn test_last_mut_with_empty_second_buffer() {
    let mut chain = Chain::new(&mut [10, 11][..], &mut [][..]);
    let last_buf = chain.last_mut();
}

#[test]
fn test_last_mut_with_full_range_buffers() {
    let mut chain = Chain::new(&mut [12; 10][..], &mut [13; 10][..]);
    let last_buf = chain.last_mut();
}

#[test]
#[should_panic]
fn test_last_mut_with_all_empty_buffers() {
    let mut chain = Chain::new(&mut [][..], &mut [][..]);
    let last_buf = chain.last_mut();
}

