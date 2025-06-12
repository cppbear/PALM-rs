fn advance(&mut self, mut cnt: usize) {
        let a_rem = self.a.remaining();

        if a_rem != 0 {
            if a_rem >= cnt {
                self.a.advance(cnt);
                return;
            }

            // Consume what is left of a
            self.a.advance(a_rem);

            cnt -= a_rem;
        }

        self.b.advance(cnt);
    }