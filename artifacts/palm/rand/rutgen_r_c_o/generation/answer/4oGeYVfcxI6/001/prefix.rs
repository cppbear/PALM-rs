// Answer 0

#[test]
fn test_p_always_true() {
    let bernoulli = Bernoulli { p_int: ALWAYS_TRUE };
    let result = bernoulli.p();
}

#[test]
fn test_p_edge_case_max_u64() {
    let bernoulli = Bernoulli { p_int: 18446744073709551615 };
    let result = bernoulli.p();
}

