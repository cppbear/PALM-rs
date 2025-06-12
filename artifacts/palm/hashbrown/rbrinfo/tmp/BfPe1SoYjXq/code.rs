unsafe fn new(ctrl: *const u8, data: Bucket<T>, len: usize) -> Self {
        debug_assert_ne!(len, 0);
        debug_assert_eq!(ctrl as usize % Group::WIDTH, 0);
        // SAFETY: The caller must uphold the safety rules for the [`RawIterRange::new`]
        let end = ctrl.add(len);

        // Load the first group and advance ctrl to point to the next group
        // SAFETY: The caller must uphold the safety rules for the [`RawIterRange::new`]
        let current_group = Group::load_aligned(ctrl.cast()).match_full();
        let next_ctrl = ctrl.add(Group::WIDTH);

        Self {
            current_group: current_group.into_iter(),
            data,
            next_ctrl,
            end,
        }
    }