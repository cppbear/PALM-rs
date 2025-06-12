// Answer 0

#[test]
fn test_advance_mut_zero() {
    let mut buffer = vec![0u8; 10];
    let mut cursor = unsafe { &mut *(&mut buffer as *mut Vec<u8> as *mut std::slice::SliceMut<u8>) };
    
    unsafe {
        cursor.advance_mut(0);
    }
    
    assert_eq!(buffer.len(), 10);
}

#[test]
fn test_advance_mut_within_bounds() {
    let mut buffer = vec![0u8; 10];
    let mut cursor = unsafe { &mut *(&mut buffer as *mut Vec<u8> as *mut std::slice::SliceMut<u8>) };
    
    unsafe {
        cursor.advance_mut(5);
    }
    
    assert_eq!(buffer.len(), 10);
}

#[should_panic]
fn test_advance_mut_out_of_bounds() {
    let mut buffer = vec![0u8; 10];
    let mut cursor = unsafe { &mut *(&mut buffer as *mut Vec<u8> as *mut std::slice::SliceMut<u8>) };
    
    unsafe {
        cursor.advance_mut(15);
    }
}

