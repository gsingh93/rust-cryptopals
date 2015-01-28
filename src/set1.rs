extern crate "rustc-serialize" as rustc_serialize;

use self::rustc_serialize::base64::{ToBase64, STANDARD};
use self::rustc_serialize::hex::{FromHex, ToHex};

fn chal1(s: &str) -> String {
    s.from_hex().unwrap().to_base64(STANDARD)
}

fn chal2(a: &str, b: &str) -> String {
    let b1 = a.from_hex().unwrap();
    let b2 = b.from_hex().unwrap();
    let mut res = String::new();
    for (i, j) in b1.iter().zip(b2.iter()) {
        res.push((i ^ j) as char);
    }
    res.as_bytes().to_hex()
}

#[test]
fn test_chal1() {
    let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(&*chal1(s), expected);
}

#[test]
fn test_chal2() {
    let a = "1c0111001f010100061a024b53535009181c";
    let b = "686974207468652062756c6c277320657965";
    let expected = "746865206b696420646f6e277420706c6179";
    assert_eq!(chal2(a, b), expected);
}
