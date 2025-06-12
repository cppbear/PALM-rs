// Answer 0

#[test]
fn test_advance_unchecked_with_zero_count() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.advance_unchecked(0);
    }
}

#[test]
fn test_advance_unchecked_with_count_equals_cap() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.resize(10, 0);
        bytes_mut.advance_unchecked(10);
    }
}

#[test]
#[should_panic]
fn test_advance_unchecked_with_exceeding_pos() {
    let mut bytes_mut = BytesMut::with_capacity(134_217_728); // should exceed MAX_VEC_POS
    unsafe {
        bytes_mut.resize(134_217_728, 0);
        bytes_mut.advance_unchecked(1);
    }
} 

#[test]
fn test_advance_unchecked_with_valid_cap() {
    let mut bytes_mut = BytesMut::with_capacity(20);
    unsafe {
        bytes_mut.resize(20, 0);
        bytes_mut.advance_unchecked(5);
    }
} 

#[test]
#[should_panic]
fn test_advance_unchecked_panic_on_invalid_count() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.advance_unchecked(11); // count > cap
    }
} 

#[test]
fn test_advance_unchecked_with_potential_positive_count() {
    let mut bytes_mut = BytesMut::with_capacity(30);
    unsafe {
        bytes_mut.resize(30, 0);
        bytes_mut.advance_unchecked(15);
    }
} 

