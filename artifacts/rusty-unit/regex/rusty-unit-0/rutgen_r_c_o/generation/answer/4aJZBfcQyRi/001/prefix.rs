// Answer 0

#[test]
fn test_start_flags_reverse_at_zero_end() {
    let text = [b'a', b'b', b'\n'];
    let at = 0;
    let fsm = Fsm { /* initialize Fsm struct here */ };
    fsm.start_flags_reverse(&text, at);
}

#[test]
fn test_start_flags_reverse_at_end() {
    let text = [b'a', b'b', b'c', b'd'];
    let at = 4; // at == text.len()
    let fsm = Fsm { /* initialize Fsm struct here */ };
    fsm.start_flags_reverse(&text, at);
}

#[test]
fn test_start_flags_reverse_at_one() {
    let text = [b'a', b'b', b'c'];
    let at = 1; // at > 0
    let fsm = Fsm { /* initialize Fsm struct here */ };
    fsm.start_flags_reverse(&text, at);
} 

#[test]
fn test_start_flags_reverse_at_one_with_word() {
    let text = [b'a', b'b', b'c'];
    let at = 1; // at > 0 and at < text.len()
    let fsm = Fsm { /* initialize Fsm struct here */ };
    fsm.start_flags_reverse(&text, at);
}

#[test]
fn test_start_flags_reverse_at_two_word_last() {
    let text = [b'a', b'b', b'c'];
    let at = 2; // is_word_last is true
    let fsm = Fsm { /* initialize Fsm struct here */ };
    fsm.start_flags_reverse(&text, at);
}

