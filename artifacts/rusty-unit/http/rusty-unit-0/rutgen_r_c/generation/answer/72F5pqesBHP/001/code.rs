// Answer 0

#[test]
fn test_set_red_when_is_yellow() {
    // Arrange
    let mut danger = Danger::Yellow;

    // Act
    danger.set_red();

    // Assert
    if let Danger::Red(_) = danger {
        // The state has been correctly changed to Red.
    } else {
        panic!("Expected Danger to be Red after calling set_red");
    }
}

#[test]
#[should_panic]
fn test_set_red_when_not_yellow() {
    // Arrange
    let mut danger = Danger::Green;

    // Act
    danger.set_red(); // This should panic because is_yellow() is false

    // Since we're expecting a panic, no assertion is needed.
}

