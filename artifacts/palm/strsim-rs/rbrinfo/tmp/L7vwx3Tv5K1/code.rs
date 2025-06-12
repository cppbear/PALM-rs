pub fn osa_distance(a: &str, b: &str) -> usize {
    let b_len = b.chars().count();
    // 0..=b_len behaves like 0..b_len.saturating_add(1) which could be a different size
    // this leads to significantly worse code gen when swapping the vectors below
    let mut prev_two_distances: Vec<usize> = (0..b_len + 1).collect();
    let mut prev_distances: Vec<usize> = (0..b_len + 1).collect();
    let mut curr_distances: Vec<usize> = vec![0; b_len + 1];

    let mut prev_a_char = char::MAX;
    let mut prev_b_char = char::MAX;

    for (i, a_char) in a.chars().enumerate() {
        curr_distances[0] = i + 1;

        for (j, b_char) in b.chars().enumerate() {
            let cost = usize::from(a_char != b_char);
            curr_distances[j + 1] = min(
                curr_distances[j] + 1,
                min(prev_distances[j + 1] + 1, prev_distances[j] + cost),
            );
            if i > 0 && j > 0 && a_char != b_char && a_char == prev_b_char && b_char == prev_a_char
            {
                curr_distances[j + 1] = min(curr_distances[j + 1], prev_two_distances[j - 1] + 1);
            }

            prev_b_char = b_char;
        }

        mem::swap(&mut prev_two_distances, &mut prev_distances);
        mem::swap(&mut prev_distances, &mut curr_distances);
        prev_a_char = a_char;
    }

    // access prev_distances instead of curr_distances since we swapped
    // them above. In case a is empty this would still contain the correct value
    // from initializing the last element to b_len
    prev_distances[b_len]
}