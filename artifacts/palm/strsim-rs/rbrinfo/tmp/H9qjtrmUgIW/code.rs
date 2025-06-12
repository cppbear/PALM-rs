pub fn sorensen_dice(a: &str, b: &str) -> f64 {
    // implementation guided by
    // https://github.com/aceakash/string-similarity/blob/f83ba3cd7bae874c20c429774e911ae8cff8bced/src/index.js#L6

    let a: String = a.chars().filter(|&x| !char::is_whitespace(x)).collect();
    let b: String = b.chars().filter(|&x| !char::is_whitespace(x)).collect();

    if a == b {
        return 1.0;
    }

    if a.len() < 2 || b.len() < 2 {
        return 0.0;
    }

    let mut a_bigrams: HashMap<(char, char), usize> = HashMap::new();

    for bigram in bigrams(&a) {
        *a_bigrams.entry(bigram).or_insert(0) += 1;
    }

    let mut intersection_size = 0_usize;

    for bigram in bigrams(&b) {
        a_bigrams.entry(bigram).and_modify(|bi| {
            if *bi > 0 {
                *bi -= 1;
                intersection_size += 1;
            }
        });
    }

    (2 * intersection_size) as f64 / (a.len() + b.len() - 2) as f64
}