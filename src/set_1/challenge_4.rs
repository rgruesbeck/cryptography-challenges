/*
    https://cryptopals.com/sets/1/challenges/4

    Detect single-character XOR

    One of the 60-character strings in this file has been encrypted by single-character XOR.
    Find it.

    (Your code from #3 should help.)


    https://en.wikipedia.org/wiki/ASCII
    https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/open.html
*/

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::utils::hex;

fn detect_xor() {
    fn decode_string(st: String, cypher: char) {
        let mut res = String::new();

        let mut contains_non_ascii_char = false;
        let mut contains_space = false;

        let mut i = 0;
        let mut buffer = [0u8; 2];
        for hex_byte in st.as_bytes() {
            let half_byte = hex::decode_byte(*hex_byte);
            buffer[i] = half_byte;
            i += 1;

            if i == 2 {
                let full_byte = u8::from(buffer[0] << 4) + u8::from(buffer[1]);
                // let xor_byte = full_byte ^ '5' as u8;
                let xor_byte = full_byte ^ cypher as u8;

                // ignore non ascii characters
                if xor_byte > 126 || xor_byte < 10 {
                    contains_non_ascii_char = true;
                    break;
                }

                // check for space char
                if xor_byte == 32 {
                    contains_space = true;
                }

                res.push(xor_byte as char);

                i = 0;
            }

        }

        let potential_match = !contains_non_ascii_char && contains_space;
        if potential_match {
            println!("\n{} -> {}", st, res);
        }

    }

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("src/set_1/4.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ls) = line {
                decode_string(ls, '5');
            }
        }
    }


}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn test() {
    println!("\nChallenge 4: Detect single-character XOR");
    detect_xor();
    // println!("exp: {}", expected);
    // println!("res: {}", result);
}
