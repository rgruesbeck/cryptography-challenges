/*
Single-byte XOR cipher

The hex encoded string:

1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736

... has been XOR'd against a single character. Find the key, decrypt the message.

You can do this by hand. But don't: write code to do it for you.

How?
Devise some method for "scoring" a piece of English plaintext.
Character frequency is a good metric.
Evaluate each output and choose the one with the best score. 

If a character is XOR'd on itself becomes 0;
For english sentences leter frequency should be ETAOIN SHRDLU.

// get frequency count:
// compare frequency count to ETAOINSHRDLU using a distance function

// simple distance function:
// XOR each byte and sum the resulting bytes
// the lower the sum, more similar they are

https://mathworld.wolfram.com/XOR.html
https://bitwisecmd.com/

// let list = get_frequency(string).keys().collect::<String>();
*/
use std::iter::zip;
use indexmap::IndexMap;
use crate::utils::hex;

// decode string with single byte xor 
fn decode(string: &str, key: u8) -> String {
    let mut res = String::new();
    let mut buffer = [0u8; 2];

    let mut i = 0;
    for byte in string.bytes() {
        let hexbyte = hex::decode(byte);
        buffer[i] = hexbyte;
        i += 1;

        if i == 2 {
            let ch = u8::from(buffer[0] << 4) + u8::from(buffer[1]);
            let xch = ch ^ key;
            res.push(xch as char);
            i = 0;
        }

        // let xbyte = hex::decode(byte) ^ key;
        // res.push(xbyte as char);
    }

    return res;
}

// https://docs.rs/indexmap/1.6.0/indexmap/map/struct.IndexMap.html#examples
fn get_frequency(string: &str) -> IndexMap<char, u32> {
    let mut frequency = IndexMap::new();
    // build frequency map
    for ch in string.chars() {
        if ch == ' ' || ch.is_alphabetic() {
            *frequency.entry(ch.to_ascii_lowercase()).or_insert(0) += 1;
        }
    }

    // sort frequency map
    frequency.sort_by(|_ka, va, kb, vb| {
        vb.cmp(va)
    });

    // println!("{:?}", frequency);
    return frequency;
}

fn score(freq: IndexMap<char, u32>, freq_list: &str) -> u32 {
    if freq.is_empty() {
        return u32::MAX;
    }

    let mut score = 0;
    let mut count = 0;
    for letters in zip(freq_list.chars(), freq.clone().into_keys()) {
        let (a, b) = letters;
        score += u32::from(a) ^ u32::from(b);
        count += 1;
    }
    score / count
}

/*
    crack
    - iterate through all byte values
        -> score the the decoded message
        -> updated lowest score

    decode message with lowest scored guess
*/
fn crack(string: &str, freq_list: &str) -> String {
    let res = String::new();
    let mut min = u32::MAX;
    let mut guess = 0;

    for key in 0..=255 {
        let msg = decode(string, key);
        let scr = score(get_frequency(&msg), freq_list);
        if scr < min {
            min = scr;
            guess = key;
        }
    }

    println!("frequency - key: '{}', score: {}", guess as char, min);
    decode(string, guess)
}


pub fn test() {
    let a: &str = "all work and no play makes jack a dull boy";
    let s: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let score = "a";
    let english = " etaoinsh";

    println!("\nChallenge 3: Single-byte XOR cipher");
    println!("crack: {}", crack(s, english));
}
