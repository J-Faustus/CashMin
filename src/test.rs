use crate::math::*;

#[test]
fn test_gcd() {
    assert_eq!(gcd(5, 10), 5);
    assert_eq!(gcd(9, 6), 3);
    assert_eq!(gcd(9, 72), 9);
    assert_eq!(gcd(18, 16), 2);
    assert_eq!(gcd(105, 30), 15);
}

#[test]
fn test_gcd_vec() {
    assert_eq!(gcd_vec(&vec![1]).expect(""), 1);
    assert_eq!(gcd_vec(&vec![1, 2]).expect(""), 1);
    assert_eq!(gcd_vec(&vec![1, 2, 3]).expect(""), 1);
    assert_eq!(gcd_vec(&vec![6, 9, 12]).expect(""), 3);
    assert_eq!(gcd_vec(&vec![100, 20, 36]).expect(""), 4);
}