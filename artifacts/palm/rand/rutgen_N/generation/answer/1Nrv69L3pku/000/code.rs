// Answer 0

#[test]
fn test_sample_random_character() {
    use rand::Rng;
    use rand::thread_rng;

    struct SampleHelper;

    impl SampleHelper {
        fn sample<R: Rng + ?Sized>(rng: &mut R) -> u8 {
            const RANGE: u32 = 26 + 26 + 10;
            const GEN_ASCII_STR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                    abcdefghijklmnopqrstuvwxyz\
                    0123456789";

            loop {
                let var = rng.next_u32() >> (32 - 6);
                if var < RANGE {
                    return GEN_ASCII_STR_CHARSET[var as usize];
                }
            }
        }
    }

    let mut rng = thread_rng();
    let result = SampleHelper::sample(&mut rng);
    assert!(result.is_ascii()); // Check if the result is an ASCII character
    assert!((result >= b'A' && result <= b'Z') || (result >= b'a' && result <= b'z') || (result >= b'0' && result <= b'9')); // Ensure it is from the expected charset
}

#[test]
fn test_sample_random_character_boundary() {
    use rand::Rng;
    use rand::thread_rng;

    struct SampleHelper;

    impl SampleHelper {
        fn sample<R: Rng + ?Sized>(rng: &mut R) -> u8 {
            const RANGE: u32 = 26 + 26 + 10;
            const GEN_ASCII_STR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                    abcdefghijklmnopqrstuvwxyz\
                    0123456789";

            loop {
                let var = rng.next_u32() >> (32 - 6);
                if var < RANGE {
                    return GEN_ASCII_STR_CHARSET[var as usize];
                }
            }
        }
    }

    let mut rng = thread_rng();

    // Testing the boundary by sampling a number of times and ensuring all characters fall in the appropriate ranges
    let mut char_counts = [0; 62]; // 26 + 26 + 10
    let samples = 10000; // Sample 10,000 times

    for _ in 0..samples {
        let result = SampleHelper::sample(&mut rng);
        if result >= b'A' && result <= b'Z' {
            char_counts[(result - b'A') as usize] += 1;
        } else if result >= b'a' && result <= b'z' {
            char_counts[26 + (result - b'a') as usize] += 1;
        } else if result >= b'0' && result <= b'9' {
            char_counts[52 + (result - b'0') as usize] += 1;
        }
    }

    // Check that we have at least a certain minimum count for each character
    for &count in &char_counts {
        assert!(count > 0); // Ensure each character has been sampled at least once
    }
}

