pub fn generic_jaro<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> f64
where
    &'a Iter1: IntoIterator<Item = Elem1>,
    &'b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>,
{
    let a_len = a.into_iter().count();
    let b_len = b.into_iter().count();

    if a_len == 0 && b_len == 0 {
        return 1.0;
    } else if a_len == 0 || b_len == 0 {
        return 0.0;
    }

    let mut search_range = max(a_len, b_len) / 2;
    search_range = search_range.saturating_sub(1);

    // combine memory allocations to reduce runtime
    let mut flags_memory = vec![false; a_len + b_len];
    let (a_flags, b_flags) = flags_memory.split_at_mut(a_len);

    let mut matches = 0_usize;

    for (i, a_elem) in a.into_iter().enumerate() {
        // prevent integer wrapping
        let min_bound = if i > search_range {
            i - search_range
        } else {
            0
        };

        let max_bound = min(b_len, i + search_range + 1);

        for (j, b_elem) in b.into_iter().enumerate().take(max_bound) {
            if min_bound <= j && a_elem == b_elem && !b_flags[j] {
                a_flags[i] = true;
                b_flags[j] = true;
                matches += 1;
                break;
            }
        }
    }

    let mut transpositions = 0_usize;
    if matches != 0 {
        let mut b_iter = b_flags.iter().zip(b);
        for (a_flag, ch1) in a_flags.iter().zip(a) {
            if *a_flag {
                loop {
                    if let Some((b_flag, ch2)) = b_iter.next() {
                        if !*b_flag {
                            continue;
                        }

                        if ch1 != ch2 {
                            transpositions += 1;
                        }
                        break;
                    }
                }
            }
        }
    }
    transpositions /= 2;

    if matches == 0 {
        0.0
    } else {
        ((matches as f64 / a_len as f64)
            + (matches as f64 / b_len as f64)
            + ((matches - transpositions) as f64 / matches as f64))
            / 3.0
    }
}