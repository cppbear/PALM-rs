pub fn case_fold_simple(&mut self) {
        let len = self.ranges.len();
        for i in 0..len {
            let range = self.ranges[i];
            range.case_fold_simple(&mut self.ranges);
        }
        self.canonicalize();
    }