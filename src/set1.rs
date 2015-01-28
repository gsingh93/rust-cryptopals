extern crate "rustc-serialize" as rustc_serialize;

use self::rustc_serialize::base64::{ToBase64, STANDARD};
use self::rustc_serialize::hex::{FromHex, ToHex};
use std::io::fs::File;

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

fn chal3(m: &str) -> Vec<(u8, String)> {
    fn is_english(v: &[u8]) -> bool {
        v.iter().all(|b| {
            let c = *b as char;
            c.is_alphabetic() || c.is_whitespace() || c.is_numeric() || ['.', '!', ',', '\''].contains(&c)
        })
    }

    let mut res = Vec::new();
    let b = m.from_hex().unwrap();
    for i in 0u16..256 {
        let mut v = Vec::new();
        for byte in b.iter() {
            v.push(byte ^ (i as u8));
        }
        if is_english(&*v) {
            res.push((i as u8, String::from_utf8(v).unwrap().as_bytes().to_hex()));
        }
    }
    res
}

fn chal4(filename: &str) -> Vec<(u8, String)> {
    let mut f = File::open(&Path::new(filename));
    let tmp = f.read_to_string().unwrap();
    let v: Vec<&str> = tmp.split('\n').collect();
    let mut res = Vec::new();
    for s in v.iter() {
        res.append(&mut chal3(*s))
    }
    res
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

#[test]
fn test_chal3() {
    let s = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let expected = "436f6f6b696e67204d432773206c696b65206120706f756e64206f66206261636f6e";
    let l = chal3(s);
    assert_eq!(l.len(), 1);
    let (ref k, ref m) = l[0];
    assert_eq!(*m, expected);
}

#[test]
fn test_chal4() {
    let expected = "4e6f77207468617420746865207061727479206973206a756d70696e670a";
    let l = chal4("src/4.txt");
    assert_eq!(l.len(), 1);
    let (ref k, ref m) = l[0];
    assert_eq!(*m, expected);
}
