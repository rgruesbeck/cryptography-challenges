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
*/

use std::iter::zip;
use indexmap::IndexMap;

// https://docs.rs/indexmap/1.6.0/indexmap/map/struct.IndexMap.html#examples
fn get_frequency(string: &str) -> IndexMap<char, u32> {
    let mut frequency = IndexMap::new();
    // build frequency map
    for ch in string.chars() {
        *frequency.entry(ch).or_insert(0) += 1;
    }

    // sort frequency map
    frequency.sort_by(|ka, va, kb, vb| {
        vb.cmp(va)
    });

    return frequency;
}

fn score(freq: IndexMap<char, u32>) -> u32 {
    let english = " etaoinsh";
    let mut score = 0;
    for letters in zip(english.chars(), freq.clone().into_keys()) {
        let (a, b) = letters;
        score += u32::from(a) ^ u32::from(b);
    }
    score
}

fn check(string: &str) -> String {
    let frequency = get_frequency(&string);
    let res = String::new();

    for ch in string.bytes() {
    }

    println!("freq: {:?}", frequency);
    println!("score: {:?}", score(frequency));
    return res;
}


pub fn test() {
    let a: &str = "all work and no play makes jack a dull boy";
    let s: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let score = "a";

    println!("\nChallenge 3: Single-byte XOR cipher");
    println!("try: {}", check(s));
    //println!("exp: {}", expected);
    // println!("res: {}", result);
}
