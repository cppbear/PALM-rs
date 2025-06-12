// Answer 0

#[test]
fn test_sorensen_dice_case_1() {
    let a = "ab";
    let b = "cd";
    sorensen_dice(a, b);
}

#[test]
fn test_sorensen_dice_case_2() {
    let a = "ab";
    let b = "ac";
    sorensen_dice(a, b);
}

#[test]
fn test_sorensen_dice_case_3() {
    let a = "ab";
    let b = "bc";
    sorensen_dice(a, b);
}

#[test]
fn test_sorensen_dice_case_4() {
    let a = "ab";
    let b = "ab";
    sorensen_dice(a, b);
}

#[test]
fn test_sorensen_dice_case_5() {
    let a = "ab";
    let b = "aa";
    sorensen_dice(a, b);
}

#[test]
fn test_sorensen_dice_case_6() {
    let a = "ab";
    let b = "ba";
    sorensen_dice(a, b);
}

