extern crate cminus;

use cminus::*;

#[test]
fn can_smile() {
    assert_eq!(smile(), ":)");
}

#[test]
fn can_frown() {
    assert_eq!(frown(), ":(");
}

#[test]
fn can_angry() {
    assert_eq!(angry(), ">:(");
}

#[test]
fn string_representation() {
    assert_eq!(which(&smile()), "Smile");
}