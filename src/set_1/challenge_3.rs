/*
    https://cryptopals.com/sets/1/challenges/3

    Single-byte XOR cipher
    The hex encoded string:
    1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
    ... has been XOR'd against a single character. Find the key, decrypt the message.
    You can do this by hand. But don't: write code to do it for you.
    How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.

    Achievement Unlocked
    You now have our permission to make "ETAOIN SHRDLU" jokes on Twitter.

    https://vimeo.com/226419795
*/

use std::collections::BTreeMap;

// https://en.wikipedia.org/wiki/Hexadecimal#Base16_(transfer_encoding)
fn decode_hex_char(b: u8) -> u8 {
    match b & 0b0111_0000 {
        48 => b & 0b0000_1111,
        96 => (b & 0b0000_1111) + 9,
        _ => 0,
    }
}

/*
    https://en.wikipedia.org/wiki/Frequency_analysis
    https://en.wikipedia.org/wiki/ASCII

    x - 111 1000
   ' '- 010 0000
    X - 101 1000

    7 - 011 0111
    o - 110 1111
    X - 101 1000

*/
fn xor_cypher(st: &str) -> String {
    let key_guess = 'X';
    let mut result = String::new();

    // https://www.programming-idioms.org/idiom/8/initialize-a-new-map-associative-array/419/rust
    let mut freq_map: BTreeMap<char, u8> = BTreeMap::new();

    let mut i = 0;
    let mut buffer = [0u8; 2];
    for byte in st.as_bytes() {
        let c = decode_hex_char(*byte);

        buffer[i] = c;
        i += 1;

        if i == 2 {
            let _chr = u8::from(buffer[0] << 4) + u8::from(buffer[1]);
            let chr = _chr as char;

            // add char to frequency map
            match freq_map.get(&chr) {
                Some(count) => freq_map.insert(chr, count + 1),
                None => freq_map.insert(chr, 1),
            };

            let chrr = (_chr ^ (key_guess as u8)) as char;
            //let chrr = _chr as char;

            // add char to result string
            result.push(chrr);

            // reset index
            i = 0;
        }
    }

    let mut freq = Vec::from_iter(freq_map);
    freq.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    println!("frequency: {:?}", freq);
    result
}

pub fn test() {
    println!("\nChallenge 3: Single-byte XOR cipher");

    let a = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let res = xor_cypher(a);
    println!("res: {}", res);
}
