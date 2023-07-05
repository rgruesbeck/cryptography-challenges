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

fn score(s: &str) -> u8 {
    0
}

pub fn test() {
    let s: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let score = "a";

    println!("\nChallenge 3: Single-byte XOR cipher");
    //println!("exp: {}", expected);
    // println!("res: {}", result);
}
