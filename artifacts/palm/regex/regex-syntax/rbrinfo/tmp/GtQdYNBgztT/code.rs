fn case_fold_simple(&self, ranges: &mut Vec<ClassBytesRange>) {
        if !ClassBytesRange::new(b'a', b'z').is_intersection_empty(self) {
            let lower = cmp::max(self.start, b'a');
            let upper = cmp::min(self.end, b'z');
            ranges.push(ClassBytesRange::new(lower - 32, upper - 32));
        }
        if !ClassBytesRange::new(b'A', b'Z').is_intersection_empty(self) {
            let lower = cmp::max(self.start, b'A');
            let upper = cmp::min(self.end, b'Z');
            ranges.push(ClassBytesRange::new(lower + 32, upper + 32));
        }
    }