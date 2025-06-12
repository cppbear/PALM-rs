// Answer 0

#[test]
fn test_p_valid_case_1() {
    let bernoulli = Bernoulli { p_int: 0 };
    let _ = bernoulli.p();
}

#[test]
fn test_p_valid_case_2() {
    let bernoulli = Bernoulli { p_int: 1 };
    let _ = bernoulli.p();
}

#[test]
fn test_p_valid_case_3() {
    let bernoulli = Bernoulli { p_int: 5000000000000000000 }; // example value < u64::MAX
    let _ = bernoulli.p();
}

#[test]
fn test_p_valid_case_4() {
    let bernoulli = Bernoulli { p_int: u64::MAX - 1 }; // example value < u64::MAX
    let _ = bernoulli.p();
}

#[test]
fn test_p_valid_case_5() {
    let bernoulli = Bernoulli { p_int: 999999999999999999 }; // another example < u64::MAX
    let _ = bernoulli.p();
}

