/*
    https://cryptopals.com/sets/1/challenges/6

    plain
*/

use crate::utils::xor;

fn hamming_distance(a: &str, b: &str) -> u32 {
    let xor = xor::str(a, b);

    // bit count helper
    fn bit_count(byte: u8) -> u32 {
        let mut count = 0;
        let mut mask = 1;

        while mask <= byte {
            let bit_set = (byte & mask) == mask;
            if bit_set {
                count += 1;
            }
            mask = mask << 1;
        }

        println!("byte: {:b}, count: {}", byte, count);
        count
    }

    // count number of 1 bits
    let mut count = 0;
    for byte in xor {
        count += bit_count(byte);
    }

    count
}

pub fn test() {
    // assert_eq!(result, expected);

    println!("\nChallenge 6: ");

    let a = "this is a test";
    let b = "wokka wokka!!!";
    let res = hamming_distance(a, b);
    println!("hamming distance: ({}, {}) => {}", a, b, res);
    // println!("res: {}", result);
}
