unsafe fn new(table: &RawTableInner, hash: u64) -> Self {
        let tag_hash = Tag::full(hash);
        let probe_seq = table.probe_seq(hash);
        let group = Group::load(table.ctrl(probe_seq.pos));
        let bitmask = group.match_tag(tag_hash).into_iter();

        RawIterHashInner {
            bucket_mask: table.bucket_mask,
            ctrl: table.ctrl,
            tag_hash,
            probe_seq,
            group,
            bitmask,
        }
    }