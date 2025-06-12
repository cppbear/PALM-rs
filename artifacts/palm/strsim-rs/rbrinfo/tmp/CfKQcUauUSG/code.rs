pub fn generic_damerau_levenshtein<Elem>(a_elems: &[Elem], b_elems: &[Elem]) -> usize
where
    Elem: Eq + Hash + Clone,
{
    let a_len = a_elems.len();
    let b_len = b_elems.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let width = a_len + 2;
    let mut distances = vec![0; (a_len + 2) * (b_len + 2)];
    let max_distance = a_len + b_len;
    distances[0] = max_distance;

    for i in 0..(a_len + 1) {
        distances[flat_index(i + 1, 0, width)] = max_distance;
        distances[flat_index(i + 1, 1, width)] = i;
    }

    for j in 0..(b_len + 1) {
        distances[flat_index(0, j + 1, width)] = max_distance;
        distances[flat_index(1, j + 1, width)] = j;
    }

    let mut elems: HashMap<Elem, usize> = HashMap::with_capacity(64);

    for i in 1..(a_len + 1) {
        let mut db = 0;

        for j in 1..(b_len + 1) {
            let k = match elems.get(&b_elems[j - 1]) {
                Some(&value) => value,
                None => 0,
            };

            let insertion_cost = distances[flat_index(i, j + 1, width)] + 1;
            let deletion_cost = distances[flat_index(i + 1, j, width)] + 1;
            let transposition_cost =
                distances[flat_index(k, db, width)] + (i - k - 1) + 1 + (j - db - 1);

            let mut substitution_cost = distances[flat_index(i, j, width)] + 1;
            if a_elems[i - 1] == b_elems[j - 1] {
                db = j;
                substitution_cost -= 1;
            }

            distances[flat_index(i + 1, j + 1, width)] = min(
                substitution_cost,
                min(insertion_cost, min(deletion_cost, transposition_cost)),
            );
        }

        elems.insert(a_elems[i - 1].clone(), i);
    }

    distances[flat_index(a_len + 1, b_len + 1, width)]
}