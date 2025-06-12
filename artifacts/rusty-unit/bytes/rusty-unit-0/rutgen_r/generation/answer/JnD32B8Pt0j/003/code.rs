// Answer 0

#[derive(Debug)]
struct BytesMut {
    data: *mut u8,
    kind: usize,
}

const KIND_VEC: usize = 1;
const MAX_VEC_POS: usize = 10;
const VEC_POS_OFFSET: usize = 4;
const NOT_VEC_POS_MASK: usize = !0b1111;

impl BytesMut {
    fn new(data: *mut u8, kind: usize) -> Self {
        BytesMut { data, kind }
    }

    fn kind(&self) -> usize {
        self.kind
    }
}

unsafe fn invalid_ptr(val: usize) -> *mut u8 {
    val as *mut u8
}

unsafe fn set_vec_pos(bytes: &mut BytesMut, pos: usize) {
    debug_assert_eq!(bytes.kind(), KIND_VEC);
    debug_assert!(pos <= MAX_VEC_POS);

    bytes.data = invalid_ptr((pos << VEC_POS_OFFSET) | (bytes.data as usize & NOT_VEC_POS_MASK));
}

#[test]
fn test_set_vec_pos_valid() {
    let mut data: usize = 0x1000; // Some initial value
    let mut bytes = BytesMut::new(&mut data as *mut usize as *mut u8, KIND_VEC);
    
    unsafe {
        set_vec_pos(&mut bytes, 5);
        let expected_data = (5 << VEC_POS_OFFSET) | (data & NOT_VEC_POS_MASK);
        assert_eq!(bytes.data as usize, expected_data);
    }
}

#[test]
#[should_panic]
fn test_set_vec_pos_exceeds_max_pos() {
    let mut data: usize = 0x1000; // Some initial value
    let mut bytes = BytesMut::new(&mut data as *mut usize as *mut u8, KIND_VEC);
    
    unsafe {
        set_vec_pos(&mut bytes, MAX_VEC_POS + 1);
    }
}

#[test]
fn test_set_vec_pos_zero() {
    let mut data: usize = 0x1000; // Some initial value
    let mut bytes = BytesMut::new(&mut data as *mut usize as *mut u8, KIND_VEC);
    
    unsafe {
        set_vec_pos(&mut bytes, 0);
        let expected_data = (0 << VEC_POS_OFFSET) | (data & NOT_VEC_POS_MASK);
        assert_eq!(bytes.data as usize, expected_data);
    }
}

#[test]
#[should_panic]
fn test_set_vec_pos_invalid_kind() {
    let mut data: usize = 0x1000; // Some initial value
    let mut bytes = BytesMut::new(&mut data as *mut usize as *mut u8, 0); // Invalid kind
    
    unsafe {
        set_vec_pos(&mut bytes, 5);
    }
}

