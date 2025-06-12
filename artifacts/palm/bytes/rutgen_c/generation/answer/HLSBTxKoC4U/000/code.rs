// Answer 0

#[test]
fn test_advance_mut_with_sufficient_capacity() {
    let mut data = vec![core::mem::MaybeUninit::<u8>::uninit(); 10];
    let ptr: &mut [core::mem::MaybeUninit<u8>] = data.as_mut_slice();
    
    unsafe {
        let initial_len = ptr.len();
        let cnt = 5;
        ptr.advance_mut(cnt);
        assert_eq!(ptr.len(), initial_len - cnt);
    }
}

#[test]
#[should_panic(expected = "requested")]
fn test_advance_mut_with_insufficient_capacity() {
    let mut data = vec![core::mem::MaybeUninit::<u8>::uninit(); 5];
    let ptr: &mut [core::mem::MaybeUninit<u8>] = data.as_mut_slice();
    
    unsafe {
        ptr.advance_mut(6);
    }
}

