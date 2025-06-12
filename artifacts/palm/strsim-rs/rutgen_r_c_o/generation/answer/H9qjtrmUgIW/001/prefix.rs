// Answer 0

#[test]
fn test_sorensen_dice_equal_strings() {
    let result1 = sorensen_dice("test", "test");
    let result2 = sorensen_dice("Rust", "Rust");
    let result3 = sorensen_dice("Hello, World!", "Hello, World!");
    let result4 = sorensen_dice("abcdef", "abcdef");
    let result5 = sorensen_dice("12345", "12345");
    let result6 = sorensen_dice("longer test string", "longer test string");
    let result7 = sorensen_dice("mixedCase", "mixedCase");
    let result8 = sorensen_dice("    spaced    ", "    spaced    ");
    let result9 = sorensen_dice("!@#$%^&*()", "!@#$%^&*()");
    let result10 = sorensen_dice("GivenString", "GivenString");
}

