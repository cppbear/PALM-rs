fn find_insert_slot_in_group(&self, group: &Group, probe_seq: &ProbeSeq) -> Option<usize> {
        let bit = group.match_empty_or_deleted().lowest_set_bit();

        if likely(bit.is_some()) {
            // This is the same as `(probe_seq.pos + bit) % self.buckets()` because the number
            // of buckets is a power of two, and `self.bucket_mask = self.buckets() - 1`.
            Some((probe_seq.pos + bit.unwrap()) & self.bucket_mask)
        } else {
            None
        }
    }