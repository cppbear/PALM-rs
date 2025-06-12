// Answer 0

#[test]
#[should_panic]
fn test_literal_with_ascii_byte() {
    let lit = Literal::Byte(0x7F);
    literal(lit);
} 

#[test]
fn test_literal_with_valid_unicode() {
    let lit = Literal::Unicode('a');
    let result = literal(lit);
}

#[test]
fn test_literal_with_valid_non_ascii_byte() {
    let lit = Literal::Byte(0x80);
    let result = literal(lit);
} 

#[test]
fn test_literal_with_another_valid_non_ascii_byte() {
    let lit = Literal::Byte(0xFF);
    let result = literal(lit);
} 

#[test]
fn test_literal_with_unicodes() {
    let lit1 = Literal::Unicode('あ');
    let result1 = literal(lit1);
    
    let lit2 = Literal::Unicode('中');
    let result2 = literal(lit2);
}

