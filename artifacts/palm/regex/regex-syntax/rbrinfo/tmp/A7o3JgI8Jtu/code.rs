pub fn case_fold_simple(&mut self) {
        match *self {
            Class::Unicode(ref mut x) => x.case_fold_simple(),
            Class::Bytes(ref mut x) => x.case_fold_simple(),
        }
    }