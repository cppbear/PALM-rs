// Answer 0

#[test]
fn test_is_word_character_lowercase() {
    is_word_character('a');
    is_word_character('b');
    is_word_character('c');
    is_word_character('d');
    is_word_character('e');
    is_word_character('f');
    is_word_character('g');
    is_word_character('h');
    is_word_character('i');
    is_word_character('j');
    is_word_character('k');
    is_word_character('l');
    is_word_character('m');
    is_word_character('n');
    is_word_character('o');
    is_word_character('p');
    is_word_character('q');
    is_word_character('r');
    is_word_character('s');
    is_word_character('t');
    is_word_character('u');
    is_word_character('v');
    is_word_character('w');
    is_word_character('x');
    is_word_character('y');
    is_word_character('z');
}

#[test]
fn test_is_word_character_uppercase() {
    is_word_character('A');
    is_word_character('B');
    is_word_character('C');
    is_word_character('D');
    is_word_character('E');
    is_word_character('F');
    is_word_character('G');
    is_word_character('H');
    is_word_character('I');
    is_word_character('J');
    is_word_character('K');
    is_word_character('L');
    is_word_character('M');
    is_word_character('N');
    is_word_character('O');
    is_word_character('P');
    is_word_character('Q');
    is_word_character('R');
    is_word_character('S');
    is_word_character('T');
    is_word_character('U');
    is_word_character('V');
    is_word_character('W');
    is_word_character('X');
    is_word_character('Y');
    is_word_character('Z');
}

#[test]
fn test_is_word_character_digits_and_underscore() {
    is_word_character('0');
    is_word_character('1');
    is_word_character('2');
    is_word_character('3');
    is_word_character('4');
    is_word_character('5');
    is_word_character('6');
    is_word_character('7');
    is_word_character('8');
    is_word_character('9');
    is_word_character('_');
}

#[test]
fn test_is_word_character_special_chars() {
    is_word_character('\0');
    is_word_character(' '); // space, should return false
    is_word_character('!'); // exclamation mark, should return false
    is_word_character('@'); // at sign, should return false
    is_word_character('#'); // hash, should return false
    is_word_character('$'); // dollar sign, should return false
    is_word_character('%'); // percent sign, should return false
    is_word_character('^'); // caret, should return false
    is_word_character('&'); // ampersand, should return false
    is_word_character('*'); // asterisk, should return false
    is_word_character('('); // left parenthesis, should return false
    is_word_character(')'); // right parenthesis, should return false
    is_word_character('-'); // hyphen, should return false
    is_word_character('+'); // plus, should return false
    is_word_character('='); // equals, should return false
}

