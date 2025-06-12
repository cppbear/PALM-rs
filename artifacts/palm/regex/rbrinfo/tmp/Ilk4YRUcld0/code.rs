pub fn expand(&self, replacement: &[u8], dst: &mut Vec<u8>) {
        expand_bytes(self, replacement, dst)
    }