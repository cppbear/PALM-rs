// Answer 0

#[test]
fn test_alphabetic() {
    let mut rng = Rng::with_seed(42);
    let generated_char = rng.alphabetic();
    assert!(generated_char.is_ascii_alphabetic());
    assert!(generated_char.is_alphabetic());
}

#[test]
fn test_alphabetic_range() {
    let mut rng = Rng::with_seed(100);
    let mut counts = [0; 52]; // 26 for A-Z and 26 for a-z
    for _ in 0..10000 {
        let generated_char = rng.alphabetic();
        if generated_char.is_ascii() {
            if generated_char.is_uppercase() {
                counts[(generated_char as usize) - ('A' as usize)] += 1;
            } else {
                counts[26 + (generated_char as usize) - ('a' as usize)] += 1;
            }
        }
    }
    assert!(counts.iter().sum::<usize>() == 10000); // Total character count should match
}

