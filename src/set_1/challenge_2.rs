/*
    https://cryptopals.com/sets/1/challenges/2

    Fixed XOR

    Write a function that takes two equal-length buffers and produces their XOR combination.
    If your function works properly, then when you feed it the string:

    1c0111001f010100061a024b53535009181c

    ... after hex decoding, and when XOR'd against:
    686974207468652062756c6c277320657965

    ... should produce:
    746865206b696420646f6e277420706c6179
*/

use crate::utils::hex;
use std::iter::zip;

fn xor(a: &str, b: &str) -> String {
    let mut result = String::new();

    // xor each byte
    for bytes in zip(a.bytes(), b.bytes()) {
        let (bytea, byteb) = bytes;
        let bytex = hex::decode(bytea) ^ hex::decode(byteb);
        result.push(hex::encode(bytex));
    }

    return result;
}

pub fn test() {
    let a: &str = "1c0111001f010100061a024b53535009181c";
    let b: &str = "686974207468652062756c6c277320657965";
    let expected: &str = "746865206b696420646f6e277420706c6179";

    let result = xor(a, b);
    assert_eq!(result, expected);

    println!("\nChallenge 2: Fixed XOR");
    println!("exp: {}", expected);
    println!("res: {}", result);
}
