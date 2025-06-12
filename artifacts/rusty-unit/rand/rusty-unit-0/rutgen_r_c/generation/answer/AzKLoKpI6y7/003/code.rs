// Answer 0

#[test]
fn test_partial_shuffle_large_slice() {
    struct TestRng;

    impl Rng for TestRng {
        // Implement required methods for Rng trait (not shown for brevity)
    }

    let mut rng = TestRng;
    let len = (u32::MAX as usize);
    let mut slice: Vec<u32> = (0..len).map(|x| x as u32).collect();
    let amount = 1;

    let (part1, part2) = slice.partial_shuffle(&mut rng, amount);

    assert_eq!(part1.len(), amount);
    assert_eq!(part2.len(), len - amount);
    assert!(part1.iter().all(|&x| x < len as u32));
    assert!(part2.iter().all(|&x| x < len as u32));
}

#[test]
fn test_partial_shuffle_empty_slice() {
    struct TestRng;

    impl Rng for TestRng {
        // Implement required methods for Rng trait (not shown for brevity)
    }

    let mut rng = TestRng;
    let mut slice: Vec<u32> = Vec::new();
    let amount = 0;

    let (part1, part2) = slice.partial_shuffle(&mut rng, amount);

    assert_eq!(part1.len(), amount);
    assert_eq!(part2.len(), 0);
}

#[test]
#[should_panic]
fn test_partial_shuffle_invalid_amount() {
    struct TestRng;

    impl Rng for TestRng {
        // Implement required methods for Rng trait (not shown for brevity)
    }

    let mut rng = TestRng;
    let slice: &mut [u32] = &mut [];
    let amount = 1; // Invalid since the slice is empty

    let _ = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_one_element() {
    struct TestRng;

    impl Rng for TestRng {
        // Implement required methods for Rng trait (not shown for brevity)
    }

    let mut rng = TestRng;
    let mut slice: Vec<u32> = vec![42];
    let amount = 1;

    let (part1, part2) = slice.partial_shuffle(&mut rng, amount);

    assert_eq!(part1.len(), amount);
    assert_eq!(part2.len(), 0);
    assert_eq!(part1[0], 42);
}

