fn next_state(
        &mut self,
        qcur: &mut SparseSet,
        qnext: &mut SparseSet,
        si: StatePtr,
        b: Byte,
    ) -> Option<StatePtr> {
        if si == STATE_DEAD {
            return Some(STATE_DEAD);
        }
        match self.cache.trans.next(si, self.byte_class(b)) {
            STATE_UNKNOWN => self.exec_byte(qcur, qnext, si, b),
            STATE_QUIT => None,
            STATE_DEAD => Some(STATE_DEAD),
            nsi => Some(nsi),
        }
    }