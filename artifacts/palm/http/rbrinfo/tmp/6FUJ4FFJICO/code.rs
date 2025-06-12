fn size_hint(&self) -> (usize, Option<usize>) {
        // At least this many names... It's unknown if the user wants
        // to count the extra_values on top.
        //
        // For instance, extending a new `HeaderMap` wouldn't need to
        // reserve the upper-bound in `entries`, only the lower-bound.
        let lower = self.len - self.idx;
        let upper = unsafe { (*self.extra_values).len() } + lower;
        (lower, Some(upper))
    }