pub fn slice_ref(&self, subset: &[u8]) -> Self {
        // Empty slice and empty Bytes may have their pointers reset
        // so explicitly allow empty slice to be a subslice of any slice.
        if subset.is_empty() {
            return Bytes::new();
        }

        let bytes_p = self.as_ptr() as usize;
        let bytes_len = self.len();

        let sub_p = subset.as_ptr() as usize;
        let sub_len = subset.len();

        assert!(
            sub_p >= bytes_p,
            "subset pointer ({:p}) is smaller than self pointer ({:p})",
            subset.as_ptr(),
            self.as_ptr(),
        );
        assert!(
            sub_p + sub_len <= bytes_p + bytes_len,
            "subset is out of bounds: self = ({:p}, {}), subset = ({:p}, {})",
            self.as_ptr(),
            bytes_len,
            subset.as_ptr(),
            sub_len,
        );

        let sub_offset = sub_p - bytes_p;

        self.slice(sub_offset..(sub_offset + sub_len))
    }