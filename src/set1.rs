extern crate "rustc-serialize" as rustc_serialize;

use self::rustc_serialize::base64::{ToBase64, STANDARD};
use self::rustc_serialize::hex::FromHex;

fn chal1(s: &str) -> String {
    s.from_hex().unwrap().to_base64(STANDARD)
}

#[test]
fn test_chal1() {
    let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(&*chal1(s), expected);
}
