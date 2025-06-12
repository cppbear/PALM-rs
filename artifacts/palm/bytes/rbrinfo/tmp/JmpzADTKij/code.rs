pub(crate) fn from_vec(vec: Vec<u8>) -> BytesMut {
        let mut vec = ManuallyDrop::new(vec);
        let ptr = vptr(vec.as_mut_ptr());
        let len = vec.len();
        let cap = vec.capacity();

        let original_capacity_repr = original_capacity_to_repr(cap);
        let data = (original_capacity_repr << ORIGINAL_CAPACITY_OFFSET) | KIND_VEC;

        BytesMut {
            ptr,
            len,
            cap,
            data: invalid_ptr(data),
        }
    }