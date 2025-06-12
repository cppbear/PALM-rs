// Answer 0

#[test]
fn test_start_flags_reverse_empty_text() {
    let text: &[u8] = b"";
    let at = 0;
    let fsm = Fsm { /* init fields */ };
    fsm.start_flags_reverse(text, at);
}

#[test]
fn test_start_flags_reverse_one_character_newline() {
    let text: &[u8] = b"\n";
    let at = 0;
    let fsm = Fsm { /* init fields */ };
    fsm.start_flags_reverse(text, at);
}

#[test]
fn test_start_flags_reverse_one_character_other() {
    let text: &[u8] = b"A";
    let at = 0;
    let fsm = Fsm { /* init fields */ };
    fsm.start_flags_reverse(text, at);
}

#[test]
fn test_start_flags_reverse_space_character() {
    let text: &[u8] = b" ";
    let at = 0;
    let fsm = Fsm { /* init fields */ };
    fsm.start_flags_reverse(text, at);
}

#[test]
fn test_start_flags_reverse_last_character() {
    let text: &[u8] = b"A";
    let at = 1;
    let fsm = Fsm { /* init fields */ };
    fsm.start_flags_reverse(text, at);
}

#[test]
fn test_start_flags_reverse_character_word() {
    let text: &[u8] = b"AB";
    let at = 1;
    let fsm = Fsm { /* init fields */ };
    fsm.start_flags_reverse(text, at);
}

