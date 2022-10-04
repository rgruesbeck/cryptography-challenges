/*
    https://cryptopals.com/sets/1/challenges/1

    convert hex to base64
    *Always operate on raw bytes, never on encoded strings.
    *Only use hex and base64 for pretty-printing. 
*/

/*
    btoa
    https://en.wikipedia.org/wiki/Base64
 */
fn btoa(s: &str) -> String {
    let char_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    // process 3 byte buffer
    fn process_buffer(buf: [u8; 3], res: &mut String, set: &str) {

        // split bits: 3 octets -> 4 sextets
        let i0 = u8::from(buf[0] & 0b1111_1100) >> 2;
        let i1 = (u8::from(buf[0] & 0b0000_0011) << 4) + (u8::from(buf[1] & 0b1111_0000) >> 4);
        let i2 = (u8::from(buf[1] & 0b0000_1111) << 2) + (u8::from(buf[2] & 0b1100_0000) >> 6);
        let i3 = u8::from(buf[2] & 0b0011_1111);

        // iterate indexes and push chars to result
        for i in [i0, i1, i2, i3] {
            match set.chars().nth(i.into()) {
                Some(_c) => {
                    res.push(_c);
                }
                None => ()
            }
        }
    }

    let mut i = 0;
    let mut buffer = [0u8; 3];
    for byte in s.as_bytes() {
        // push byte to buffer and increment counter
        buffer[i] = *byte;
        i += 1;

        // if buffer is full OR at the last byte:
        // process and clear buffer, reset counter
        if i == 3 {
            process_buffer(buffer, &mut result, char_set);
            buffer = [0; 3];
            i = 0;
        }
    }

    result
}


/*
    hex char set: "0123456789abcdef"
    b64 char set: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
 */
fn hex_to_base64(s: &str) -> String {
    let char_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    // convert hex char into byte containing decimal value of the hex char
    fn decode_hex_byte(n: u8) -> u8 {
        let a = u8::from(n & 0b_0_111_0000) >> 4;
        let b = u8::from(n & 0b_0_000_1111);
        let c = match a {
            3 => b,
            6 => b + 9,
            _ => 0
        };
        c
    }

    fn process_buffer(buf: [u8; 3], res: &mut String, set: &str) {
        let i0 = (u8::from(buf[0] & 0b_0000_1111) << 2) + (u8::from(buf[1] & 0b_0000_1100) >> 2);
        let i1 = (u8::from(buf[1] & 0b_0000_0011) << 4) + u8::from(buf[2] & 0b_0000_1111);

        for i in [i0, i1] {
            match set.chars().nth(i.into()) {
                Some(_c) => {
                    res.push(_c);
                }
                None => ()
            }
        }
    }

    let mut i = 0;
    let mut buffer = [0u8; 3];
    for byte in s.bytes() {
        // add decimal byte to buffer
        let _byte = decode_hex_byte(byte);
        buffer[i] = _byte;
        i += 1;

        if i == 3 {
            process_buffer(buffer, &mut result, char_set);
            buffer = [0; 3];
            i = 0;
        }
    }

    result
}

pub fn test() {
    let input_string: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let result = hex_to_base64(input_string);
    assert_eq!(result, expected);

    println!("\nChallenge 1: Hex to Base64");
    println!("exp: {}", expected);
    println!("res: {}", result);

    let result_a = btoa("hello world");
    println!("\nextra");
    println!("btoa: hello world -> {}", result_a);
}
