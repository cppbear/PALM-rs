// Answer 0

#[test]
fn test_parse_str_raw_empty_slice() {
    let mut scratch = Vec::new();
    let delegate = SliceRead { slice: &[], index: 0 };
    let mut reader = StrRead { delegate };

    let _ = reader.parse_str_raw(&mut scratch);
}

#[test]
fn test_parse_str_raw_single_byte() {
    let mut scratch = vec![0u8];
    let delegate = SliceRead { slice: &[1], index: 0 };
    let mut reader = StrRead { delegate };

    let _ = reader.parse_str_raw(&mut scratch);
}

#[test]
fn test_parse_str_raw_multiple_bytes() {
    let mut scratch = vec![0u8; 10];
    let delegate = SliceRead { slice: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], index: 0 };
    let mut reader = StrRead { delegate };

    let _ = reader.parse_str_raw(&mut scratch);
}

#[test]
fn test_parse_str_raw_large_slice() {
    let mut scratch = vec![0u8; 1000];
    let delegate = SliceRead { slice: &[0; 1000], index: 0 };
    let mut reader = StrRead { delegate };

    let _ = reader.parse_str_raw(&mut scratch);
}

#[test]
fn test_parse_str_raw_non_empty_scratch() {
    let mut scratch = vec![1u8, 2u8, 3u8];
    let delegate = SliceRead { slice: &[1, 2, 3, 4], index: 0 };
    let mut reader = StrRead { delegate };

    let _ = reader.parse_str_raw(&mut scratch);
} 

#[test]
#[should_panic]
fn test_parse_str_raw_invalid_state() {
    let mut scratch = Vec::new();
    let delegate = SliceRead { slice: &[1, 2, 3], index: 3 };
    let mut reader = StrRead { delegate };

    let _ = reader.parse_str_raw(&mut scratch);
}

