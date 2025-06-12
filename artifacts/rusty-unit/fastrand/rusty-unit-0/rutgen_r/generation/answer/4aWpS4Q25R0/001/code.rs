// Answer 0

#[test]
fn test_digit_valid_base() {
    assert_eq!(digit(10), '0'); // lowest valid digit for base 10
    assert_eq!(digit(10), '1'); // next valid digit
    assert_eq!(digit(10), '2'); // continuing...
    assert_eq!(digit(10), '3');
    assert_eq!(digit(10), '4');
    assert_eq!(digit(10), '5');
    assert_eq!(digit(10), '6');
    assert_eq!(digit(10), '7');
    assert_eq!(digit(10), '8');
    assert_eq!(digit(10), '9'); // highest valid digit for base 10

    assert_eq!(digit(36), '0'); // lowest valid digit for base 36
    assert_eq!(digit(36), '1'); // next valid digit
    assert_eq!(digit(36), '2'); // continuing...
    assert_eq!(digit(36), '3');
    assert_eq!(digit(36), '4');
    assert_eq!(digit(36), '5');
    assert_eq!(digit(36), '6');
    assert_eq!(digit(36), '7');
    assert_eq!(digit(36), '8');
    assert_eq!(digit(36), '9');
    assert_eq!(digit(36), 'a'); // continuing for base 36
    assert_eq!(digit(36), 'b');
    assert_eq!(digit(36), 'c');
    assert_eq!(digit(36), 'd');
    assert_eq!(digit(36), 'e');
    assert_eq!(digit(36), 'f');
    assert_eq!(digit(36), 'g');
    assert_eq!(digit(36), 'h');
    assert_eq!(digit(36), 'i');
    assert_eq!(digit(36), 'j');
    assert_eq!(digit(36), 'k');
    assert_eq!(digit(36), 'l');
    assert_eq!(digit(36), 'm');
    assert_eq!(digit(36), 'n');
    assert_eq!(digit(36), 'o');
    assert_eq!(digit(36), 'p');
    assert_eq!(digit(36), 'q');
    assert_eq!(digit(36), 'r');
    assert_eq!(digit(36), 's');
    assert_eq!(digit(36), 't');
    assert_eq!(digit(36), 'u');
    assert_eq!(digit(36), 'v');
    assert_eq!(digit(36), 'w');
    assert_eq!(digit(36), 'x');
    assert_eq!(digit(36), 'y');
    assert_eq!(digit(36), 'z'); // highest valid digit for base 36
}

#[should_panic(expected = "zero or greater than 36")]
#[test]
fn test_digit_zero_base() {
    digit(0); // panic expected
}

#[should_panic(expected = "zero or greater than 36")]
#[test]
fn test_digit_exceeding_base() {
    digit(37); // panic expected
}

