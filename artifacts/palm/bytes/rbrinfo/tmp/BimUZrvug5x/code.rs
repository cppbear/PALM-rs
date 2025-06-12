pub fn copy_from_slice(data: &[u8]) -> Self {
        data.to_vec().into()
    }