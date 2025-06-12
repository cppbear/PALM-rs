fn probe_seq(&self, hash: u64) -> ProbeSeq {
        ProbeSeq {
            // This is the same as `hash as usize % self.buckets()` because the number
            // of buckets is a power of two, and `self.bucket_mask = self.buckets() - 1`.
            pos: h1(hash) & self.bucket_mask,
            stride: 0,
        }
    }