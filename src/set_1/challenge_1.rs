/*
    https://cryptopals.com/sets/1/challenges/1

    convert hex to base64
    *Always operate on raw bytes, never on encoded strings.
    *Only use hex and base64 for pretty-printing.
*/


/*
   hex
   hex char set: "0123456789abcdef"
   4 bits per char

   b64
   b64 char set: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"

*/
fn hex_to_base64(s: &str) -> String {
    let char_set_64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    fn to_hex(byte: u8) -> u8 {
        return match byte & 0b1111_0000 {
            0b0110_0000 => (byte & 0b0000_1111) + 9,
            _ => byte & 0b0000_1111
        }
    }

    fn process_buffer(mut result: &mut String, set: &str, buf: [u8; 3], idx: usize) {
        match idx {
            1 => {
                let i = ((buf[0] & 0b0000_1111) << 2) + ((buf[1] & 0b0000_1100) >> 2);
                match set.chars().nth(i.into()) {
                    Some(c) => {
                        result.push(c);
                    },
                    None => ()
                }
            }
            2 => {
                let i = ((buf[1] & 0b0000_0011) << 4) + (buf[2] & 0b0000_1111);
                match set.chars().nth(i.into()) {
                    Some(c) => {
                        result.push(c);
                    },
                    None => ()
                }
            }
            _ => {},
        }
    }

    // iterate over bytes converting hex encoded bytes to base64
    let mut i: usize = 0;
    let mut buffer: [u8; 3] = [0; 3];
    for byte in s.bytes() {
        buffer[i] = to_hex(byte);
        process_buffer(&mut result, char_set_64, buffer, i);

        i += 1;
        if i > 2 { i = 0 }
    }

    return result;
}

pub fn test() {
    let input_string: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let result = hex_to_base64(input_string);
    assert_eq!(result, expected);

    println!("\nChallenge 1: Hex to Base64");
    println!("exp: {}", expected);
    println!("res: {}", result);
}
