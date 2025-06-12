fn clear(&mut self) {
        // Reset the job memory so that we start fresh.
        self.m.jobs.clear();

        // Now we need to clear the bit state set.
        // We do this by figuring out how much space we need to keep track
        // of the states we've visited.
        // Then we reset all existing allocated space to 0.
        // Finally, we request more space if we need it.
        //
        // This is all a little circuitous, but doing this unsafely
        // doesn't seem to have a measurable impact on performance.
        // (Probably because backtracking is limited to such small
        // inputs/regexes in the first place.)
        let visited_len =
            (self.prog.len() * (self.input.len() + 1) + BIT_SIZE - 1)
            /
            BIT_SIZE;
        self.m.visited.truncate(visited_len);
        for v in &mut self.m.visited {
            *v = 0;
        }
        if visited_len > self.m.visited.len() {
            let len = self.m.visited.len();
            self.m.visited.reserve_exact(visited_len - len);
            for _ in 0..(visited_len - len) {
                self.m.visited.push(0);
            }
        }
    }