use std::str::FromStr;

use rust_triangle_star::prelude::{accept, is_language_valid, is_n, is_number, MyBool};

#[test]
fn validate_language() {
    assert!(!is_language_valid("gg"));
    assert!(is_language_valid("en"));
    assert!(is_language_valid("es"));
}

#[test]
fn validate_n() {
    assert!(!is_n("0"));
    assert!(!is_n("-10"));
    assert!(is_n("1"));
    assert!(is_n("20"));
}

#[test]
fn validate_number() {
    assert!(!is_number("g"));
    assert!(!is_number(""));
    assert!(!is_number(" "));
    assert!(is_number("0"));
    assert!(is_number("10"));
    assert!(is_number("-10"));
}

#[test]
fn validate_cancel_acept() {
    assert!(!accept(""));
    assert!(accept("     "));
    assert_eq!(!accept("n "), false);
    assert_eq!(!accept("N"), false);
    assert_eq!(!accept("NO"), false);
    assert_eq!(!accept("No"), false);
    assert_eq!(!accept("YES"), false);
    assert!(accept(" y"));
    assert!(accept("y  "));
    assert!(accept("Y"));
    assert!(accept("yes"));
    assert!(accept("Yes"));
}
