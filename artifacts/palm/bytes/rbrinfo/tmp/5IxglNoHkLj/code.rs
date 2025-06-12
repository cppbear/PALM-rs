pub fn zeroed(len: usize) -> BytesMut {
        BytesMut::from_vec(vec![0; len])
    }