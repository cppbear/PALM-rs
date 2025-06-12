// Answer 0

#[test]
fn test_repeat_range_literals_min_zero() {
    let e = Hir::empty(); // Assume Hir::empty creates an empty Hir
    let min = 0;
    let max = Some(3);
    let greedy = true;
    let mut lits = Literals::empty(); // Assume Literals::empty creates an empty Literals

    let mut output = Vec::new();
    let f = |hir: &Hir, literals: &mut Literals| {
        output.push(hir.clone());
    };

    repeat_range_literals(&e, min, max, greedy, &mut lits, f);
    
    assert_eq!(output.len(), 1); // We expect f to be called once since min is 0
}

#[test]
fn test_repeat_range_literals_min_greater_than_zero() {
    let e = Hir::empty(); // Assume Hir::empty creates an empty Hir
    let min = 2; // min > 0
    let max = Some(5); // max is a bound greater than min
    let greedy = false;
    let mut lits = Literals::empty(); // Assume Literals::empty creates an empty Literals
    lits.set_limit_size(5);  // Assume setting the limit size allows for multiple repetitions

    let mut output = Vec::new();
    let f = |hir: &Hir, literals: &mut Literals| {
        output.push(hir.clone());
    };

    repeat_range_literals(&e, min, max, greedy, &mut lits, f);
    
    assert_eq!(output.len(), 2); // We expect f to be called twice since min is 2
}

#[test]
fn test_repeat_range_literals_n_equals_min() {
    let e = Hir::empty(); // Assume Hir::empty creates an empty Hir
    let min = 3; // min > 0
    let max = Some(5); // max bounds the range
    let greedy = true;
    let mut lits = Literals::empty(); // Assume Literals::empty creates an empty Literals
    lits.set_limit_size(3); // Set to 3 to match min, allowing for no more than min repetitions

    let mut output = Vec::new();
    let f = |hir: &Hir, literals: &mut Literals| {
        output.push(hir.clone());
    };

    repeat_range_literals(&e, min, max, greedy, &mut lits, f);
    
    assert_eq!(output.len(), 3); // Expect f to be called 3 times since n (3) == min
}

#[test]
fn test_repeat_range_literals_contains_empty_false() {
    let e = Hir::literal(Literal::new(b'a')); // Assume we create a literal for non-empty
    let min = 1; // min > 0
    let max = Some(3); // max is greater than min
    let greedy = true;
    let mut lits = Literals::empty(); // Assume Literals::empty creates an empty Literals
    lits.add(e.clone()); // Adding a non-empty literal, so contains_empty is false

    let mut output = Vec::new();
    let f = |hir: &Hir, literals: &mut Literals| {
        output.push(hir.clone());
    };

    repeat_range_literals(&e, min, max, greedy, &mut lits, f);
    
    assert!(!lits.contains_empty()); // Ensure contains_empty is false
}

#[test]
fn test_repeat_range_literals_max_map_or_false() {
    let e = Hir::empty(); // Assume Hir::empty creates an empty Hir
    let min = 2; // min > 0
    let max = Some(2); // Set max to equal min
    let greedy = false;
    let mut lits = Literals::empty(); // Assume Literals::empty creates an empty Literals
    lits.set_limit_size(3); // Set limit size larger than min

    let mut output = Vec::new();
    let f = |hir: &Hir, literals: &mut Literals| {
        output.push(hir.clone());
    };

    repeat_range_literals(&e, min, max, greedy, &mut lits, f);
    
    assert_eq!(output.len(), 2); // We expect f to be called twice since min equals max
}

