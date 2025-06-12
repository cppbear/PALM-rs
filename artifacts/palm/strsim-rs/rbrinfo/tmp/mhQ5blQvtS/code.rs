fn damerau_levenshtein_impl<Iter1, Iter2>(s1: Iter1, len1: usize, s2: Iter2, len2: usize) -> usize
where
    Iter1: Iterator<Item = char> + Clone,
    Iter2: Iterator<Item = char> + Clone,
{
    // The implementations is based on the paper
    // `Linear space string correction algorithm using the Damerau-Levenshtein distance`
    // from Chunchun Zhao and Sartaj Sahni
    //
    // It has a runtime complexity of `O(N*M)` and a memory usage of `O(N+M)`.
    let max_val = max(len1, len2) as isize + 1;

    let mut last_row_id = HybridGrowingHashmapChar::<RowId>::default();

    let size = len2 + 2;
    let mut fr = vec![max_val; size];
    let mut r1 = vec![max_val; size];
    let mut r: Vec<isize> = (max_val..max_val + 1)
        .chain(0..(size - 1) as isize)
        .collect();

    for (i, ch1) in s1.enumerate().map(|(i, ch1)| (i + 1, ch1)) {
        mem::swap(&mut r, &mut r1);
        let mut last_col_id: isize = -1;
        let mut last_i2l1 = r[1];
        r[1] = i as isize;
        let mut t = max_val;

        for (j, ch2) in s2.clone().enumerate().map(|(j, ch2)| (j + 1, ch2)) {
            let diag = r1[j] + isize::from(ch1 != ch2);
            let left = r[j] + 1;
            let up = r1[j + 1] + 1;
            let mut temp = min(diag, min(left, up));

            if ch1 == ch2 {
                last_col_id = j as isize; // last occurence of s1_i
                fr[j + 1] = r1[j - 1]; // save H_k-1,j-2
                t = last_i2l1; // save H_i-2,l-1
            } else {
                let k = last_row_id.get(ch2).val;
                let l = last_col_id;

                if j as isize - l == 1 {
                    let transpose = fr[j + 1] + (i as isize - k);
                    temp = min(temp, transpose);
                } else if i as isize - k == 1 {
                    let transpose = t + (j as isize - l);
                    temp = min(temp, transpose);
                }
            }

            last_i2l1 = r[j + 1];
            r[j + 1] = temp;
        }
        last_row_id.get_mut(ch1).val = i as isize;
    }

    r[len2 + 1] as usize
}