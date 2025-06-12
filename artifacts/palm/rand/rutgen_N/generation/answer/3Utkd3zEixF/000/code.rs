// Answer 0

#[test]
fn test_new_with_zero() {
    struct DummyRng;

    let rng = DummyRng;
    let n = 0;

    let roller = new(rng, n);
    assert_eq!(roller.chunk_remaining, 1);
    assert_eq!(roller.chunk, 0);
}

#[test]
fn test_new_with_non_zero() {
    struct DummyRng;

    let rng = DummyRng;
    let n = 5;

    let roller = new(rng, n);
    assert_eq!(roller.chunk_remaining, 0);
    assert_eq!(roller.chunk, 0);
}

