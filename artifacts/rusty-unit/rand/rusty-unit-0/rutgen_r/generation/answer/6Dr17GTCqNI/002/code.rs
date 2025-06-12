// Answer 0

#[test]
fn test_fill_via_chunks_with_full_chunks_and_partial() {
    use std::convert::TryInto;

    struct ObservableStruct(u32);
    
    impl Observable for ObservableStruct {
        fn to_le_bytes(&self) -> [u8; core::mem::size_of::<u32>()] {
            self.0.to_le_bytes()
        }
    }

    let src: Vec<ObservableStruct> = vec![ObservableStruct(1), ObservableStruct(2), ObservableStruct(3)];
    let mut dest: Vec<u8> = vec![0; 12]; // 3 full chunks of 4 bytes each

    let (num_chunks, byte_len) = fill_via_chunks(&src, &mut dest);

    assert_eq!(num_chunks, 3);
    assert_eq!(byte_len, 12);
    assert_eq!(&dest[..], &[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]);
}

#[test]
fn test_fill_via_chunks_with_some_partial() {
    use std::convert::TryInto;

    struct ObservableStruct(u32);
    
    impl Observable for ObservableStruct {
        fn to_le_bytes(&self) -> [u8; core::mem::size_of::<u32>()] {
            self.0.to_le_bytes()
        }
    }

    let src: Vec<ObservableStruct> = vec![ObservableStruct(1), ObservableStruct(2)];
    let mut dest: Vec<u8> = vec![0; 8]; // 2 full chunks of 4 bytes each, with room for partial

    let (num_chunks, byte_len) = fill_via_chunks(&src, &mut dest);

    assert_eq!(num_chunks, 2);
    assert_eq!(byte_len, 8);
    assert_eq!(&dest[..], &[1, 0, 0, 0, 2, 0, 0, 0]);
}

#[test]
fn test_fill_via_chunks_with_exact_space() {
    use std::convert::TryInto;

    struct ObservableStruct(u32);
    
    impl Observable for ObservableStruct {
        fn to_le_bytes(&self) -> [u8; core::mem::size_of::<u32>()] {
            self.0.to_le_bytes()
        }
    }

    let src: Vec<ObservableStruct> = vec![ObservableStruct(1)];
    let mut dest: Vec<u8> = vec![0; 4]; // Exact space for one ObservableStruct

    let (num_chunks, byte_len) = fill_via_chunks(&src, &mut dest);

    assert_eq!(num_chunks, 1);
    assert_eq!(byte_len, 4);
    assert_eq!(&dest[..], &[1, 0, 0, 0]);
}

#[test]
fn test_fill_via_chunks_with_zero_dest() {
    use std::convert::TryInto;

    struct ObservableStruct(u32);
    
    impl Observable for ObservableStruct {
        fn to_le_bytes(&self) -> [u8; core::mem::size_of::<u32>()] {
            self.0.to_le_bytes()
        }
    }

    let src: Vec<ObservableStruct> = vec![ObservableStruct(1), ObservableStruct(2)];
    let mut dest: Vec<u8> = vec![0; 0]; // No space in dest

    // Because of the constraints n > 0 and let Some(src) = src.next(), we will pass a single item
    let (num_chunks, byte_len) = fill_via_chunks(&src[..1], &mut dest);

    assert_eq!(num_chunks, 1);
    assert_eq!(byte_len, 0);
}

#[test]
#[should_panic]
fn test_fill_via_chunks_with_insufficient_dest_space() {
    use std::convert::TryInto;

    struct ObservableStruct(u32);
    
    impl Observable for ObservableStruct {
        fn to_le_bytes(&self) -> [u8; core::mem::size_of::<u32>()] {
            self.0.to_le_bytes()
        }
    }

    let src: Vec<ObservableStruct> = vec![ObservableStruct(1)];
    let mut dest: Vec<u8> = vec![0; 2]; // Insufficient space for even one ObservableStruct

    fill_via_chunks(&src, &mut dest); // This should panic
}

