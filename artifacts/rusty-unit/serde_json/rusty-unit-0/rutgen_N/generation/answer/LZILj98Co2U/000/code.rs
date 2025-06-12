// Answer 0

#[derive(Debug)]
struct SliceRead<'a> {
    slice: &'a [u8],
    index: usize,
    #[cfg(feature = "raw_value")]
    raw_buffering_start_index: usize,
}

impl<'a> SliceRead<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        SliceRead {
            slice,
            index: 0,
            #[cfg(feature = "raw_value")]
            raw_buffering_start_index: 0,
        }
    }
}

#[test]
fn test_slice_read_initialization() {
    let data: &[u8] = b"test data";
    let slice_read = SliceRead::new(data);

    assert_eq!(slice_read.slice, data);
    assert_eq!(slice_read.index, 0);
    #[cfg(feature = "raw_value")]
    {
        assert_eq!(slice_read.raw_buffering_start_index, 0);
    }
}

#[test]
fn test_empty_slice() {
    let data: &[u8] = b"";
    let slice_read = SliceRead::new(data);

    assert_eq!(slice_read.slice, data);
    assert_eq!(slice_read.index, 0);
    #[cfg(feature = "raw_value")]
    {
        assert_eq!(slice_read.raw_buffering_start_index, 0);
    }
}

