pub(super) fn new(set1: &'a IndexSet<T, S1>, set2: &'a IndexSet<T, S2>) -> Self {
        let diff1 = set1.difference(set2);
        let diff2 = set2.difference(set1);
        Self {
            iter: diff1.chain(diff2),
        }
    }