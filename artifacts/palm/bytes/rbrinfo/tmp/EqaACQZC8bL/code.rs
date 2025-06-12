unsafe fn advance_mut(&mut self, mut cnt: usize) {
        let a_rem = self.a.remaining_mut();

        if a_rem != 0 {
            if a_rem >= cnt {
                self.a.advance_mut(cnt);
                return;
            }

            // Consume what is left of a
            self.a.advance_mut(a_rem);

            cnt -= a_rem;
        }

        self.b.advance_mut(cnt);
    }