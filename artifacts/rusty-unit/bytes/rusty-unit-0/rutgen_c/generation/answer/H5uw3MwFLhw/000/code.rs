// Answer 0

#[test]
fn test_from_raw_parts_mut() {
    let mut data: Vec<u8> = vec![1, 2, 3, 4, 5];
    let ptr = data.as_mut_ptr();
    let len = data.len();

    let slice: &mut UninitSlice;
    unsafe {
        slice = UninitSlice::from_raw_parts_mut(ptr, len);
    }

    assert_eq!(slice.len(), len);
}

#[test]
#[should_panic]
fn test_from_raw_parts_mut_zero_length() {
    let mut data: Vec<u8> = vec![];
    let ptr = data.as_mut_ptr();
    let len = data.len();

    let slice: &mut UninitSlice;
    unsafe {
        slice = UninitSlice::from_raw_parts_mut(ptr, len);
    }

    assert_eq!(slice.len(), len);
}

#[test]
fn test_from_raw_parts_mut_with_modified_data() {
    let mut data: Vec<u8> = vec![10, 20, 30];
    let ptr = data.as_mut_ptr();
    let len = data.len();

    let slice: &mut UninitSlice;
    unsafe {
        slice = UninitSlice::from_raw_parts_mut(ptr, len);
        slice.write_byte(0, 100);
        assert_eq!(data[0], 100);
    }
}

